mod assets_server;
mod camera;
mod di;
mod duck;
mod duck_world;
mod input_service;
mod physics;
mod tiled_map;
mod time;

use macroquad::prelude::*;
use macroquad_platformer::*;

use crate::assets_server::AssetsServer;
use crate::camera::Camera;
use crate::di::DiContainer;
use crate::duck::Duck;
use crate::duck_world::DuckWorld;
use crate::input_service::InputService;
use crate::tiled_map::TiledMap;
use crate::time::Time;

const VIEWPORT_HEIGHT: f32 = 720.0;

mod tile_layers {
    pub const BORDERS: &str = "Tile Layer 2";
    pub const BARRIERS: &str = "Tile Layer 3";
}

async fn init_di() -> DiContainer {
    let mut di = DiContainer::default();

    di.insert(AssetsServer::new("assets"));
    di.insert(InputService::default());
    di.insert(Time::new());
    di.insert(load_map(&di.get().unwrap()).await);

    di
}

#[macroquad::main("Уточка")]
async fn main() {
    let di = init_di().await;

    let assets_server = di.get::<AssetsServer>().unwrap();
    let input_service = di.get::<InputService>().unwrap();

    let tiled_map = di.get::<TiledMap>().unwrap();
    let map_size = tiled_map.size();
    let aspect_ratio = screen_width() / screen_height();
    let viewport_size = vec2(aspect_ratio * VIEWPORT_HEIGHT, VIEWPORT_HEIGHT);

    let mut world = DuckWorld::new(di.get().unwrap());
    let duck_texture = load_duck_texture(&assets_server).await;
    let mut duck = Duck::create(duck_texture, &mut world, vec2(50_f32, map_size.y - 150_f32));
    let mut camera = Camera::new(map_size, viewport_size);

    world.add_static_colliders(tile_layers::BORDERS, Tile::Solid);
    world.add_static_colliders(tile_layers::BARRIERS, Tile::JumpThrough);

    loop {
        if input_service.is_key_pressed(KeyCode::Escape) {
            return;
        }

        duck.update(&mut world);
        camera.update(duck.center(&world));

        world.draw(camera.viewport());
        duck.draw(&world);
        camera.focus();

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
