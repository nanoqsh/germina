pub trait Layout {
    fn to_index(point: (u32, u32, u32)) -> u32;
    fn to_point(index: u32) -> (u32, u32, u32);
}

pub struct Straight<const X: u32, const Y: u32>;

impl<const X: u32, const Y: u32> Layout for Straight<X, Y> {
    fn to_index((x, y, z): (u32, u32, u32)) -> u32 {
        z * X * Y + y * X + x
    }

    fn to_point(index: u32) -> (u32, u32, u32) {
        debug_assert!(X != 0);
        debug_assert!(Y != 0);

        (index % X, index / X % Y, index / X / Y)
    }
}

pub struct Curve;

impl Layout for Curve {
    fn to_index((mut x, mut y, mut z): (u32, u32, u32)) -> u32 {
        let mut index = 0;
        let mut step = 0;
        while x != 0 || y != 0 || z != 0 {
            index |= ((x & 1) | ((y & 1) << 1) | ((z & 1) << 2)) << step;
            step += 3;
            x >>= 1;
            y >>= 1;
            z >>= 1;
        }

        index
    }

    fn to_point(mut index: u32) -> (u32, u32, u32) {
        let (mut x, mut y, mut z) = (0, 0, 0);
        let mut step = 0;
        while index != 0 {
            x |= (index & 0b001) << step;
            y |= ((index & 0b010) >> 1) << step;
            z |= ((index & 0b100) >> 2) << step;
            step += 1;
            index >>= 3;
        }

        (x, y, z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::zip;

    const STRAIGHT_ORDER: [(u32, u32, u32); 12] = [
        (0, 0, 0),
        (1, 0, 0),
        (0, 1, 0),
        (1, 1, 0),
        (0, 2, 0),
        (1, 2, 0),
        (0, 0, 1),
        (1, 0, 1),
        (0, 1, 1),
        (1, 1, 1),
        (0, 2, 1),
        (1, 2, 1),
    ];

    #[test]
    fn straight_to_point() {
        for (index, point) in zip(0.., STRAIGHT_ORDER) {
            assert_eq!(Straight::<2, 3>::to_point(index), point);
        }
    }

    #[test]
    fn straight_to_index() {
        for (index, point) in zip(0.., STRAIGHT_ORDER) {
            assert_eq!(Straight::<2, 3>::to_index(point), index);
        }
    }

    const CURVE_ORDER: [(u32, u32, u32); 32] = [
        (0, 0, 0),
        (1, 0, 0),
        (0, 1, 0),
        (1, 1, 0),
        (0, 0, 1),
        (1, 0, 1),
        (0, 1, 1),
        (1, 1, 1),
        (2, 0, 0),
        (3, 0, 0),
        (2, 1, 0),
        (3, 1, 0),
        (2, 0, 1),
        (3, 0, 1),
        (2, 1, 1),
        (3, 1, 1),
        (0, 2, 0),
        (1, 2, 0),
        (0, 3, 0),
        (1, 3, 0),
        (0, 2, 1),
        (1, 2, 1),
        (0, 3, 1),
        (1, 3, 1),
        (2, 2, 0),
        (3, 2, 0),
        (2, 3, 0),
        (3, 3, 0),
        (2, 2, 1),
        (3, 2, 1),
        (2, 3, 1),
        (3, 3, 1),
    ];

    #[test]
    fn curve_to_point() {
        for (index, point) in zip(0.., CURVE_ORDER) {
            assert_eq!(Curve::to_point(index), point);
        }
    }

    #[test]
    fn curve_to_index() {
        for (index, point) in zip(0.., CURVE_ORDER) {
            assert_eq!(Curve::to_index(point), index);
        }
    }
}
