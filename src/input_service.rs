use macroquad::prelude::*;

#[derive(Default)]
pub struct InputService;

impl InputService {
    pub fn is_key_pressed(&self, key_code: KeyCode) -> bool {
        is_key_pressed(key_code)
    }
}
