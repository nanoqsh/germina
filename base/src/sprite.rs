mod pack;
mod spritemap;

pub use self::spritemap::SpriteMap;

#[derive(Clone, Copy, Default)]
pub struct Rect {
    pub pos: (u16, u16),
    pub size: (u16, u16),
}
