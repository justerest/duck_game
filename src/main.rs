mod assets_server;
mod camera;
mod duck;
mod duck_world;
mod input_service;
mod physics;
mod tiled_map;

use async_trait::async_trait;
use macroquad::prelude::*;
use macroquad_platformer::*;

use crate::assets_server::AssetsServer;
use crate::camera::Camera;
use crate::duck::Duck;
use crate::duck_world::DuckWorld;
use crate::input_service::InputService;
use crate::tiled_map::TiledMap;

const VIEWPORT_HEIGHT: f32 = 720.0;

mod tile_layers {
    pub const BORDERS: &str = "Tile Layer 2";
    pub const BARRIERS: &str = "Tile Layer 3";
}

#[macroquad::main("Уточка")]
async fn main() {
    let input_service = InputService::default();

    let mut game: Box<dyn Stage> = Box::new(Game::default());

    game.load().await;

    loop {
        if input_service.is_key_pressed(KeyCode::Escape) {
            return;
        }

        game.tick();

        next_frame().await;
    }
}

async fn load_map(assets_server: &AssetsServer) -> TiledMap {
    let tiled_map_json = assets_server.load_string("map.json").await.unwrap();

    let tileset_json = assets_server
        .load_string("tmw_desert_spacing.json")
        .await
        .unwrap();

    let tileset_png = assets_server
        .load_texture("tmw_desert_spacing.png")
        .await
        .unwrap();

    let macroquad_map = macroquad_tiled::load_map(
        &tiled_map_json,
        &[("tmw_desert_spacing.png", tileset_png)],
        &[("tmw_desert_spacing.json", &tileset_json)],
    )
    .unwrap();

    TiledMap::new(macroquad_map)
}

async fn load_duck_texture(assets_server: &AssetsServer) -> Texture2D {
    assets_server.load_texture("duck.png").await.unwrap()
}

#[async_trait]
trait Stage {
    async fn load(&mut self);
    fn tick(&mut self);
    fn destroy(self);
}

enum Game {
    Unloaded,
    Loaded {
        world: Box<DuckWorld>,
        duck: Duck,
        camera: Camera,
    },
}

impl Default for Game {
    fn default() -> Self {
        Self::Unloaded
    }
}

#[async_trait]
impl Stage for Game {
    async fn load(&mut self) {
        let assets_server = AssetsServer::new("assets");

        let tiled_map = load_map(&assets_server).await;
        let map_size = tiled_map.size();
        let aspect_ratio = screen_width() / screen_height();
        let viewport_size = vec2(aspect_ratio * VIEWPORT_HEIGHT, VIEWPORT_HEIGHT);

        let mut world = DuckWorld::new(tiled_map);
        let duck_texture = load_duck_texture(&assets_server).await;

        world.add_static_colliders(tile_layers::BORDERS, Tile::Solid);
        world.add_static_colliders(tile_layers::BARRIERS, Tile::JumpThrough);

        let duck = Duck::create(duck_texture, &mut world, vec2(50_f32, map_size.y - 150_f32));
        let camera = Camera::new(map_size, viewport_size);

        *self = Self::Loaded {
            world: Box::new(world),
            duck,
            camera,
        };
    }

    fn tick(&mut self) {
        if let Self::Loaded {
            world,
            duck,
            camera,
        } = self
        {
            duck.update(world);
            camera.update(duck.center(world));

            world.draw(camera.viewport());
            duck.draw(world);

            camera.focus();
        } else {
            panic!("Can't call tick before loading")
        }
    }

    fn destroy(self) {}
}
