use {
    crate::{chunk, side::Side},
    std::fmt,
};

const WIDTH: u8 = chunk::WIDTH as u8;
const HEIGHT: u8 = chunk::HEIGHT as u8;
const DEPTH: u8 = chunk::DEPTH as u8;

/// A chunk point.
///
/// Internal implementation of a point.
/// It maintains an invariant that a point is always within the boundaries of a chunk.
/// Since the some `unsafe` code relies on this invariant, violation of it
/// results an undefined behavior.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct InnerPoint {
    x: u8,
    y: u8,
    z: u8,
}

impl InnerPoint {
    pub const fn new(x: u8, y: u8, z: u8) -> Option<Self> {
        if x < WIDTH && y < HEIGHT && z < DEPTH {
            Some(Self { x, y, z })
        } else {
            None
        }
    }

    const unsafe fn new_unchecked(x: u8, y: u8, z: u8) -> Self {
        debug_assert!(x < WIDTH);
        debug_assert!(y < HEIGHT);
        debug_assert!(z < DEPTH);
        Self { x, y, z }
    }

    pub fn to(self, side: Side, n: u8) -> Result<Self, Self> {
        let Self { x, y, z } = self;
        let points = match side {
            Side::Left => {
                assert!(n < WIDTH);
                let v = x + n;
                if v < WIDTH {
                    Ok((v, y, z))
                } else {
                    Err((WIDTH - v, y, z))
                }
            }
            Side::Right => {
                assert!(n < WIDTH);
                if n <= x {
                    Ok((x - n, y, z))
                } else {
                    Err((WIDTH - n + x, y, z))
                }
            }
            Side::Up => {
                assert!(n < HEIGHT);
                let v = y + n;
                if v < HEIGHT {
                    Ok((x, v, z))
                } else {
                    Err((x, HEIGHT - v, z))
                }
            }
            Side::Down => {
                assert!(n < HEIGHT);
                if n <= y {
                    Ok((x, y - n, z))
                } else {
                    Err((x, HEIGHT - n + y, z))
                }
            }
            Side::Forth => {
                assert!(n < DEPTH);
                let v = z + n;
                if v < DEPTH {
                    Ok((x, y, v))
                } else {
                    Err((x, y, v - DEPTH))
                }
            }
            Side::Back => {
                assert!(n < DEPTH);
                if n <= z {
                    Ok((x, y, z - n))
                } else {
                    Err((x, y, DEPTH - n + z))
                }
            }
        };

        unsafe {
            points
                .map(|(x, y, z)| Self::new_unchecked(x, y, z))
                .map_err(|(x, y, z)| Self::new_unchecked(x, y, z))
        }
    }
}

impl From<InnerPoint> for (u8, u8, u8) {
    fn from(InnerPoint { x, y, z }: InnerPoint) -> Self {
        (x, y, z)
    }
}

impl fmt::Debug for InnerPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Self { x, y, z } = self;
        write!(f, "[{x}, {y}, {z}]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn point(x: u8, y: u8, z: u8) -> InnerPoint {
        InnerPoint::new(x, y, z).unwrap()
    }

    #[test]
    fn to() {
        let a = point(1, 0, 0);
        assert_eq!(a.to(Side::Left, 0), Ok(a));
        assert_eq!(a.to(Side::Left, 1), Ok(point(2, 0, 0)));
        assert_eq!(a.to(Side::Left, WIDTH - 1), Err(point(0, 0, 0)));
        assert_eq!(a.to(Side::Right, 0), Ok(a));
        assert_eq!(a.to(Side::Right, 1), Ok(point(0, 0, 0)));
        assert_eq!(a.to(Side::Right, 2), Err(point(WIDTH - 1, 0, 0)));

        let a = point(0, 1, 0);
        assert_eq!(a.to(Side::Up, 0), Ok(a));
        assert_eq!(a.to(Side::Up, 1), Ok(point(0, 2, 0)));
        assert_eq!(a.to(Side::Up, HEIGHT - 1), Err(point(0, 0, 0)));
        assert_eq!(a.to(Side::Down, 0), Ok(a));
        assert_eq!(a.to(Side::Down, 1), Ok(point(0, 0, 0)));
        assert_eq!(a.to(Side::Down, 2), Err(point(0, HEIGHT - 1, 0)));

        let a = point(0, 0, 1);
        assert_eq!(a.to(Side::Forth, 0), Ok(a));
        assert_eq!(a.to(Side::Forth, 1), Ok(point(0, 0, 2)));
        assert_eq!(a.to(Side::Forth, DEPTH - 1), Err(point(0, 0, 0)));
        assert_eq!(a.to(Side::Back, 0), Ok(a));
        assert_eq!(a.to(Side::Back, 1), Ok(point(0, 0, 0)));
        assert_eq!(a.to(Side::Back, 2), Err(point(0, 0, DEPTH - 1)));
    }
}
