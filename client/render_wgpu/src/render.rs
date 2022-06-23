use crate::{mesh::Mesh, texture::Texture};
use client_core::render::{pass, Frame, Render as ClientRender, SetPass, Size};

pub struct Render {}

impl ClientRender for Render {
    type Mesh = Mesh;
    type Texture = Texture;

    fn resize(&mut self, _: Size) {
        todo!()
    }

    fn start_frame(&mut self) -> Frame<Self> {
        todo!()
    }

    fn make_mesh(&mut self) -> Self::Mesh {
        todo!()
    }

    fn make_texture(&mut self) -> Self::Texture {
        todo!()
    }
}

impl SetPass<pass::Solid> for Render {
    fn set_pass(&mut self) {
        todo!()
    }
}

impl SetPass<pass::Color> for Render {
    fn set_pass(&mut self) {
        todo!()
    }
}
