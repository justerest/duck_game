use std::time::Duration;

use macroquad::prelude::*;
use macroquad_platformer::{Actor, Tile, World};

use crate::physics::*;

pub const MAX_JUMP_HEIGHT: Length = Length::from_meters(1.6);
pub const HOVER_VELOCITY: Velocity = Velocity::from_meters_on_second(1.6);
pub const GRAVITY_ACCELERATION: Acceleration =
    Acceleration::from_meters_on_second_on_second(2.0 * EARTH_G.as_meters_on_second_on_second());
pub const ADDITIONAL_GRAVITY_ACCELERATION: Acceleration = GRAVITY_ACCELERATION;
pub const MAX_FALL_VELOCITY: Velocity = Velocity::from_meters_on_second(10.0);
pub const MAX_MOVE_VELOCITY: Velocity = Velocity::from_meters_on_second(3.2);
pub const MOVE_ACCELERATION: Acceleration = Acceleration::from_meters_on_second_on_second(12.0);
pub const MOVE_DECELERATION: Acceleration = Acceleration::from_meters_on_second_on_second(6.0);

#[derive(PartialEq)]
enum HorizontalDirection {
    Left,
    Right,
}

pub struct Duck {
    texture: Texture2D,
    actor: Actor,
    velocity: XY<Velocity>,
    direction_h: HorizontalDirection,
}

impl Duck {
    pub fn create(texture: Texture2D, world: &mut World, init_pos: Vec2) -> Self {
        Self {
            texture,
            actor: world.add_actor(init_pos, texture.width() as _, texture.height() as _),
            velocity: Default::default(),
            direction_h: HorizontalDirection::Right,
        }
    }

    pub fn draw(&self, world: &World) {
        let pos = self.pos(world);
        let width = self.texture.width();
        let height = self.texture.height();
        draw_texture_ex(
            self.texture,
            pos.x,
            pos.y,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(0.0, 0.0, width as _, height as _)),
                flip_x: self.direction_h == HorizontalDirection::Left,
                ..Default::default()
            },
        );
    }

    pub fn pos(&self, world: &World) -> Vec2 {
        world.actor_pos(self.actor)
    }

    pub fn center(&self, world: &World) -> Vec2 {
        self.pos(world) + vec2(self.texture.width() / 2.0, 0.0)
    }

    pub fn update(&mut self, world: &mut World) {
        DuckUpdateAction::new(self, world).apply();
    }
}

struct DuckUpdateAction<'a> {
    duck: &'a mut Duck,
    world: &'a mut World,
    frame_time: Duration,
    is_on_ground: bool,
}

impl<'a> DuckUpdateAction<'a> {
    fn new(duck: &'a mut Duck, world: &'a mut World) -> Self {
        let mut duck_update_action = Self {
            duck,
            world,
            frame_time: Duration::from_secs_f32(get_frame_time()),
            is_on_ground: Default::default(),
        };
        duck_update_action.init();
        duck_update_action
    }

    fn init(&mut self) {
        let pos = self.world.actor_pos(self.duck.actor) + vec2(0.0, 1.0);
        self.is_on_ground = self.world.collide_check(self.duck.actor, pos)
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
        } else if self.is_top_at_solid() {
            self.duck.velocity.y = -self.duck.velocity.y / 2.0;
        } else {
            let dv = GRAVITY_ACCELERATION * self.frame_time;
            self.duck.velocity.y = (self.duck.velocity.y + dv).min(MAX_FALL_VELOCITY);
        }
    }

    fn is_on_ground(&self) -> bool {
        self.is_on_ground && !self.is_moving_up()
    }

    fn is_moving_up(&self) -> bool {
        self.duck.velocity.y < Velocity::ZERO
    }

    fn is_top_at_solid(&self) -> bool {
        let pos = self.world.actor_pos(self.duck.actor) - vec2(0.0, 1.0);
        self.world.collide_check(self.duck.actor, pos) && self.is_solid_at(pos)
    }

    fn is_solid_at(&self, pos: Vec2) -> bool {
        let width = self.duck.texture.width();
        let height = self.duck.texture.height();
        self.world.collide_solids(pos, width as _, height as _) == Tile::Solid
    }

    fn handle_move(&mut self) {
        if is_key_down(KeyCode::Right) {
            let dv = MOVE_ACCELERATION * self.frame_time;
            self.duck.velocity.x = (self.duck.velocity.x + dv).min(MAX_MOVE_VELOCITY);
            self.duck.direction_h = HorizontalDirection::Right;
        } else if is_key_down(KeyCode::Left) {
            let dv = MOVE_ACCELERATION * self.frame_time;
            self.duck.velocity.x = (self.duck.velocity.x - dv).max(-MAX_MOVE_VELOCITY);
            self.duck.direction_h = HorizontalDirection::Left;
        } else {
            let dv = MOVE_DECELERATION * self.frame_time;
            self.duck.velocity.x = self.duck.velocity.x.signum()
                * (self.duck.velocity.x.abs() - dv).max(Velocity::ZERO);
        }
    }

    fn handle_jump(&mut self) {
        if self.is_descent() {
            self.world.descent(self.duck.actor);
            self.duck.velocity.y = 2.0 * GRAVITY_ACCELERATION * self.frame_time;
        } else if self.is_jump_start() {
            self.duck.velocity.y = -jump_velocity();
        } else if self.is_jump_end() {
            self.duck.velocity.y += ADDITIONAL_GRAVITY_ACCELERATION * self.frame_time;
        } else if self.is_hover() {
            self.duck.velocity.y = self.duck.velocity.y.min(HOVER_VELOCITY);
        }
    }

    fn is_descent(&self) -> bool {
        is_key_down(KeyCode::Down) && is_key_pressed(KeyCode::Space) && self.is_on_ground()
    }

    fn is_jump_start(&self) -> bool {
        is_key_pressed(KeyCode::Space) && self.is_on_ground()
    }

    fn is_jump_end(&self) -> bool {
        !is_key_down(KeyCode::Space) && self.duck.velocity.y < Velocity::ZERO
    }

    fn is_hover(&self) -> bool {
        is_key_down(KeyCode::Space) && self.duck.velocity.y > Velocity::ZERO
    }

    fn update_position(&mut self) {
        let dx = self.duck.velocity.x * self.frame_time;
        let dy = self.duck.velocity.y * self.frame_time;
        self.world.move_h(self.duck.actor, dx.as_cm());
        self.world.move_v(self.duck.actor, dy.as_cm());
    }
}

// mv^2/2 = mgh
// v = sqrt(2gh)
fn jump_velocity() -> Velocity {
    let g = GRAVITY_ACCELERATION.as_meters_on_second_on_second();
    let h = MAX_JUMP_HEIGHT.as_meters();
    Velocity::from_meters_on_second((2.0 * g * h).sqrt())
}
