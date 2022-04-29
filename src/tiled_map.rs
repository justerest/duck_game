use macroquad::prelude::Rect;
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
        for layer in self.raw_tiled_map.layers.iter() {
            self.source.draw_tiles(
                &layer.name,
                Rect::new(0.0, 0.0, self.width() as _, self.height() as _),
                None,
            );
        }
    }

    pub fn width(&self) -> u32 {
        self.raw_tiled_map.width * self.raw_tiled_map.tilewidth
    }

    pub fn height(&self) -> u32 {
        self.raw_tiled_map.height * self.raw_tiled_map.tileheight
    }
}
