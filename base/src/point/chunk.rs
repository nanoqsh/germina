use crate::{chunk::point::InnerPoint, point::Error, side::Side};
use std::fmt;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Point(InnerPoint);

impl Point {
    pub fn new(x: u8, y: u8, z: u8) -> Result<Self, Error> {
        InnerPoint::new(x, y, z).map(Self).ok_or(Error)
    }

    /// Steps to `side` `n` times.
    ///
    /// If it stops in the current chunk bounds, returns `Ok`.
    /// If it goes to the next chunk, returns `Err`.
    ///
    /// # Panics
    ///
    /// Panics if `n` is not in chunk bounds.
    pub fn to(self, side: Side, n: u8) -> Result<Self, Self> {
        self.0.to(side, n).map(Self).map_err(Self)
    }

    pub(crate) fn into_inner(self) -> InnerPoint {
        self.0
    }
}

impl From<Point> for (u8, u8, u8) {
    fn from(point: Point) -> Self {
        point.0.into()
    }
}

impl TryFrom<(u8, u8, u8)> for Point {
    type Error = Error;

    fn try_from((x, y, z): (u8, u8, u8)) -> Result<Self, Self::Error> {
        Self::new(x, y, z)
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
