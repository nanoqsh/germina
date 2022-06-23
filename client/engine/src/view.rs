use crate::State;
use client_core::render::{pass, Mesh, Passes, Render};

pub struct View<R, M> {
    render: R,
    meshes: Vec<M>,
}

impl<R, M> View<R, M> {
    pub fn new(render: R) -> Self {
        Self {
            render,
            meshes: vec![],
        }
    }

    pub fn render_state(&mut self, _: &State)
    where
        R: Render + Passes,
        M: Mesh<Render = R>,
    {
        let mut frame = self.render.start_frame();
        let mut solid_pass = frame.pass(pass::Solid);
        for mesh in &self.meshes {
            solid_pass.draw_mesh(mesh);
        }
    }
}
