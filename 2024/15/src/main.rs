use std::collections::HashMap;

use anyhow::{anyhow, bail, Result};
use aoc::{Direction, XY};

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 1516281)?;
    aoc::run!(part_two(input), 1527969)?;
    Ok(())
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Type {
    Empty,
    Robot,
    SmallBox,
    BoxWest,
    BoxEast,
    Wall,
}

type Grid = HashMap<XY, Type>;

fn parse(input: &str, expand: bool) -> Result<(Grid, Vec<Direction>)> {
    let (first, second) = input
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("missing blank line"))?;

    let mut grid = Grid::new();
    for (y, line) in first.lines().enumerate() {
        let mut x = 0;
        for ch in line.chars() {
            let type_ = match ch {
                '.' => Type::Empty,
                '@' => Type::Robot,
                'O' => Type::SmallBox,
                '#' => Type::Wall,
                _ => bail!("unexpected char '{ch}'"),
            };
            if expand {
                if type_ == Type::SmallBox {
                    grid.insert((x, y as i32).into(), Type::BoxWest);
                    grid.insert((x + 1, y as i32).into(), Type::BoxEast);
                } else if type_ == Type::Robot {
                    grid.insert((x, y as i32).into(), Type::Robot);
                    grid.insert((x + 1, y as i32).into(), Type::Empty);
                } else {
                    grid.insert((x, y as i32).into(), type_);
                    grid.insert((x + 1, y as i32).into(), type_);
                }
                x += 2;
            } else {
                grid.insert((x, y as i32).into(), type_);
                x += 1;
            }
        }
    }

    let mut directions = vec![];
    for ch in second.chars().filter(|ch| !ch.is_whitespace()) {
        let dir = match ch {
            '^' => Direction::North,
            '>' => Direction::East,
            'v' => Direction::South,
            '<' => Direction::West,
            _ => bail!("unexpected char '{ch}'"),
        };
        directions.push(dir);
    }

    Ok((grid, directions))
}

fn try_move(grid: &Grid, from: XY, dir: Direction, shifts: &mut Vec<(XY, XY)>) -> bool {
    let to = from.forward(dir);
    match grid[&to] {
        Type::Empty => {
            if !shifts.contains(&(from, to)) {
                shifts.push((from, to));
            }
            true
        }
        Type::Robot => panic!("should not happen"),
        Type::SmallBox => {
            if try_move(grid, to, dir, shifts) {
                shifts.push((from, to));
                true
            } else {
                false
            }
        }
        Type::BoxWest => match dir {
            Direction::East => {
                let toto = to.forward(dir);
                if try_move(grid, toto, dir, shifts) {
                    shifts.push((to, toto));
                    shifts.push((from, to));
                    true
                } else {
                    false
                }
            }
            Direction::North | Direction::South => {
                let east = to.east();
                debug_assert!(grid[&east] == Type::BoxEast);
                if try_move(grid, to, dir, shifts) && try_move(grid, east, dir, shifts) {
                    if !shifts.contains(&(from, to)) {
                        shifts.push((from, to));
                    }
                    true
                } else {
                    false
                }
            }
            Direction::West => panic!("should not happen"),
        },
        Type::BoxEast => match dir {
            Direction::East => panic!("should never happen"),
            Direction::North | Direction::South => {
                let west = to.west();
                debug_assert!(grid[&west] == Type::BoxWest);
                if try_move(grid, west, dir, shifts) && try_move(grid, to, dir, shifts) {
                    if !shifts.contains(&(from, to)) {
                        shifts.push((from, to));
                    }
                    true
                } else {
                    false
                }
            }
            Direction::West => {
                let toto = to.forward(dir);
                if try_move(grid, toto, dir, shifts) {
                    shifts.push((to, toto));
                    shifts.push((from, to));
                    true
                } else {
                    false
                }
            }
        },
        Type::Wall => false,
    }
}

fn score(grid: &Grid) -> usize {
    grid.iter()
        .filter(|(_, ty)| **ty == Type::SmallBox || **ty == Type::BoxWest)
        .map(|(xy, _)| (xy.y * 100 + xy.x) as usize)
        .sum()
}

fn part_x(input: &str, expand: bool) -> Result<usize> {
    let (mut grid, directions) = parse(input, expand)?;
    let mut xy = *grid
        .iter()
        .find(|(_, ty)| **ty == Type::Robot)
        .map(|(xy, _)| xy)
        .ok_or_else(|| anyhow!("missing robot"))?;

    for dir in directions {
        let mut shifts = vec![];
        if try_move(&grid, xy, dir, &mut shifts) {
            for (from, to) in shifts {
                *grid.get_mut(&to).expect("valid index") = grid[&from];
                *grid.get_mut(&from).expect("valid index") = Type::Empty;
            }
            xy = xy.forward(dir);
        }
    }

    Ok(score(&grid))
}

fn part_one(input: &str) -> Result<usize> {
    part_x(input, false)
}

fn part_two(input: &str) -> Result<usize> {
    part_x(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_SMALL: &str = include_str!("test-input-small.txt");
    const INPUT_LARGE: &str = include_str!("test-input-large.txt");

    #[test]
    fn test_parse() {
        let (grid, _) = parse(INPUT_SMALL, false).unwrap();
        assert_eq!(grid.len(), 8 * 8);
        assert_eq!(grid[&(3, 1).into()], Type::SmallBox);

        let (grid, _) = parse(INPUT_SMALL, true).unwrap();
        assert_eq!(grid.len(), 16 * 8);
        assert_eq!(grid[&(6, 1).into()], Type::BoxWest);
        assert_eq!(grid[&(7, 1).into()], Type::BoxEast);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT_SMALL).unwrap(), 2028);
        assert_eq!(part_one(INPUT_LARGE).unwrap(), 10092);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT_LARGE).unwrap(), 9021);
    }
}
