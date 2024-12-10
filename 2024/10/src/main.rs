use anyhow::{anyhow, Result};
use aoc::{parse_grid, XY};
use std::collections::{HashMap, HashSet};

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 501)?;
    aoc::run!(part_two(input), 1017)?;
    Ok(())
}

type Grid = HashMap<XY, usize>;

fn valid_trail_neighbours(grid: &Grid, xy: &XY) -> Vec<XY> {
    let Some(value) = grid.get(xy) else {
        return vec![];
    };
    let mut v = vec![];
    for n in xy.four_neighbours() {
        if grid.get(&n) == Some(value + 1).as_ref() {
            v.push(n);
        }
    }
    v
}

fn parse(input: &str) -> Result<Grid> {
    let mut grid = Grid::new();
    let _ = parse_grid(input, |xy, ch| {
        let value =
            ch.to_digit(10)
                .ok_or_else(|| anyhow!("failed to convert {ch} to digit"))? as usize;
        grid.insert(xy, value);
        Ok(())
    })?;
    Ok(grid)
}

fn count_trails(grid: &Grid, xy: &XY, already_visited: &mut Option<HashSet<XY>>) -> usize {
    if let Some(already_visited) = already_visited {
        if already_visited.contains(xy) {
            return 0;
        }
        already_visited.insert(*xy);
    }
    if grid[xy] == 9 {
        return 1;
    }
    let mut count = 0;
    for n in valid_trail_neighbours(grid, xy) {
        count += count_trails(grid, &n, already_visited);
    }
    count
}

fn part_x(input: &str, part_two: bool) -> Result<usize> {
    let grid = parse(input)?;
    let mut count = 0;
    for xy in grid.keys() {
        if grid[xy] == 0 {
            let mut visited = if !part_two {
                Some(HashSet::new())
            } else {
                None
            };
            count += count_trails(&grid, xy, &mut visited);
        }
    }
    Ok(count)
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

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 36);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 81);
    }
}
