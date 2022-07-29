mod map;
mod pack;

pub use self::map::Map;

#[derive(Clone, Copy, Default)]
pub struct Rect {
    pub pos: (u16, u16),
    pub size: (u16, u16),
}
