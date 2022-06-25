use crate::state::State;
use render::{ClientRender as Render, Mesh};

pub struct View {
    render: Render,
    meshes: Vec<Mesh>,
}

impl View {
    pub fn new(render: Render) -> Self {
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
        self.render.draw_frame(|frame| {
            for mesh in self.meshes.iter().copied() {
                frame.draw_mesh(mesh);
            }
        });
    }
}
