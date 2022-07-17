use base::{
    shape::Shape,
    side::{Side, Sides},
};
use core_client::render::{Face, MeshData, Vert};

pub fn shape_data(shape: Shape) -> &[Data] {
    match shape {
        Shape::S0 => &[Data {
            // TODO: Add faces
            mesh: MeshData {
                verts: &[
                    Vert {
                        pos: [0., 0.5, 0.],
                        tex: [0., 0.],
                    },
                    Vert {
                        pos: [1., 0.5, 0.],
                        tex: [1., 0.],
                    },
                    Vert {
                        pos: [1., 0.5, 1.],
                        tex: [1., 1.],
                    },
                    Vert {
                        pos: [0., 0.5, 1.],
                        tex: [0., 1.],
                    },
                ],
                faces: &[[0, 1, 2], [1, 3, 0]],
            },
            sides: Side::Up.into(),
        }],
    }
}

pub struct Data {
    pub mesh: MeshData<'static>,
    pub sides: Sides,
}
