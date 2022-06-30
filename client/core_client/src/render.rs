use bytemuck::{Pod, Zeroable};
use std::num::NonZeroU32;

pub type Size = (NonZeroU32, NonZeroU32);

pub struct MeshData<'a> {
    pub verts: &'a [Vert],
    pub faces: &'a [Face],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vert {
    pub pos: [f32; 3],
    pub uv: [f32; 2],
}

impl AsBytes for [Vert] {
    fn as_bytes(&self) -> &[u8] {
        bytemuck::cast_slice(self)
    }
}

pub type Face = [u16; 3];

pub trait AsBytes {
    fn as_bytes(&self) -> &[u8];
}
