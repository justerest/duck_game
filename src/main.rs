use macroquad::prelude::*;
use macroquad_platformer::*;
use macroquad_tiled::Map;

mod tile_layers {
    pub const LAYER_1: &str = "Tile Layer 1";
    pub const LAYER_2: &str = "Tile Layer 2";
    pub const LAYER_3: &str = "Tile Layer 3";
}

mod duck {
    pub const WIDTH: f32 = 50.;
    pub const HEIGHT: f32 = 72.;
}

mod consts {
    pub const JUMP_SPEED: f32 = -800.0;
    pub const GRAVITY: f32 = 2000.0;
    pub const MOVE_SPEED: f32 = 300.0;
}

struct Player {
    collider: Actor,
    speed: Vec2,
}

#[macroquad::main("Уточка")]
async fn main() {
    let duck = load_texture("assets/duck.png").await.unwrap();
    let tiled_map = load_tiled_map().await;

    let mut world = World::new();

    let mut static_colliders = vec![];

    for (_, _, tile) in tiled_map.tiles(tile_layers::LAYER_2, None) {
        if tile.is_some() {
            static_colliders.push(Tile::Collider);
        } else {
            static_colliders.push(Tile::Empty);
        }
    }

    world.add_static_tiled_layer(
        static_colliders,
        tiled_map.raw_tiled_map.tilewidth as _,
        tiled_map.raw_tiled_map.tileheight as _,
        tiled_map.raw_tiled_map.width as _,
        1,
    );

    let mut static_colliders = vec![];

    for (_, _, tile) in tiled_map.tiles(tile_layers::LAYER_3, None) {
        if tile.is_some() {
            static_colliders.push(Tile::JumpThrough);
        } else {
            static_colliders.push(Tile::Empty);
        }
    }

    world.add_static_tiled_layer(
        static_colliders,
        tiled_map.raw_tiled_map.tilewidth as _,
        tiled_map.raw_tiled_map.tileheight as _,
        tiled_map.raw_tiled_map.width as _,
        1,
    );

    let mut player = Player {
        collider: world.add_actor(vec2(400.0, 100.0), duck::WIDTH as _, duck::HEIGHT as _),
        speed: vec2(0., 0.),
    };

    request_new_screen_size((tiled_map.width() / 2) as _, (tiled_map.height() / 2 + 10) as _);

    loop {
        clear_background(BLACK);

        if is_key_pressed(KeyCode::Escape) {
            return;
        }

        tiled_map.draw();

        let pos = world.actor_pos(player.collider);

        draw_texture_ex(
            duck,
            pos.x,
            pos.y,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(0.0, 0.0, duck::WIDTH, duck::HEIGHT)),
                ..Default::default()
            },
        );

        let pos = world.actor_pos(player.collider);
        let on_ground = world.collide_check(player.collider, pos + vec2(0., 1.));

        if !on_ground {
            player.speed.y += consts::GRAVITY * get_frame_time();
        }

        if is_key_down(KeyCode::Right) {
            player.speed.x = consts::MOVE_SPEED;
        } else if is_key_down(KeyCode::Left) {
            player.speed.x = -consts::MOVE_SPEED;
        } else {
            player.speed.x = 0.;
        }

        if is_key_pressed(KeyCode::Space) && on_ground {
            player.speed.y = consts::JUMP_SPEED;
        }

        world.move_h(player.collider, player.speed.x * get_frame_time());
        world.move_v(player.collider, player.speed.y * get_frame_time());

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

trait MapExt {
    fn draw(&self);
    fn draw_layer(&self, tile_name: &str);
    fn width(&self) -> u32;
    fn height(&self) -> u32;
}

impl MapExt for Map {
    fn draw(&self) {
        self.draw_layer(tile_layers::LAYER_1);
        self.draw_layer(tile_layers::LAYER_2);
        self.draw_layer(tile_layers::LAYER_3);
    }

    fn draw_layer(&self, tile_name: &str) {
        self.draw_tiles(
            tile_name,
            Rect::new(0.0, 0.0, self.width() as _, self.height() as _),
            None,
        );
    }

    fn width(&self) -> u32 {
        self.raw_tiled_map.width * self.raw_tiled_map.tilewidth
    }

    fn height(&self) -> u32 {
        self.raw_tiled_map.height * self.raw_tiled_map.tileheight
    }
}
