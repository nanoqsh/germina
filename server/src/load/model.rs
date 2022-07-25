mod tile;

use crate::load::model::tile::Tile;
use base::kit::Resources;

#[derive(Default)]
pub struct Model {
    pub tiles: Resources<Tile>,
    pub tile_sprites: Resources<Vec<u8>>,
}
