use macroquad::camera::{set_camera, Camera2D};
use macroquad::prelude::{vec2, Rect, Vec2};

use self::follow_buffer::FollowBuffer;

mod follow_buffer;

const FOLLOW_X_BUFFER_CAPACITY: usize = 75;
const FOLLOW_Y_BUFFER_CAPACITY: usize = 25;

pub struct Camera {
    map_size: Vec2,
    viewport_size: Vec2,
    viewport: Rect,
    x_follow_buffer: FollowBuffer,
    y_follow_buffer: FollowBuffer,
}

impl Camera {
    pub fn new(map_size: Vec2, viewport_size: Vec2) -> Self {
        Self {
            map_size,
            viewport_size,
            x_follow_buffer: FollowBuffer::new(FOLLOW_X_BUFFER_CAPACITY),
            y_follow_buffer: FollowBuffer::new(FOLLOW_Y_BUFFER_CAPACITY),
            viewport: Default::default(),
        }
    }

    pub fn viewport(&self) -> Rect {
        self.viewport
    }

    pub fn update(&mut self, focus_position: Vec2) {
        self.update_follow_buffers(focus_position);
        self.update_viewport(focus_position);
    }

    fn update_follow_buffers(&mut self, focus_position: Vec2) {
        if self.x_follow_buffer.last() != Some(focus_position.x) {
            self.x_follow_buffer.push(focus_position.x)
        }
        self.y_follow_buffer.push(focus_position.y);
    }

    fn update_viewport(&mut self, focus_position: Vec2) {
        let x_offset = focus_position.x - self.x_follow_buffer.mean();
        let y_offset = (self.y_follow_buffer.mean() - focus_position.y) / 2.;
        let viewport_center = focus_position + vec2(x_offset, y_offset);
        self.viewport = self.get_viewport_bounded_by_window_size(viewport_center);
    }

    fn get_viewport_bounded_by_window_size(&self, viewport_center: Vec2) -> Rect {
        let start = viewport_center - (self.viewport_size / 2.);
        let end = start + self.viewport_size;
        let res = start + (self.map_size - end).min(Vec2::ZERO) + (-start).max(Vec2::ZERO);
        Rect::new(res.x, res.y, self.viewport_size.x, self.viewport_size.y)
    }

    pub fn focus(&mut self) {
        set_camera(&Camera2D::from_display_rect(self.viewport));
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
        assert_eq!(
            camera.get_viewport_bounded_by_window_size(vec2(0., 100.)),
            Rect::new(0., 50., 50., 50.)
        );
    }

    #[test]
    fn should_be_in_bound_on_move_to_bottom_right() {
        let camera = Camera::new(vec2(100., 100.), vec2(50., 50.));
        assert_eq!(
            camera.get_viewport_bounded_by_window_size(vec2(100., 100.)),
            Rect::new(50., 50., 50., 50.)
        );
    }

    #[test]
    fn should_be_in_bound_on_move_to_right() {
        let camera = Camera::new(vec2(100., 100.), vec2(50., 50.));
        assert_eq!(
            camera.get_viewport_bounded_by_window_size(vec2(100., 25.)),
            Rect::new(50., 0., 50., 50.)
        );
    }

    #[test]
    fn should_be_in_bound_on_move_to_top_right() {
        let camera = Camera::new(vec2(100., 100.), vec2(50., 50.));
        assert_eq!(
            camera.get_viewport_bounded_by_window_size(vec2(100., 0.)),
            Rect::new(50., 0., 50., 50.)
        );
    }

    #[test]
    fn should_be_in_bound_on_move_to_top_left() {
        let camera = Camera::new(vec2(100., 100.), vec2(50., 50.));
        assert_eq!(
            camera.get_viewport_bounded_by_window_size(vec2(0., 0.)),
            Rect::new(0., 0., 50., 50.)
        );
    }
}
