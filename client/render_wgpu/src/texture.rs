use crate::render::Render;
use client_core::render::Texture as ClientTexture;

pub struct Texture {}

impl ClientTexture for Texture {
    type Render = Render;

    fn bind(&self, _: &mut Self::Render) {
        todo!()
    }
}
