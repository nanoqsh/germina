use std::num::NonZeroU32;

pub type Size = (NonZeroU32, NonZeroU32);

pub trait Render: Sized {
    type Mesh;
    type Texture;

    fn resize(&mut self, size: Size);
    fn start_frame(&mut self);
    fn end_frame(&mut self);
    fn make_mesh(&mut self) -> Self::Mesh;
    fn draw_mesh(&mut self, mesh: &Self::Mesh);
    fn make_texture(&mut self) -> Self::Texture;
    fn bind_texture(&mut self, texture: &Self::Texture);
}
