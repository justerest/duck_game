use macroquad::prelude::*;
use macroquad_platformer::*;
use macroquad_tiled::Map;

use crate::camera::Camera;
use crate::duck::Duck;
use crate::duck_world::DuckWorld;
use crate::tiled_map::TiledMap;

mod camera;
mod duck;
mod duck_world;
mod tiled_map;

mod tile_layers {
    pub const BORDERS: &str = "Tile Layer 2";
    pub const BARRIERS: &str = "Tile Layer 3";
}

const VIEWPORT_HEIGHT: f32 = 640.;

async fn load_tiled_map() -> Map {
    let tiled_map_json = load_string("assets/map.json").await.unwrap();
    let tileset_json = load_string("assets/tmw_desert_spacing.json").await.unwrap();
    let tileset_png = load_texture("assets/tmw_desert_spacing.png").await.unwrap();

    macroquad_tiled::load_map(
        &tiled_map_json,
        &[("tmw_desert_spacing.png", tileset_png)],
        &[("tmw_desert_spacing.json", &tileset_json)],
    )
    .unwrap()
}

async fn load_duck_texture() -> Texture2D {
    load_texture("assets/duck.png").await.unwrap()
}

#[macroquad::main("Уточка")]
async fn main() {
    let tiled_map = TiledMap::new(load_tiled_map().await);
    let duck_texture = load_duck_texture().await;

    let map_size = tiled_map.size();
    let aspect_ratio = screen_width() / screen_height();
    let viewport_size = vec2(aspect_ratio * VIEWPORT_HEIGHT, VIEWPORT_HEIGHT);

    let mut world = DuckWorld::new(tiled_map);
    let mut duck = Duck::create(duck_texture, &mut world, vec2(50., map_size.y - 150.));
    let mut camera = Camera::new(map_size, viewport_size);

    world.add_static_colliders(tile_layers::BORDERS, Tile::Solid);
    world.add_static_colliders(tile_layers::BARRIERS, Tile::JumpThrough);

    loop {
        if is_key_pressed(KeyCode::Escape) {
            return;
        }

        clear_background(BLACK);

        camera.add_hero_pos(duck.center(&world));
        camera.focus_on_hero();

        world.draw();
        duck.draw(&world);

        duck.update(&mut world);

        next_frame().await;
    }
}
