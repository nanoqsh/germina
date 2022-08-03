mod data;
pub mod layout;
pub(crate) mod point;

use {
    self::{
        data::Data,
        layout::{Layout, Straight},
        point::InnerPoint,
    },
    crate::point::BlockPoint,
    std::ops,
};

pub mod size {
    const fn boundary(size: u32) -> u32 {
        assert!(0 < size && size <= u8::MAX as u32);
        size
    }

    pub const WIDTH: u32 = boundary(16);
    pub const HEIGHT: u32 = boundary(32);
    pub const DEPTH: u32 = boundary(16);
}

pub struct ChunkData<T, L = Straight<{ size::WIDTH }, { size::HEIGHT }>> {
    data: Data<T, L, { ({ size::WIDTH } * { size::HEIGHT } * { size::DEPTH }) as usize }>,
}

impl<T, L> ChunkData<T, L> {
    pub fn new(val: T) -> Self
    where
        T: Copy,
        L: Layout,
    {
        Self {
            data: Data::new(val),
        }
    }

    fn get(&self, point: InnerPoint) -> &T
    where
        L: Layout,
    {
        let (x, y, z) = point.into();
        // SAFETY: The `InnerPoint` invariant is its coordinates
        // is always in bound of chunk.
        unsafe {
            self.data
                .get_unchecked((u32::from(x), u32::from(y), u32::from(z)))
        }
    }

    fn get_mut(&mut self, point: InnerPoint) -> &mut T
    where
        L: Layout,
    {
        let (x, y, z) = point.into();
        // SAFETY: The `InnerPoint` invariant is its coordinates
        // is always in bound of chunk.
        unsafe {
            self.data
                .get_unchecked_mut((u32::from(x), u32::from(y), u32::from(z)))
        }
    }
}

impl<T, L> ops::Index<BlockPoint> for ChunkData<T, L>
where
    L: Layout,
{
    type Output = T;

    fn index(&self, point: BlockPoint) -> &Self::Output {
        self.get(point.into_inner())
    }
}

impl<T, L> ops::IndexMut<BlockPoint> for ChunkData<T, L>
where
    L: Layout,
{
    fn index_mut(&mut self, point: BlockPoint) -> &mut Self::Output {
        self.get_mut(point.into_inner())
    }
}
