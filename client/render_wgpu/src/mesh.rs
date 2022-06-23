use crate::render::Render;
use client_core::render::Mesh as ClientMesh;

pub struct Mesh {}

impl ClientMesh for Mesh {
    type Render = Render;

    fn draw(&self, _: &mut Self::Render) {
        todo!()
    }
}
