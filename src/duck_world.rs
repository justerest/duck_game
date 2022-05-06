use macroquad::prelude::Rect;
use macroquad_platformer::{Tile, World};

use crate::tiled_map::TiledMap;

pub struct DuckWorld {
    source: World,
    map: TiledMap,
}

impl std::ops::Deref for DuckWorld {
    type Target = World;

    fn deref(&self) -> &Self::Target {
        &self.source
    }
}

impl std::ops::DerefMut for DuckWorld {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.source
    }
}

impl DuckWorld {
    pub fn new(map: TiledMap) -> Self {
        Self {
            source: World::new(),
            map,
        }
    }

    pub fn draw(&self, dest: Rect) {
        self.map.draw_all_layers(dest);
    }

    pub fn add_static_colliders(&mut self, layer: &str, collider_type: Tile) {
        let static_colliders = self
            .map
            .tiles(layer, None)
            .map(|(_, _, tile)| tile.as_ref().map_or(Tile::Empty, |_| collider_type))
            .collect::<Vec<_>>();

        let map = &self.map.raw_tiled_map;
        let tile_width = map.tilewidth;
        let tile_height = map.tileheight;
        let width = map.width;

        self.add_static_tiled_layer(
            static_colliders,
            tile_width as _,
            tile_height as _,
            width as _,
            1,
        );
    }
}
