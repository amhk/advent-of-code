#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    /// Get the direction you would face if you turned 90 degrees to the right
    pub fn turn_right(&self) -> Direction {
        match &self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    /// Get the direction you would face if you turned 90 degrees to the left
    pub fn turn_left(&self) -> Direction {
        match &self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    /// Get the direction you would face if you turned 180 degrees
    pub fn turn_180(&self) -> Direction {
        match &self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turns() {
        #[rustfmt::skip]
        macro_rules! assert_turns {
            ($left:expr, $dir:expr, $right:expr) => {
                assert_eq!($dir.turn_right(), $right, "turn_right x 1");
                assert_eq!($dir.turn_left(), $left, "turn_left x 1");

                assert_eq!($dir.turn_right().turn_right(), $dir.turn_180(), "turn_right x 2");
                assert_eq!($dir.turn_left().turn_left(), $dir.turn_180(), "turn_left x 2");

                assert_eq!($dir.turn_right().turn_right().turn_right(), $dir.turn_left(), "turn_right x 3");
                assert_eq!($dir.turn_left().turn_left().turn_left(), $dir.turn_right(), "turn_left x 3");

                assert_eq!($dir.turn_right().turn_right().turn_right().turn_right(), $dir, "turn_right x 4");
                assert_eq!($dir.turn_left().turn_left().turn_left().turn_left(), $dir, "turn_left x 4");
            };
        }

        assert_turns!(Direction::West, Direction::North, Direction::East);
        assert_turns!(Direction::North, Direction::East, Direction::South);
        assert_turns!(Direction::East, Direction::South, Direction::West);
        assert_turns!(Direction::South, Direction::West, Direction::North);
    }
}
