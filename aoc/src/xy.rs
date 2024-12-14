use crate::Direction;
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
        [self.north(), self.east(), self.south(), self.west()]
    }

    /// Get the XY coordinate one step north of this one
    pub fn north(&self) -> XY {
        (self.x, self.y - 1).into()
    }

    /// Get the XY coordinate one step north-east of this one
    pub fn north_east(&self) -> XY {
        (self.x + 1, self.y - 1).into()
    }

    /// Get the XY coordinate one step east of this one
    pub fn east(&self) -> XY {
        (self.x + 1, self.y).into()
    }

    /// Get the XY coordinate one step south-east of this one
    pub fn south_east(&self) -> XY {
        (self.x + 1, self.y + 1).into()
    }

    /// Get the XY coordinate one step south of this one
    pub fn south(&self) -> XY {
        (self.x, self.y + 1).into()
    }

    /// Get the XY coordinate one step south-west of this one
    pub fn south_west(&self) -> XY {
        (self.x - 1, self.y + 1).into()
    }

    /// Get the XY coordinate one step west of this one
    pub fn west(&self) -> XY {
        (self.x - 1, self.y).into()
    }

    /// Get the XY coordinate one step north-west of this one
    pub fn north_west(&self) -> XY {
        (self.x - 1, self.y - 1).into()
    }

    /// Get the XY coordinate one step ahead in the given direction
    pub fn forward(&self, dir: Direction) -> XY {
        match dir {
            Direction::North => self.north(),
            Direction::East => self.east(),
            Direction::South => self.south(),
            Direction::West => self.west(),
        }
    }

    /// Get the XY coordinate one step to the right of the given direction
    pub fn right(&self, dir: Direction) -> XY {
        match dir {
            Direction::North => self.east(),
            Direction::East => self.south(),
            Direction::South => self.west(),
            Direction::West => self.north(),
        }
    }

    /// Get the XY coordinate one step to the left of the given direction
    pub fn left(&self, dir: Direction) -> XY {
        match dir {
            Direction::North => self.west(),
            Direction::East => self.north(),
            Direction::South => self.east(),
            Direction::West => self.south(),
        }
    }

    /// Get the XY coordinate one step back in the given direction
    pub fn behind(&self, dir: Direction) -> XY {
        match dir {
            Direction::North => self.south(),
            Direction::East => self.west(),
            Direction::South => self.north(),
            Direction::West => self.east(),
        }
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
    fn test_north() {
        let xy = XY { x: 1, y: 2 };
        assert_eq!(xy.north(), XY { x: 1, y: 1 });
    }

    #[test]
    fn test_north_east() {
        let xy = XY { x: 0, y: 0 };
        assert_eq!(xy.north_east(), XY { x: 1, y: -1 });
    }

    #[test]
    fn test_east() {
        let xy = XY { x: 1, y: 2 };
        assert_eq!(xy.east(), XY { x: 2, y: 2 });
    }

    #[test]
    fn test_south_east() {
        let xy = XY { x: 0, y: 0 };
        assert_eq!(xy.south_east(), XY { x: 1, y: 1 });
    }

    #[test]
    fn test_south() {
        let xy = XY { x: 1, y: 2 };
        assert_eq!(xy.south(), XY { x: 1, y: 3 });
    }

    #[test]
    fn test_south_west() {
        let xy = XY { x: 0, y: 0 };
        assert_eq!(xy.south_west(), XY { x: -1, y: 1 });
    }

    #[test]
    fn test_west() {
        let xy = XY { x: 1, y: 2 };
        assert_eq!(xy.west(), XY { x: 0, y: 2 });
    }

    #[test]
    fn test_north_west() {
        let xy = XY { x: 0, y: 0 };
        assert_eq!(xy.north_west(), XY { x: -1, y: -1 });
    }

    #[test]
    fn test_forward() {
        let xy = XY { x: 1, y: 2 };
        assert_eq!(xy.forward(Direction::East), XY { x: 2, y: 2 });
    }

    #[test]
    fn test_left() {
        let xy = XY { x: 1, y: 2 };
        assert_eq!(xy.left(Direction::East), XY { x: 1, y: 1 });
    }

    #[test]
    fn test_right() {
        let xy = XY { x: 1, y: 2 };
        assert_eq!(xy.right(Direction::East), XY { x: 1, y: 3 });
    }

    #[test]
    fn test_behind() {
        let xy = XY { x: 1, y: 2 };
        assert_eq!(xy.behind(Direction::East), XY { x: 0, y: 2 });
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
