use std::fmt;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Point {
    x: i8,
    y: i8,
    z: i8,
}

impl Point {
    pub const fn new(x: i8, y: i8, z: i8) -> Option<Self> {
        if x != i8::MIN && y != i8::MIN && z != i8::MIN {
            Some(Self { x, y, z })
        } else {
            None
        }
    }
}

impl From<Point> for (i8, i8, i8) {
    fn from(Point { x, y, z }: Point) -> Self {
        (x, y, z)
    }
}

impl TryFrom<(i8, i8, i8)> for Point {
    type Error = ();

    fn try_from((x, y, z): (i8, i8, i8)) -> Result<Self, Self::Error> {
        Self::new(x, y, z).ok_or(())
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (x, y, z) = (*self).into();
        write!(f, "[{x}, {y}, {z}]")
    }
}
