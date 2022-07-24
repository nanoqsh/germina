use std::{fmt, ops};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Side {
    Left = 0,
    Right = 1,
    Up = 2,
    Down = 3,
    Forth = 4,
    Back = 5,
}

impl Side {
    #[must_use]
    pub const fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Forth => Self::Back,
            Self::Back => Self::Forth,
        }
    }
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            Self::Left => 'l',
            Self::Right => 'r',
            Self::Up => 'u',
            Self::Down => 'd',
            Self::Forth => 'f',
            Self::Back => 'b',
        };

        write!(f, "{}", c)
    }
}

impl<S> ops::BitOr<S> for Side
where
    S: Into<Sides>,
{
    type Output = Sides;

    fn bitor(self, rhs: S) -> Self::Output {
        let lhs: Sides = self.into();
        lhs | rhs.into()
    }
}

#[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
pub struct Sides(u8);

impl Sides {
    const ALL: Self = Self(0b0011_1111);

    #[must_use]
    pub const fn empty() -> Self {
        Self(0)
    }

    #[must_use]
    pub const fn all() -> Self {
        Self::ALL
    }

    #[must_use]
    pub const fn len(self) -> usize {
        self.0.count_ones() as usize
    }

    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    #[must_use]
    pub fn contains(self, side: Side) -> bool {
        let sides: Self = side.into();
        self.0 & sides.0 != 0
    }

    pub fn remove(&mut self, side: Side) {
        self.0 &= !(1 << side as u8);
    }
}

impl From<Side> for Sides {
    fn from(side: Side) -> Self {
        Self(1 << side as u8)
    }
}

impl fmt::Display for Sides {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for side in *self {
            write!(f, "{side}")?;
        }
        write!(f, "]")
    }
}

impl fmt::Debug for Sides {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl ops::Not for Sides {
    type Output = Self;

    fn not(mut self) -> Self::Output {
        self.0 ^= Self::ALL.0;
        self
    }
}

impl<S> ops::BitOrAssign<S> for Sides
where
    S: Into<Self>,
{
    fn bitor_assign(&mut self, rhs: S) {
        let rhs = rhs.into();
        self.0 |= rhs.0;
    }
}

impl<S> ops::BitOr<S> for Sides
where
    S: Into<Self>,
{
    type Output = Self;

    fn bitor(mut self, rhs: S) -> Self::Output {
        self |= rhs.into();
        self
    }
}

impl ops::BitAndAssign for Sides {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl ops::BitAnd for Sides {
    type Output = Self;

    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}

impl FromIterator<Side> for Sides {
    fn from_iter<T: IntoIterator<Item = Side>>(iter: T) -> Self {
        iter.into_iter().fold(Self::empty(), ops::BitOr::bitor)
    }
}

impl IntoIterator for Sides {
    type Item = Side;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

pub struct IntoIter(Sides);

impl Iterator for IntoIter {
    type Item = Side;

    fn next(&mut self) -> Option<Self::Item> {
        for side in [
            Side::Left,
            Side::Right,
            Side::Up,
            Side::Down,
            Side::Forth,
            Side::Back,
        ] {
            if self.0.contains(side) {
                self.0.remove(side);
                return Some(side);
            }
        }

        None
    }
}
