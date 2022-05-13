use macroquad::prelude::{vec2, Rect, Vec2};
use macroquad_tiled::Map;

pub struct TiledMap {
    m_map: Map,
}

impl std::ops::Deref for TiledMap {
    type Target = Map;

    fn deref(&self) -> &Self::Target {
        &self.m_map
    }
}

impl TiledMap {
    pub fn new(source: Map) -> Self {
        Self { m_map: source }
    }

    pub fn draw_all_layers_at_viewport(&self, viewport: Rect) {
        let x_start = (viewport.left() / self.tile_width() - 1.0) as u32;
        let x_end = (viewport.right() / self.tile_width() + 1.0) as u32;

        let y_start = (viewport.top() / self.tile_height() - 1.0) as u32;
        let y_end = (viewport.bottom() / self.tile_height() + 1.0) as u32;

        for layer in self.m_map.raw_tiled_map.layers.iter() {
            for x in x_start..=x_end {
                for y in y_start..=y_end {
                    self.draw_tile(&layer.name, x, y);
                }
            }
        }
    }

    fn draw_tile(&self, layer: &str, x: u32, y: u32) {
        if let Some(tile) = self.m_map.get_tile(layer, x, y) {
            let pos = vec2(x as f32 * self.tile_width(), y as f32 * self.tile_height());
            self.m_map.spr(
                &tile.tileset,
                tile.id,
                Rect::new(pos.x, pos.y, self.tile_width(), self.tile_height()),
            );
        }
    }

    fn tile_width(&self) -> f32 {
        self.m_map.raw_tiled_map.tilewidth as f32
    }

    fn tile_height(&self) -> f32 {
        self.m_map.raw_tiled_map.tileheight as f32
    }

    pub fn size(&self) -> Vec2 {
        vec2(self.width(), self.height())
    }

    fn width(&self) -> f32 {
        (self.m_map.raw_tiled_map.width * self.m_map.raw_tiled_map.tilewidth) as f32
    }

    fn height(&self) -> f32 {
        (self.m_map.raw_tiled_map.height * self.m_map.raw_tiled_map.tileheight) as f32
    }
}
