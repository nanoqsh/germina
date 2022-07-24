use base::{kit::Key, shape::Shape as ShapeId};
use fxhash::FxHashMap as Map;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Tile {
    pub layout: Layout,
    pub blocks: Map<Key, Block>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Layout {
    D1(BlockPointer),
    D2(Vec<BlockPointer>),
    D3(Vec<Vec<BlockPointer>>),
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum BlockPointer {
    None,
    Key(Key),
    Block(Block),
}

#[derive(Deserialize)]
pub struct Block {
    pub shape: Shape,
}

#[derive(Deserialize)]
pub struct Shape {
    pub id: ShapeId,
    pub sprites: Sprites,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Sprites {
    Single(SpritePointer),
    Multiple(Vec<SpritePointer>),
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum SpritePointer {
    None,
    Key(Key),
    Sprite {
        name: Key,
        offset: (f32, f32),
        discard: bool,
    },
}
