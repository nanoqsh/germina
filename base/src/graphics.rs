use {
    bytemuck::{Pod, Zeroable},
    std::num::NonZeroU32,
};

pub type Size = (NonZeroU32, NonZeroU32);

pub struct MeshData<'a> {
    pub verts: &'a [Vert],
    pub faces: &'a [Face],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Vert {
    pub pos: [f32; 3],
    pub tex: [f32; 2],
}

impl AsBytes for [Vert] {
    fn as_bytes(&self) -> &[u8] {
        bytemuck::cast_slice(self)
    }
}

pub type Face = [u16; 3];

impl AsBytes for [Face] {
    fn as_bytes(&self) -> &[u8] {
        bytemuck::cast_slice(self)
    }
}

pub struct TextureData<'a> {
    pub bytes: &'a [u8],
    pub size: (u32, u32),
}

pub trait AsBytes {
    fn as_bytes(&self) -> &[u8];
}
