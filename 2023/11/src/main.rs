use anyhow::{bail, Result};
use itertools::Itertools;
use std::collections::BTreeSet;

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(solve(input, 2), 9521776)?;
    aoc::run!(solve(input, 1_000_000), 553224415344)?;
    Ok(())
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
struct XY {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for XY {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

fn parse(input: &str) -> Result<BTreeSet<XY>> {
    let mut galaxies = BTreeSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '.' => {}
                '#' => {
                    let xy: XY = (x, y).into();
                    galaxies.insert(xy);
                }
                _ => bail!("unexpected char {ch}"),
            }
        }
    }
    Ok(galaxies)
}

fn expand(galaxies: impl Iterator<Item = XY>, step_size: usize) -> BTreeSet<XY> {
    let galaxies: Vec<_> = galaxies.collect();

    let min_x = galaxies.iter().map(|xy| xy.x).min().unwrap();
    let max_x = galaxies.iter().map(|xy| xy.x).max().unwrap();
    let mut empty_columns = vec![];
    for x in min_x..=max_x {
        if galaxies.iter().all(|xy| xy.x != x) {
            empty_columns.push(x);
        }
    }

    let min_y = galaxies.iter().map(|xy| xy.y).min().unwrap();
    let max_y = galaxies.iter().map(|xy| xy.y).max().unwrap();
    let mut empty_rows = vec![];
    for y in min_y..=max_y {
        if galaxies.iter().all(|xy| xy.y != y) {
            empty_rows.push(y);
        }
    }

    let mut out: BTreeSet<XY> = BTreeSet::new();
    for xy in galaxies.iter() {
        out.insert(
            (
                xy.x + empty_columns.iter().filter(|&&x| x < xy.x).count() * step_size,
                xy.y + empty_rows.iter().filter(|&&y| y < xy.y).count() * step_size,
            )
                .into(),
        );
    }
    assert!(galaxies.len() == out.len());
    out
}

fn solve(input: &str, expand_times: usize) -> Result<usize> {
    let galaxies = parse(input)?;
    let galaxies = expand(galaxies.into_iter(), expand_times - 1);
    let mut sum = 0;
    for xys in galaxies.into_iter().combinations(2) {
        let src = xys[0];
        let dest = xys[1];
        sum += src.x.abs_diff(dest.x);
        sum += src.y.abs_diff(dest.y);
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(solve(INPUT, 2).unwrap(), 374);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(solve(INPUT, 10).unwrap(), 1030);
        assert_eq!(solve(INPUT, 100).unwrap(), 8410);
    }
}
