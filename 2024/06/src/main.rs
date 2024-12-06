use anyhow::{anyhow, bail, ensure, Result};
use rustc_hash::FxHashSet;

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 4722)?;
    aoc::run!(part_two(input), 1602)?;
    Ok(())
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
struct XY {
    x: i32,
    y: i32,
}

impl From<(i32, i32)> for XY {
    fn from(value: (i32, i32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl XY {
    fn north(&self) -> XY {
        (self.x, self.y - 1).into()
    }

    fn south(&self) -> XY {
        (self.x, self.y + 1).into()
    }

    fn west(&self) -> XY {
        (self.x - 1, self.y).into()
    }

    fn east(&self) -> XY {
        (self.x + 1, self.y).into()
    }

    fn forward(&self, direction: Direction) -> XY {
        match direction {
            Direction::North => self.north(),
            Direction::East => self.east(),
            Direction::South => self.south(),
            Direction::West => self.west(),
        }
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn ninety_degrees_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

struct Grid {
    bounding_box: (XY, XY),
    obstacles: FxHashSet<XY>,
}

fn parse(input: &str) -> Result<(Grid, XY)> {
    let mut x = 0;
    let mut y = 0;
    let mut max = (0, 0);
    let mut start: Option<XY> = None;
    let mut obstacles = FxHashSet::default();
    for ch in input.chars() {
        match ch {
            '\n' => {
                max = (x - 1, y);
                x = 0;
                y += 1;
            }
            '.' => {
                x += 1;
            }
            '#' => {
                obstacles.insert((x, y).into());
                x += 1;
            }
            '^' => {
                start = Some((x, y).into());
                x += 1;
            }
            _ => bail!("unexpected char '{}'", ch),
        }
    }
    Ok((
        Grid {
            bounding_box: ((0, 0).into(), (max).into()),
            obstacles,
        },
        start.ok_or_else(|| anyhow!("no start position found"))?,
    ))
}

// Return all visited squares, or None if the pattern loops
fn simulate_patrol(grid: &Grid, mut position: XY) -> Option<FxHashSet<XY>> {
    let mut direction = Direction::North;
    let mut seen = FxHashSet::default();
    loop {
        if position.x < grid.bounding_box.0.x
            || position.x > grid.bounding_box.1.x
            || position.y < grid.bounding_box.0.y
            || position.y > grid.bounding_box.1.y
        {
            break;
        }
        if !seen.insert((position, direction)) {
            // loop detected
            return None;
        }
        while grid.obstacles.contains(&position.forward(direction)) {
            direction = direction.ninety_degrees_right();
        }
        position = position.forward(direction);
    }
    Some(seen.into_iter().map(|(xy, _)| xy).collect())
}

fn part_one(input: &str) -> Result<usize> {
    let (grid, start) = parse(input)?;
    simulate_patrol(&grid, start)
        .map(|set| set.len())
        .ok_or_else(|| anyhow!("unexpected loop detected"))
}

fn part_two(input: &str) -> Result<usize> {
    let (mut grid, start) = parse(input)?;
    let mut visited =
        simulate_patrol(&grid, start).ok_or_else(|| anyhow!("unexpected loop detected"))?;
    ensure!(visited.remove(&start));

    let mut count = 0;
    for candidate in visited.into_iter() {
        debug_assert!(!grid.obstacles.contains(&candidate));
        grid.obstacles.insert(candidate);
        if simulate_patrol(&grid, start).is_none() {
            count += 1;
        }
        grid.obstacles.remove(&candidate);
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_parse() {
        let (grid, start) = parse(INPUT).unwrap();
        assert_eq!(start, (4, 6).into());
        assert_eq!(grid.obstacles.len(), 8);
        assert!(grid.obstacles.contains(&(4, 0).into()));
        assert!(!grid.obstacles.contains(&(0, 0).into()));
        assert_eq!(grid.bounding_box, ((0, 0).into(), (9, 9).into()));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 41);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 6);
    }
}
