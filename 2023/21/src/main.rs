use anyhow::{anyhow, bail, ensure, Result};
use std::collections::BTreeSet;

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input, 64), 3637)?;
    aoc::run!(part_two(input, 26_501_365), 601113643448699)?;
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct XY {
    x: i32,
    y: i32,
}

impl From<(i32, i32)> for XY {
    fn from(value: (i32, i32)) -> Self {
        XY {
            x: value.0,
            y: value.1,
        }
    }
}

impl XY {
    fn north(&self) -> XY {
        (self.x, self.y - 1).into()
    }

    fn east(&self) -> XY {
        (self.x + 1, self.y).into()
    }

    fn south(&self) -> XY {
        (self.x, self.y + 1).into()
    }

    fn west(&self) -> XY {
        (self.x - 1, self.y).into()
    }
}

struct MapSlice {
    cells: BTreeSet<XY>,
    width: usize,
    height: usize,
}

fn parse(input: &str) -> Result<(MapSlice, XY)> {
    let mut cells = BTreeSet::new();
    let mut start: Option<XY> = None;
    let height = input.lines().count();
    ensure!(height > 0);
    let width = input.lines().count();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            match ch {
                'S' => {
                    ensure!(start.is_none(), "multiple start positions");
                    start = Some((x as i32, y as i32).into());
                    cells.insert((x as i32, y as i32).into());
                }
                '.' => {
                    cells.insert((x as i32, y as i32).into());
                }
                '#' => {}
                _ => {
                    bail!("unexpected character '{ch}'");
                }
            }
        }
    }
    let start = start.ok_or_else(|| anyhow!("no start position"))?;
    Ok((
        MapSlice {
            cells,
            width,
            height,
        },
        start,
    ))
}

fn solve(input: &str, total_number_of_steps: usize) -> Result<usize> {
    let (map, start) = parse(input)?;
    let width = map.width as i32;
    let height = map.height as i32;
    let mut previous_frontier: BTreeSet<XY> = BTreeSet::new();
    let mut current_frontier: BTreeSet<XY> = BTreeSet::from([start]);
    // odd total_number_of_step: start at 1 because zero moves == one position (the start position)
    // even total_number_of_step: start at 0
    let mut count = if total_number_of_steps.is_multiple_of(2) {
        1
    } else {
        0
    };
    for step in 1..=total_number_of_steps {
        let mut next_frontier: BTreeSet<XY> = BTreeSet::new();
        for origin in current_frontier.iter() {
            for xy in [origin.north(), origin.east(), origin.south(), origin.west()] {
                let offset_x = {
                    let x = xy.x % width;
                    if x < 0 {
                        x + width
                    } else {
                        x
                    }
                };
                let offset_y = {
                    let y = xy.y % height;
                    if y < 0 {
                        y + height
                    } else {
                        y
                    }
                };
                if !previous_frontier.contains(&xy)
                    && map.cells.contains(&(offset_x, offset_y).into())
                {
                    next_frontier.insert(xy);
                }
            }
        }
        if step % 2 == total_number_of_steps % 2 {
            count += next_frontier.len();
        }
        previous_frontier = current_frontier;
        current_frontier = next_frontier;
    }
    Ok(count)
}

fn part_one(input: &str, total_number_of_steps: usize) -> Result<usize> {
    solve(input, total_number_of_steps)
}

fn part_two(input: &str, total_number_of_steps: usize) -> Result<usize> {
    let (map, _) = parse(input)?;
    ensure!(map.width == map.height);
    let size = map.width;
    let rest = total_number_of_steps % size;

    // credit to https://www.reddit.com/r/adventofcode/comments/18nevo3/comment/kghck00 for
    // pointing me in the right direction
    //
    // after the initial `rest` number of steps, the number of plots visited grows exponentially (y
    // = ax^2 + bx + c) for each increment of `size` steps
    //
    // calculate three data points (y0, y1, y2) for three different increments of `n * size + rest`
    // then calculate the values of a, b, c
    // then calculate y for a large value of x
    let y0 = solve(input, rest)? as f64;
    let y1 = solve(input, size + rest)? as f64;
    let y2 = solve(input, 2 * size + rest)? as f64;

    let a = (y0 - 2.0 * y1 + y2) / 2.0;
    let b = (4.0 * y1 - 3.0 * y0 - y2) / 2.0;
    let c = y0;

    let x = (total_number_of_steps / size) as f64;
    let y = (a * x * x) + (b * x) + c;
    Ok(y as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_solve() {
        assert_eq!(solve(INPUT, 0).unwrap(), 1);
        assert_eq!(solve(INPUT, 1).unwrap(), 2);
        assert_eq!(solve(INPUT, 2).unwrap(), 4);
        assert_eq!(solve(INPUT, 3).unwrap(), 6);
        assert_eq!(solve(INPUT, 6).unwrap(), 16);
        assert_eq!(solve(INPUT, 10).unwrap(), 50);
        assert_eq!(solve(INPUT, 50).unwrap(), 1594);
        assert_eq!(solve(INPUT, 100).unwrap(), 6536);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT, 6).unwrap(), 16);
    }

    // no tests for part_two: the test input data does not have the properties required
}
