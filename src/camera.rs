use std::collections::LinkedList;

use macroquad::camera::{set_camera, Camera2D};
use macroquad::prelude::{vec2, Rect, Vec2};

const FOLLOW_X_BUFFER_CAPACITY: usize = 50;
const FOLLOW_Y_BUFFER_CAPACITY: usize = 20;

struct FollowBuffer {
    list: LinkedList<f32>,
    capacity: usize,
}

impl FollowBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            list: LinkedList::new(),
            capacity,
        }
    }

    pub fn add(&mut self, val: f32) {
        self.list.push_back(val);
        if self.list.len() > self.capacity {
            self.list.pop_front();
        }
    }

    pub fn mean(&self) -> f32 {
        self.list.iter().sum::<f32>() / self.list.len() as f32
    }

    pub fn last(&self) -> Option<f32> {
        self.list.front().cloned()
    }
}

pub struct Camera {
    map_size: Vec2,
    viewport_size: Vec2,
    x_follow_buffer: FollowBuffer,
    y_follow_buffer: FollowBuffer,
    hero_pos: Vec2,
}

impl Camera {
    pub fn new(map_size: Vec2, viewport_size: Vec2) -> Self {
        Self {
            map_size,
            viewport_size,
            x_follow_buffer: FollowBuffer::new(FOLLOW_X_BUFFER_CAPACITY),
            y_follow_buffer: FollowBuffer::new(FOLLOW_Y_BUFFER_CAPACITY),
            hero_pos: Vec2::ZERO,
        }
    }

    pub fn add_hero_pos(&mut self, pos: Vec2) {
        self.hero_pos = pos;
        if self.x_follow_buffer.last() != Some(pos.x) {
            self.x_follow_buffer.add(pos.x)
        }
        self.y_follow_buffer.add(pos.y);
    }

    pub fn focus_on_hero(&self) {
        let x_offset = self.hero_pos.x - self.x_follow_buffer.mean();
        let y_offset = (self.y_follow_buffer.mean() - self.hero_pos.y) / 2.;
        let res = self.hero_pos + vec2(x_offset, y_offset);
        set_camera(&Camera2D::from_display_rect(self.bound(res)));
    }

    fn bound(&self, point: Vec2) -> Rect {
        let start = point - (self.viewport_size / 2.);
        let end = start + self.viewport_size;
        let res = start + (self.map_size - end).min(Vec2::ZERO) + (-start).max(Vec2::ZERO);
        Rect::new(res.x, res.y, self.viewport_size.x, self.viewport_size.y)
    }
}

#[cfg(test)]
mod tests {
    use macroquad::prelude::vec2;

    use super::*;

    #[test]
    fn should_be_in_bound_on_move_to_bottom_left() {
        let bound = vec2(100., 100.);
        let viewport = vec2(50., 50.);
        let camera = Camera::new(bound, viewport);
        assert_eq!(camera.bound(vec2(0., 100.)), Rect::new(0., 50., 50., 50.));
    }

    #[test]
    fn should_be_in_bound_on_move_to_bottom_right() {
        let camera = Camera::new(vec2(100., 100.), vec2(50., 50.));
        assert_eq!(
            camera.bound(vec2(100., 100.)),
            Rect::new(50., 50., 50., 50.)
        );
    }

    #[test]
    fn should_be_in_bound_on_move_to_right() {
        let camera = Camera::new(vec2(100., 100.), vec2(50., 50.));
        assert_eq!(camera.bound(vec2(100., 25.)), Rect::new(50., 0., 50., 50.));
    }

    #[test]
    fn should_be_in_bound_on_move_to_top_right() {
        let camera = Camera::new(vec2(100., 100.), vec2(50., 50.));
        assert_eq!(camera.bound(vec2(100., 0.)), Rect::new(50., 0., 50., 50.));
    }

    #[test]
    fn should_be_in_bound_on_move_to_top_left() {
        let camera = Camera::new(vec2(100., 100.), vec2(50., 50.));
        assert_eq!(camera.bound(vec2(0., 0.)), Rect::new(0., 0., 50., 50.));
    }
}
