use anyhow::{bail, Result};
use std::collections::BTreeMap;

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 7173)?;
    aoc::run!(part_two(input), 291)?;
    Ok(())
}

type Grid = BTreeMap<XY, Pipe>;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
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
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Pipe {
    Start,      // unknown
    Vertical,   // |
    Horizontal, // -
    NorthEast,  // └
    NorthWest,  // ┘
    SouthWest,  // ┐
    SouthEast,  // ┌
}

impl TryFrom<char> for Pipe {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            'S' => Ok(Pipe::Start),
            '|' => Ok(Pipe::Vertical),
            '-' => Ok(Pipe::Horizontal),
            'L' => Ok(Pipe::NorthEast),
            'J' => Ok(Pipe::NorthWest),
            '7' => Ok(Pipe::SouthWest),
            'F' => Ok(Pipe::SouthEast),
            _ => Err(anyhow::anyhow!("{}: not a pipe char", value)),
        }
    }
}

fn parse(input: &str) -> Result<Grid> {
    let mut grid: BTreeMap<XY, Pipe> = BTreeMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if let Ok(pipe) = Pipe::try_from(ch) {
                grid.insert((x as i32, y as i32).into(), pipe);
            }
        }
    }
    Ok(grid)
}

// - history must include at least one element
fn find_path_to_start(grid: &Grid, history: &mut Vec<XY>, current: XY) -> bool {
    let Some(current_pipe) = grid.get(&current) else {
        return false;
    };
    let previous = *history.last().unwrap();
    match current_pipe {
        Pipe::Start => true,
        Pipe::Vertical => {
            history.push(current);
            if current.north() == previous {
                find_path_to_start(grid, history, current.south())
            } else if current.south() == previous {
                find_path_to_start(grid, history, current.north())
            } else {
                false
            }
        }
        Pipe::Horizontal => {
            history.push(current);
            if current.east() == previous {
                find_path_to_start(grid, history, current.west())
            } else if current.west() == previous {
                find_path_to_start(grid, history, current.east())
            } else {
                false
            }
        }
        Pipe::NorthEast => {
            history.push(current);
            if current.north() == previous {
                find_path_to_start(grid, history, current.east())
            } else if current.east() == previous {
                find_path_to_start(grid, history, current.north())
            } else {
                false
            }
        }
        Pipe::NorthWest => {
            history.push(current);
            if current.north() == previous {
                find_path_to_start(grid, history, current.west())
            } else if current.west() == previous {
                find_path_to_start(grid, history, current.north())
            } else {
                false
            }
        }
        Pipe::SouthWest => {
            history.push(current);
            if current.south() == previous {
                find_path_to_start(grid, history, current.west())
            } else if current.west() == previous {
                find_path_to_start(grid, history, current.south())
            } else {
                false
            }
        }
        Pipe::SouthEast => {
            history.push(current);
            if current.south() == previous {
                find_path_to_start(grid, history, current.east())
            } else if current.east() == previous {
                find_path_to_start(grid, history, current.south())
            } else {
                false
            }
        }
    }
}

fn find_loop(input: &str) -> Result<Vec<XY>> {
    let grid = parse(input)?;
    let start_xy = *grid
        .iter()
        .find(|(_, value)| **value == Pipe::Start)
        .map(|(key, _)| key)
        .ok_or(anyhow::anyhow!("no start node"))?;
    for xy in [
        start_xy.north(),
        start_xy.east(),
        start_xy.south(),
        start_xy.west(),
    ] {
        let mut history = vec![start_xy];
        if find_path_to_start(&grid, &mut history, xy) {
            return Ok(history);
        }
    }
    bail!("no solution found")
}

fn part_one(input: &str) -> Result<usize> {
    let path = find_loop(input)?;
    Ok(path.len() / 2)
}

fn part_two(input: &str) -> Result<usize> {
    let path = find_loop(input)?;
    let grid = {
        let mut grid = parse(input)?;
        // replace S with actual pipe
        let start = path[0];
        let next = path[1];
        let prev = path[path.len() - 1];
        let actual_pipe = if (start.west() == prev && start.east() == next)
            || (start.east() == prev && start.west() == next)
        {
            Pipe::Horizontal
        } else if (start.south() == prev && start.north() == next)
            || (start.north() == prev && start.south() == next)
        {
            Pipe::Vertical
        } else if (start.south() == prev && start.east() == next)
            || (start.east() == prev && start.south() == next)
        {
            Pipe::SouthEast
        } else if (start.south() == prev && start.west() == next)
            || (start.west() == prev && start.south() == next)
        {
            Pipe::SouthWest
        } else if (start.north() == prev && start.west() == next)
            || (start.west() == prev && start.north() == next)
        {
            Pipe::NorthWest
        } else if (start.north() == prev && start.east() == next)
            || (start.east() == prev && start.north() == next)
        {
            Pipe::NorthEast
        } else {
            panic!();
        };
        grid.insert(start, actual_pipe);
        grid
    };
    let min_y = path.iter().map(|xy| xy.y).min().unwrap();
    let max_y = path.iter().map(|xy| xy.y).max().unwrap();
    let min_x = path.iter().map(|xy| xy.x).min().unwrap();
    let max_x = path.iter().map(|xy| xy.x).max().unwrap();
    let mut sum: i32 = 0;
    let mut last: Option<Pipe> = None;
    for y in min_y..=max_y {
        let mut inside = false;
        for x in min_x..=max_x {
            let xy = (x, y).into();
            if path.contains(&xy) {
                match grid.get(&xy).unwrap() {
                    Pipe::Start => panic!(),
                    Pipe::Vertical => {
                        inside = !inside;
                    }
                    Pipe::Horizontal => {}
                    Pipe::NorthEast => {
                        inside = !inside;
                        last = Some(Pipe::NorthEast);
                    }
                    Pipe::NorthWest => {
                        if last == Some(Pipe::NorthEast) {
                            inside = !inside;
                        }
                        last = None;
                    }
                    Pipe::SouthWest => {
                        if last == Some(Pipe::SouthEast) {
                            inside = !inside;
                        }
                        last = None;
                    }
                    Pipe::SouthEast => {
                        inside = !inside;
                        last = Some(Pipe::SouthEast);
                    }
                }
            } else if inside {
                sum += 1;
            }
        }
    }
    Ok(sum as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_ONE: &str = include_str!("test-input-part-one.txt");
    const INPUT_TWO_1: &str = include_str!("test-input-part-two-1.txt");
    const INPUT_TWO_2: &str = include_str!("test-input-part-two-2.txt");
    const INPUT_TWO_3: &str = include_str!("test-input-part-two-3.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT_ONE).unwrap(), 8);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT_TWO_1).unwrap(), 4);
        assert_eq!(part_two(INPUT_TWO_2).unwrap(), 8);
        assert_eq!(part_two(INPUT_TWO_3).unwrap(), 10);
    }
}
