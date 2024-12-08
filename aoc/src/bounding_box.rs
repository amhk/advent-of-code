use crate::XY;

#[derive(Debug, Clone, PartialEq)]
pub struct BoundingBox {
    top_left: XY,
    bottom_right: XY,
}

impl BoundingBox {
    /// Create a new BoundingBox, defined by the two XY objects a and b. The coordinates
    /// represented by a and b can be top-left and bottom-right, or bottom-left and top-right, and
    /// are both inclusive (to allow both to be (0, 0)).
    pub fn new(a: XY, b: XY) -> Self {
        BoundingBox {
            top_left: XY {
                x: if a.x < b.x { a.x } else { b.x },
                y: if a.y < b.y { a.y } else { b.y },
            },
            bottom_right: XY {
                x: if a.x > b.x { a.x } else { b.x },
                y: if a.y > b.y { a.y } else { b.y },
            },
        }
    }

    /// Check if an XY coordinate is within the bounding box (including on the border of the
    /// bounding box).
    pub fn contains(&self, xy: &XY) -> bool {
        xy.x >= self.top_left.x
            && xy.x <= self.bottom_right.x
            && xy.y >= self.top_left.y
            && xy.y <= self.bottom_right.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let a = BoundingBox::new((0, 1).into(), (2, 3).into());
        let b = BoundingBox::new((0, 3).into(), (2, 1).into());
        assert_eq!(a.top_left, b.top_left);
    }

    #[test]
    fn test_contains() {
        // ....
        // +-+.
        // |.|.
        // +-+.
        let bb = BoundingBox::new((0, 1).into(), (2, 3).into());

        assert!(bb.contains(&(0, 1).into()));
        assert!(bb.contains(&(2, 3).into()));
        assert!(bb.contains(&(2, 1).into()));
        assert!(bb.contains(&(0, 3).into()));
        assert!(bb.contains(&(1, 1).into()));
        assert!(bb.contains(&(1, 2).into()));

        assert!(!bb.contains(&(0, 0).into()));
        assert!(!bb.contains(&(-1, -1).into()));
    }
}
