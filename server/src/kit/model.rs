mod tile;

use crate::kit::{model::tile::Tile, Resources};

#[derive(Default)]
pub struct Model {
    pub tiles: Resources<Tile>,
}
