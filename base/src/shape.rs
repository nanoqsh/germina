use crate::{
    geometry::{MeshData, Vert},
    side::Side,
};
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize, Copy, Clone)]
#[serde(try_from = "u8")]
pub enum Shape {
    S0 = 0,
}

impl Shape {
    pub fn from_id(id: u8) -> Result<Self, ShapeIdError> {
        let shape = match id {
            0 => Self::S0,
            _ => return Err(ShapeIdError(())),
        };

        Ok(shape)
    }

    pub fn data(self) -> &'static [Data] {
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

        match self {
            Self::S0 => &[S0],
        }
    }
}

impl TryFrom<u8> for Shape {
    type Error = ShapeIdError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_id(value)
    }
}

pub struct ShapeIdError(());

impl fmt::Display for ShapeIdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "wrong shape id")
    }
}

pub struct Data {
    pub mesh: MeshData<'static>,
    pub side: Side,
}
