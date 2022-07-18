use base::{shape::Shape, side::Side};
use core_client::render::{MeshData, Vert};

pub fn shape_data(shape: Shape) -> &'static [Data] {
    const S0: Data = Data {
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
        side: Side::Up,
    };

    match shape {
        Shape::S0 => &[S0],
    }
}

pub struct Data {
    pub mesh: MeshData<'static>,
    pub side: Side,
}
