use macroquad::prelude::*;
use macroquad_platformer::{Actor, World};

// [speed] = px/s
// [acceleration] = px/s^2
const JUMP_SPEED: f32 = -800.;
const HOVER_SPEED: f32 = 160.;
const GRAVITY_ACCELERATION: f32 = 1960.;
const MAX_GRAVITY_SPEED: f32 = 960.;
const MAX_MOVE_SPEED: f32 = 320.;
const MOVE_ACCELERATION: f32 = 1280.;
const MOVE_DECELERATION: f32 = 1280.;

#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
}

pub struct Duck {
    texture: Texture2D,
    collider: Actor,
    speed: Vec2,
    dir: Direction,
}

impl Duck {
    pub fn create(texture: Texture2D, world: &mut World, init_pos: Vec2) -> Self {
        Self {
            texture,
            collider: world.add_actor(init_pos, texture.width() as i32, texture.height() as i32),
            speed: Vec2::ZERO,
            dir: Direction::Right,
        }
    }

    pub fn draw(&self, world: &World) {
        let pos = self.pos(world);
        draw_texture_ex(
            self.texture,
            pos.x,
            pos.y,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(
                    0.0,
                    0.0,
                    self.texture.width() as _,
                    self.texture.height() as _,
                )),
                flip_x: self.dir == Direction::Left,
                ..Default::default()
            },
        );
    }

    pub fn pos(&self, world: &World) -> Vec2 {
        world.actor_pos(self.collider)
    }

    pub fn center(&self, world: &World) -> Vec2 {
        self.pos(world) + vec2(self.texture.width() / 2., 0.)
    }

    pub fn update(&mut self, world: &mut World) {
        DuckUpdateAction::new(self, world).apply();
    }
}

struct DuckUpdateAction<'a> {
    duck: &'a mut Duck,
    world: &'a mut World,
}

impl<'a> DuckUpdateAction<'a> {
    fn new(duck: &'a mut Duck, world: &'a mut World) -> Self {
        Self { duck, world }
    }

    pub fn apply(mut self) {
        self.handle_gravity();
        self.handle_key_events();
        self.update_move();
    }

    fn handle_gravity(&mut self) {
        if self.is_on_ground() {
            self.duck.speed.y = 0.;
        } else if self.is_top_at_barrier() {
            self.duck.speed.y = HOVER_SPEED;
        } else {
            self.duck.speed.y = (self.duck.speed.y + GRAVITY_ACCELERATION * get_frame_time())
                .min(MAX_GRAVITY_SPEED);
        }
    }

    fn is_on_ground(&self) -> bool {
        let actor_pos = self.world.actor_pos(self.duck.collider);
        self.world
            .collide_check(self.duck.collider, actor_pos + vec2(0., 1.))
    }

    fn is_top_at_barrier(&self) -> bool {
        let actor_pos = self.world.actor_pos(self.duck.collider);
        self.world
            .collide_check(self.duck.collider, actor_pos + vec2(0., -1.))
    }

    fn handle_key_events(&mut self) {
        let speed_x = self.duck.speed.x;
        if is_key_down(KeyCode::Right) {
            self.duck.speed.x =
                (speed_x + MOVE_ACCELERATION * get_frame_time()).min(MAX_MOVE_SPEED);
            self.duck.dir = Direction::Right;
        } else if is_key_down(KeyCode::Left) {
            self.duck.speed.x =
                (speed_x - MOVE_ACCELERATION * get_frame_time()).max(-MAX_MOVE_SPEED);
            self.duck.dir = Direction::Left;
        } else {
            self.duck.speed.x =
                speed_x.signum() * (speed_x.abs() - MOVE_DECELERATION * get_frame_time()).max(0.);
        }

        if is_key_pressed(KeyCode::Space) && self.is_on_ground() {
            self.duck.speed.y = JUMP_SPEED;
        }

        if is_key_down(KeyCode::Space) && !self.is_on_ground() {
            self.duck.speed.y = self.duck.speed.y.min(HOVER_SPEED);
        }
    }

    fn update_move(&mut self) {
        let dx = self.duck.speed.x * get_frame_time();
        let dy = self.duck.speed.y * get_frame_time();
        self.world.move_h(self.duck.collider, dx);
        self.world.move_v(self.duck.collider, dy);
    }
}
