use base::{kit::Key, shape::Shape as ShapeId};
use fxhash::FxHashMap as Map;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Tile {
    pub layout: Layout,
    pub blocks: Map<Key, Block>,
}

impl Tile {
    pub fn sprites<F>(&self, mut callback: F)
    where
        F: FnMut(&Key),
    {
        let mut block = |bl: &Block| {
            let mut sprite = |ptr: &_| {
                if let SpritePointer::Key(key) | SpritePointer::Sprite { name: key, .. } = ptr {
                    callback(key);
                }
            };

            match &bl.shape.sprites {
                Sprites::Single(ptr) => sprite(ptr),
                Sprites::Multiple(v) => v.iter().for_each(sprite),
            }
        };

        match &self.layout {
            Layout::D1(ptr) => {
                ptr.block().map(&mut block);
            }
            Layout::D2(v) => v.iter().for_each(|ptr| {
                ptr.block().map(&mut block);
            }),
            Layout::D3(v) => v.iter().flatten().for_each(|ptr| {
                ptr.block().map(&mut block);
            }),
        }

        self.blocks.values().for_each(&mut block)
    }
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

impl BlockPointer {
    fn block(&self) -> Option<&Block> {
        match self {
            Self::Block(block) => Some(block),
            _ => None,
        }
    }
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
