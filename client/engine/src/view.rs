use crate::state::State;
use client_core::render::Render;

pub struct View<R>
where
    R: Render,
{
    render: R,
    meshes: Vec<R::Mesh>,
}

impl<R> View<R>
where
    R: Render,
{
    pub fn new(render: R) -> Self {
        Self {
            render,
            meshes: vec![],
        }
    }

    pub fn resize(&mut self, (width, height): (u32, u32)) {
        use std::num::NonZeroU32;

        self.render.resize((
            NonZeroU32::new(width).unwrap_or(NonZeroU32::new(1).expect("non zero")),
            NonZeroU32::new(height).unwrap_or(NonZeroU32::new(1).expect("non zero")),
        ));
    }

    pub fn render_state(&mut self, _: &State) {
        self.render.start_frame();
        for mesh in &self.meshes {
            self.render.draw_mesh(mesh);
        }
        self.render.end_frame();
    }
}
