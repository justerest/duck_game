use macroquad::prelude::{vec2, Rect, Vec2};
use macroquad_tiled::Map;

pub struct TiledMap {
    source: Map,
}

impl std::ops::Deref for TiledMap {
    type Target = Map;

    fn deref(&self) -> &Self::Target {
        &self.source
    }
}

impl TiledMap {
    pub fn new(source: Map) -> Self {
        Self { source }
    }

    pub fn draw_all_layers(&self) {
        let size = self.size();
        for layer in self.raw_tiled_map.layers.iter() {
            self.source
                .draw_tiles(&layer.name, Rect::new(0.0, 0.0, size.x, size.y), None);
        }
    }

    pub fn width(&self) -> f32 {
        (self.raw_tiled_map.width * self.raw_tiled_map.tilewidth) as f32
    }

    pub fn height(&self) -> f32 {
        (self.raw_tiled_map.height * self.raw_tiled_map.tileheight) as f32
    }

    pub fn size(&self) -> Vec2 {
        vec2(self.width(), self.height())
    }
}
