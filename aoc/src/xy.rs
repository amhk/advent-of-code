use std::fmt::Debug;
use std::ops::{Add, AddAssign, Sub, SubAssign};

/// A pair of (x, y) coordinates.
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
pub struct XY {
    pub x: i32,
    pub y: i32,
}

impl XY {
    /// Get the four neighbouring XY coordinates (north, east, south and west) of this XY
    /// coordinate.
    pub fn four_neighbours(&self) -> [XY; 4] {
        [
            (self.x, self.y - 1).into(),
            (self.x + 1, self.y).into(),
            (self.x, self.y + 1).into(),
            (self.x - 1, self.y).into(),
        ]
    }
}

impl From<(i32, i32)> for XY {
    fn from(value: (i32, i32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

macro_rules! impl_add {
    ($lhs: ty, $rhs: ty) => {
        impl Add<$rhs> for $lhs {
            type Output = XY;

            fn add(self, rhs: $rhs) -> Self::Output {
                XY {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                }
            }
        }
    };
}

impl_add! {XY, XY}
impl_add! {&XY, XY}
impl_add! {XY, &XY}
impl_add! {&XY, &XY}

macro_rules! impl_add_assign {
    ($lhs: ty, $rhs: ty) => {
        impl AddAssign<$rhs> for $lhs {
            fn add_assign(&mut self, rhs: $rhs) {
                self.x = self.x + rhs.x;
                self.y = self.y + rhs.y;
            }
        }
    };
}

impl_add_assign! {XY, XY}
impl_add_assign! {&mut XY, XY}
impl_add_assign! {XY, &XY}
impl_add_assign! {&mut XY, &XY}

macro_rules! impl_sub {
    ($lhs: ty, $rhs: ty) => {
        impl Sub<$rhs> for $lhs {
            type Output = XY;

            fn sub(self, rhs: $rhs) -> Self::Output {
                XY {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                }
            }
        }
    };
}

impl_sub! {XY, XY}
impl_sub! {&XY, XY}
impl_sub! {XY, &XY}
impl_sub! {&XY, &XY}

macro_rules! impl_sub_assign {
    ($lhs: ty, $rhs: ty) => {
        impl SubAssign<$rhs> for $lhs {
            fn sub_assign(&mut self, rhs: $rhs) {
                self.x = self.x - rhs.x;
                self.y = self.y - rhs.y;
            }
        }
    };
}

impl_sub_assign! {XY, XY}
impl_sub_assign! {&mut XY, XY}
impl_sub_assign! {XY, &XY}
impl_sub_assign! {&mut XY, &XY}

impl Debug for XY {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_four_neighbours() {
        let xy = XY { x: 1, y: 2 };
        let mut actual = xy.four_neighbours();
        actual.sort();
        let mut expected = [(1, 1).into(), (2, 2).into(), (1, 3).into(), (0, 2).into()];
        expected.sort();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_from() {
        let a: XY = XY { x: 1, y: 2 };
        let b: XY = (1, 2).into();
        assert_eq!(a, b);
    }

    #[test]
    fn test_add() {
        let a: XY = XY { x: 1, y: 2 };
        let b: XY = XY { x: 10, y: 20 };
        let sum = XY { x: 11, y: 22 };
        assert_eq!(a + b, sum);
        assert_eq!(&a + b, sum);
        assert_eq!(a + &b, sum);
        assert_eq!(&a + &b, sum);
    }

    #[test]
    fn test_add_assign() {
        let b: XY = XY { x: 10, y: 20 };

        let mut a: XY = XY { x: 1, y: 2 };
        a += b;
        assert_eq!(a, XY { x: 11, y: 22 });

        let mut a: XY = XY { x: 1, y: 2 };
        let mut a = &mut a;
        a += b;
        assert_eq!(*a, XY { x: 11, y: 22 });

        let mut a: XY = XY { x: 1, y: 2 };
        a += &b;
        assert_eq!(a, XY { x: 11, y: 22 });

        let mut a: XY = XY { x: 1, y: 2 };
        let mut a = &mut a;
        a += &b;
        assert_eq!(*a, XY { x: 11, y: 22 });
    }

    #[test]
    fn test_sub() {
        let a: XY = XY { x: 1, y: 2 };
        let b: XY = XY { x: 10, y: 20 };
        let diff = XY { x: -9, y: -18 };
        assert_eq!(a - b, diff);
        assert_eq!(&a - b, diff);
        assert_eq!(a - &b, diff);
        assert_eq!(&a - &b, diff);
    }

    #[test]
    fn test_sub_assign() {
        let b: XY = XY { x: 10, y: 20 };

        let mut a: XY = XY { x: 1, y: 2 };
        a -= b;
        assert_eq!(a, XY { x: -9, y: -18 });

        let mut a: XY = XY { x: 1, y: 2 };
        let mut a = &mut a;
        a -= b;
        assert_eq!(*a, XY { x: -9, y: -18 });

        let mut a: XY = XY { x: 1, y: 2 };
        a -= &b;
        assert_eq!(a, XY { x: -9, y: -18 });

        let mut a: XY = XY { x: 1, y: 2 };
        let mut a = &mut a;
        a -= &b;
        assert_eq!(*a, XY { x: -9, y: -18 });
    }

    #[test]
    fn test_debug() {
        let a: XY = XY { x: 1, y: 2 };
        assert_eq!(format!("{:?}", a), "(1, 2)");
    }
}
