use macroquad::prelude::*;
use macroquad_platformer::*;
use macroquad_tiled::Map;

use crate::duck::Duck;
use crate::duck_world::DuckWorld;
use crate::tiled_map::TiledMap;

mod duck;
mod duck_world;
mod tiled_map;

mod tile_layers {
    pub const BORDERS: &str = "Tile Layer 2";
    pub const BARRIERS: &str = "Tile Layer 3";
}

#[macroquad::main("Уточка")]
async fn main() {
    let duck_texture = load_texture("assets/duck.png").await.unwrap();
    let tiled_map = TiledMap::new(load_tiled_map().await);

    let width = tiled_map.width();
    let height = tiled_map.height();

    let mut world = DuckWorld::new(tiled_map);
    let mut duck = Duck::new(duck_texture, &mut world);

    world.add_static_colliders(tile_layers::BORDERS, Tile::Solid);
    world.add_static_colliders(tile_layers::BARRIERS, Tile::JumpThrough);

    request_new_screen_size((width / 2) as _, (height / 2 + 13) as _);

    loop {
        clear_background(BLACK);

        if is_key_pressed(KeyCode::Escape) {
            return;
        }

        world.draw();
        duck.draw(&world);
        duck.update(&mut world);

        next_frame().await;
    }
}

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
