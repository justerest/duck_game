use macroquad::prelude::*;
use macroquad_platformer::{Actor, World};

const JUMP_SPEED: f32 = -800.0;
const GRAVITY: f32 = 2000.0;
const MOVE_SPEED: f32 = 300.0;

pub struct Duck {
    texture: Texture2D,
    collider: Actor,
    speed: Vec2,
    flip_x: bool,
}

impl std::ops::Deref for Duck {
    type Target = Actor;

    fn deref(&self) -> &Self::Target {
        &self.collider
    }
}

impl Duck {
    pub fn new(texture: Texture2D, world: &mut World) -> Self {
        Self {
            texture,
            collider: world.add_actor(
                vec2(50.0, 50.0),
                texture.width() as _,
                texture.height() as _,
            ),
            speed: vec2(0., 0.),
            flip_x: false,
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
                flip_x: self.flip_x,
                ..Default::default()
            },
        );
    }

    pub fn update(&mut self, world: &mut World) {
        let pos = self.pos(world);
        let on_ground = world.collide_check(self.collider, pos + vec2(0., 1.));

        if !on_ground {
            self.speed.y += GRAVITY * get_frame_time();
        }

        if is_key_down(KeyCode::Right) {
            self.speed.x = MOVE_SPEED;
            self.flip_x = false;
        } else if is_key_down(KeyCode::Left) {
            self.speed.x = -MOVE_SPEED;
            self.flip_x = true;
        } else {
            self.speed.x = 0.;
        }

        if is_key_down(KeyCode::Space) && on_ground {
            self.speed.y = JUMP_SPEED;
        } else if is_key_down(KeyCode::Up)
            && (is_key_down(KeyCode::Left) || is_key_down(KeyCode::Right))
        {
            self.speed.y = JUMP_SPEED / 50.;
        }

        world.move_h(self.collider, self.speed.x * get_frame_time());
        world.move_v(self.collider, self.speed.y * get_frame_time());
    }

    fn pos(&self, world: &World) -> Vec2 {
        world.actor_pos(self.collider)
    }
}
