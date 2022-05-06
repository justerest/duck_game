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

    pub fn draw_all_layers(&self, dest: Rect) {
        let spr_width = self.raw_tiled_map.tilewidth as f32;
        let spr_height = self.raw_tiled_map.tileheight as f32;
        let source = Rect::new(
            (dest.x / self.raw_tiled_map.tilewidth as f32).floor(),
            (dest.y / self.raw_tiled_map.tileheight as f32).floor(),
            (dest.w / self.raw_tiled_map.tilewidth as f32).ceil(),
            (dest.h / self.raw_tiled_map.tileheight as f32).ceil(),
        );

        for layer in self.raw_tiled_map.layers.iter() {
            let layer = &self.layers[&layer.name];
            for x in source.left() as u32..=source.right() as u32 {
                for y in source.top() as u32..=source.bottom() as u32 {
                    let spr_index = (y * layer.width + x) as usize;
                    if let Some(tile) = layer.data.get(spr_index).and_then(Option::as_ref) {
                        let pos = vec2(x as f32 * spr_width, y as f32 * spr_height);
                        self.spr(
                            &tile.tileset,
                            tile.id,
                            Rect::new(pos.x, pos.y, spr_width, spr_height),
                        );
                    }
                }
            }
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
