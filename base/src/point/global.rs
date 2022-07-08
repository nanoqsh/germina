use crate::{
    chunk::{DEPTH, HEIGHT, WIDTH},
    point::{ChunkPoint, ClusterPoint, Error},
};
use std::fmt;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct Point {
    ch: ChunkPoint,
    cl: ClusterPoint,
}

impl Point {
    pub const fn new(ch: ChunkPoint, cl: ClusterPoint) -> Self {
        Self { ch, cl }
    }

    pub fn from_absolute(x: i32, y: i32, z: i32) -> Result<Self, Error> {
        let cl = ClusterPoint::new(
            x.div_euclid(WIDTH as i32).try_into()?,
            y.div_euclid(HEIGHT as i32).try_into()?,
            z.div_euclid(DEPTH as i32).try_into()?,
        )?;

        let ch = ChunkPoint::new(
            x.rem_euclid(WIDTH as i32) as u8,
            y.rem_euclid(HEIGHT as i32) as u8,
            z.rem_euclid(DEPTH as i32) as u8,
        )
        .expect("cast");

        Ok(Self::new(ch, cl))
    }

    // TODO: Private
    pub const fn chunk_point(self) -> ChunkPoint {
        self.ch
    }

    // TODO: Private
    pub const fn cluster_point(self) -> ClusterPoint {
        self.cl
    }

    pub fn absolute(self) -> (i32, i32, i32) {
        let (chx, chy, chz) = self.ch.into();
        let (clx, cly, clz) = self.cl.into();

        (
            i32::from(clx) * WIDTH as i32 + i32::from(chx),
            i32::from(cly) * HEIGHT as i32 + i32::from(chy),
            i32::from(clz) * DEPTH as i32 + i32::from(chz),
        )
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (x, y, z) = self.absolute();
        write!(f, "[{x}, {y}, {z}]")
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const POINTS: [(i32, i32, i32); 6] = [
        (0, 0, 0),
        (1, -1, 0),
        (2, -2, 3),
        (15, 0, -15),
        (16, 0, -16),
        (-45, 50, 32),
    ];

    #[test]
    fn absolute() {
        for (x, y, z) in POINTS {
            let point = Point::from_absolute(x, y, z).unwrap();
            assert_eq!(point.absolute(), (x, y, z));
        }
    }
}
