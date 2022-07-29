use {
    crate::{
        chunk::{DEPTH, HEIGHT, WIDTH},
        point::{BlockPoint, ChunkPoint},
    },
    std::fmt,
};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Point {
    bl: BlockPoint,
    ch: ChunkPoint,
}

impl Point {
    pub const fn new(bl: BlockPoint, ch: ChunkPoint) -> Self {
        Self { bl, ch }
    }

    pub fn from_absolute(x: i32, y: i32, z: i32) -> Option<Self> {
        let ch = ChunkPoint::new(
            x.div_euclid(WIDTH as i32).try_into().ok()?,
            y.div_euclid(HEIGHT as i32).try_into().ok()?,
            z.div_euclid(DEPTH as i32).try_into().ok()?,
        )?;

        let bl = BlockPoint::new(
            x.rem_euclid(WIDTH as i32) as u8,
            y.rem_euclid(HEIGHT as i32) as u8,
            z.rem_euclid(DEPTH as i32) as u8,
        )
        .expect("cast");

        Some(Self::new(bl, ch))
    }

    // TODO: Private
    pub const fn block_point(self) -> BlockPoint {
        self.bl
    }

    // TODO: Private
    pub const fn chunk_point(self) -> ChunkPoint {
        self.ch
    }

    pub fn absolute(self) -> (i32, i32, i32) {
        let (blx, bly, blz) = self.bl.into();
        let (chx, chy, chz) = self.ch.into();

        (
            i32::from(chx) * WIDTH as i32 + i32::from(blx),
            i32::from(chy) * HEIGHT as i32 + i32::from(bly),
            i32::from(chz) * DEPTH as i32 + i32::from(blz),
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
