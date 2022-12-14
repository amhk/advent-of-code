use std::collections::HashSet;
use std::iter::zip;

use anyhow::{ensure, Context, Result};

fn main() -> Result<()> {
    let input = include_str!("input.txt");

    let answer = part_one(input).context("no solution for part one")?;
    println!("Part 1: {}", answer);
    assert_eq!(answer, 774);

    let answer = part_two(input).context("no solution for part two")?;
    println!("Part 2: {}", answer);
    assert_eq!(answer, 22499);

    Ok(())
}

type XY = (i32, i32);

fn parse(input: &str) -> Result<HashSet<XY>> {
    let mut grid: HashSet<XY> = HashSet::new();
    for line in input.lines() {
        for (src, dest) in zip(line.split(" -> "), line.split(" -> ").skip(1)) {
            let src = src.split_once(',').context("input not '<int>,<int>'")?;
            let src: (i32, i32) = (
                src.0.parse().context("input not <int>")?,
                src.1.parse().context("input not <int>")?,
            );
            let dest = dest.split_once(',').context("input not '<int>,<int>'")?;
            let dest: (i32, i32) = (
                dest.0.parse().context("input not <int>")?,
                dest.1.parse().context("input not <int>")?,
            );
            if src.0 != dest.0 {
                ensure!(src.1 == dest.1);
                for x in src.0.min(dest.0)..=src.0.max(dest.0) {
                    grid.insert((x, src.1));
                }
            } else if src.1 != dest.1 {
                ensure!(src.0 == dest.0);
                for y in src.1.min(dest.1)..=src.1.max(dest.1) {
                    grid.insert((src.0, y));
                }
            }
        }
    }
    Ok(grid)
}

fn max_y(set: &HashSet<XY>) -> Option<i32> {
    set.iter().map(|(_, y)| y).max().copied()
}

fn part_one(input: &str) -> Result<usize> {
    let mut grid = parse(input)?;
    let max = max_y(&grid).context("empty input")?;

    let mut count = 0;
    loop {
        let mut current = (500, 0);
        count += 1;
        loop {
            if current.1 > max {
                return Ok(count - 1);
            }

            if !grid.contains(&(current.0, current.1 + 1)) {
                current = (current.0, current.1 + 1);
            } else if !grid.contains(&(current.0 - 1, current.1 + 1)) {
                current = (current.0 - 1, current.1 + 1);
            } else if !grid.contains(&(current.0 + 1, current.1 + 1)) {
                current = (current.0 + 1, current.1 + 1);
            } else {
                grid.insert(current);
                break;
            }
        }
    }
}

fn part_two(input: &str) -> Result<usize> {
    let mut grid = parse(input)?;
    let max = max_y(&grid).context("empty input")? + 1;

    let mut count = 0;
    loop {
        let mut current = (500, 0);
        count += 1;
        if grid.contains(&current) {
            return Ok(count - 1);
        }

        loop {
            if current.1 >= max {
                grid.insert(current);
                break;
            }

            if !grid.contains(&(current.0, current.1 + 1)) {
                current = (current.0, current.1 + 1);
            } else if !grid.contains(&(current.0 - 1, current.1 + 1)) {
                current = (current.0 - 1, current.1 + 1);
            } else if !grid.contains(&(current.0 + 1, current.1 + 1)) {
                current = (current.0 + 1, current.1 + 1);
            } else {
                grid.insert(current);
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 24);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 93);
    }
}
