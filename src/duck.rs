use std::time::Duration;

use macroquad::prelude::*;
use macroquad_platformer::{Actor, World};

use crate::physics::*;

pub const MAX_JUMP_HEIGHT: Length = Length::from_meters(1.6);
pub const HOVER_VELOCITY: Velocity = Velocity::from_meters_on_second(1.6);
pub const GRAVITY_ACCELERATION: Acceleration =
    Acceleration::from_meters_on_second_on_second(2.0 * EARTH_G.as_meters_on_second_on_second());
pub const ADDITIONAL_GRAVITY_ACCELERATION: Acceleration = EARTH_G;
pub const MAX_FALL_VELOCITY: Velocity = Velocity::from_meters_on_second(10.0);
pub const MAX_MOVE_VELOCITY: Velocity = Velocity::from_meters_on_second(3.2);
pub const MOVE_ACCELERATION: Acceleration = Acceleration::from_meters_on_second_on_second(12.0);
pub const MOVE_DECELERATION: Acceleration = Acceleration::from_meters_on_second_on_second(6.0);

#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
}

pub struct Duck {
    texture: Texture2D,
    collider: Actor,
    velocity: XY<Velocity>,
    dir: Direction,
}

impl Duck {
    pub fn create(texture: Texture2D, world: &mut World, init_pos: Vec2) -> Self {
        Self {
            texture,
            collider: world.add_actor(init_pos, texture.width() as i32, texture.height() as i32),
            velocity: Default::default(),
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
    frame_time: Duration,
}

impl<'a> DuckUpdateAction<'a> {
    fn new(duck: &'a mut Duck, world: &'a mut World) -> Self {
        Self {
            duck,
            world,
            frame_time: Duration::from_secs_f32(get_frame_time()),
        }
    }

    pub fn apply(mut self) {
        self.handle_gravity();
        self.handle_move();
        self.handle_jump();
        self.update_position();
    }

    fn handle_gravity(&mut self) {
        if self.is_on_ground() {
            self.duck.velocity.y = Velocity::ZERO;
        } else if self.is_top_at_barrier() {
            self.duck.velocity.y = -self.duck.velocity.y / 2.0;
        } else {
            let dv = GRAVITY_ACCELERATION * self.frame_time;
            self.duck.velocity.y = (self.duck.velocity.y + dv).min(MAX_FALL_VELOCITY);
        }
    }

    fn is_on_ground(&self) -> bool {
        let actor_pos = self.world.actor_pos(self.duck.collider) + vec2(0.0, 1.0);
        let is_move_up = self.duck.velocity.y < Velocity::ZERO;
        self.world.collide_check(self.duck.collider, actor_pos) && !is_move_up
    }

    fn is_top_at_barrier(&self) -> bool {
        let actor_pos = self.world.actor_pos(self.duck.collider) + vec2(0.0, -1.0);
        let is_move_down = self.duck.velocity.y > Velocity::ZERO;
        self.world.collide_check(self.duck.collider, actor_pos) && !is_move_down
    }

    fn handle_move(&mut self) {
        if is_key_down(KeyCode::Right) {
            let dv = MOVE_ACCELERATION * self.frame_time;
            self.duck.velocity.x = (self.duck.velocity.x + dv).min(MAX_MOVE_VELOCITY);
            self.duck.dir = Direction::Right;
        } else if is_key_down(KeyCode::Left) {
            let dv = MOVE_ACCELERATION * self.frame_time;
            self.duck.velocity.x = (self.duck.velocity.x - dv).max(-MAX_MOVE_VELOCITY);
            self.duck.dir = Direction::Left;
        } else {
            let dv = MOVE_DECELERATION * self.frame_time;
            self.duck.velocity.x = self.duck.velocity.x.signum()
                * (self.duck.velocity.x.abs() - dv).max(Velocity::ZERO);
        }
    }

    fn handle_jump(&mut self) {
        if self.is_jump_from_ground() {
            self.duck.velocity.y = -jump_velocity();
        }

        if self.is_jump_end() {
            self.duck.velocity.y += ADDITIONAL_GRAVITY_ACCELERATION * self.frame_time;
        }

        if self.is_hover() {
            self.duck.velocity.y = self.duck.velocity.y.min(HOVER_VELOCITY);
        }
    }

    fn is_jump_from_ground(&self) -> bool {
        is_key_pressed(KeyCode::Space) && self.is_on_ground()
    }

    fn is_jump_end(&self) -> bool {
        !is_key_down(KeyCode::Space) && self.is_jump()
    }

    fn is_jump(&self) -> bool {
        self.duck.velocity.y < Velocity::ZERO
    }

    fn is_hover(&self) -> bool {
        is_key_down(KeyCode::Space) && !self.is_on_ground()
    }

    fn update_position(&mut self) {
        let dx = self.duck.velocity.x * self.frame_time;
        let dy = self.duck.velocity.y * self.frame_time;
        self.world.move_h(self.duck.collider, dx.as_cm());
        self.world.move_v(self.duck.collider, dy.as_cm());
    }
}

// mv^2/2 = mgh
// v = sqrt(2gh)
fn jump_velocity() -> Velocity {
    let g = GRAVITY_ACCELERATION.as_meters_on_second_on_second();
    let h = MAX_JUMP_HEIGHT.as_meters();
    Velocity::from_meters_on_second((2.0 * g * h).sqrt())
}
