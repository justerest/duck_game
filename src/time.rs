use std::time::Duration;

use macroquad::prelude::*;

#[derive(Default)]
pub struct Time {
    frame_delta: Duration,
}

impl Time {
    pub fn new() -> Self {
        Self {
            frame_delta: Default::default(),
        }
    }

    #[allow(unused)]
    pub fn frame_delta(&self) -> Duration {
        self.frame_delta
    }

    pub fn tick(&mut self) {
        self.frame_delta = Duration::from_secs_f32(get_frame_time());
    }
}
