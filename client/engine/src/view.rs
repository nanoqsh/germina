use crate::state::State;
use render::{ClientRender as Render, Mesh, Texture};

struct Model {
    mesh: Mesh,
    texture: Texture,
}

pub struct View {
    render: Render,
    models: Vec<Model>,
}

impl View {
    pub fn new(mut render: Render) -> Self {
        use core_client::render::{MeshData, TextureData, Vert};
        use image::GenericImageView;

        let mesh = render.make_mesh(MeshData {
            verts: &[
                Vert {
                    pos: [-0.5, -0.5, 0.],
                    tex: [0., 1.],
                },
                Vert {
                    pos: [-0.5, 0.5, 0.],
                    tex: [0., 0.],
                },
                Vert {
                    pos: [0.5, 0.5, 0.],
                    tex: [1., 0.],
                },
                Vert {
                    pos: [0.5, -0.5, 0.],
                    tex: [1., 1.],
                },
            ],
            faces: &[[0, 2, 1], [0, 3, 2]],
        });

        let raw_image = include_bytes!("../texture.png");
        let image = image::load_from_memory(raw_image).expect("load image");
        let texture = render.make_texture(TextureData {
            bytes: &image.to_rgba8(),
            size: image.dimensions(),
        });

        Self {
            render,
            models: vec![Model { mesh, texture }],
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
            for model in &self.models {
                frame.bind_texture(model.texture);
                frame.draw_mesh(model.mesh);
            }
        });
    }
}
