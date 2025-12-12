use std::{collections::HashSet, mem::swap};

use anyhow::{anyhow, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input, 1000), 133574)?;
    aoc::run!(part_two(input), 2435100380)?;
    Ok(())
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct XYZ {
    x: i64,
    y: i64,
    z: i64,
}

impl XYZ {
    // should return f32 but to make sorting easier (f32 does not implement Ord), cast to usize and
    // assume that throwing away the fractional part doesn't matter
    fn distance(&self, other: &XYZ) -> usize {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)) as f32)
            .sqrt() as usize
    }
}

impl From<(i64, i64, i64)> for XYZ {
    fn from(value: (i64, i64, i64)) -> Self {
        Self {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}

fn parse(input: &str) -> Result<Vec<XYZ>> {
    let mut out = vec![];
    for line in input.lines() {
        let mut iter = line.split(',');
        let x: i64 = iter.next().ok_or_else(|| anyhow!("bad input"))?.parse()?;
        let y: i64 = iter.next().ok_or_else(|| anyhow!("bad input"))?.parse()?;
        let z: i64 = iter.next().ok_or_else(|| anyhow!("bad input"))?.parse()?;
        out.push((x, y, z).into())
    }
    Ok(out)
}

fn connect_points(input: &str, max_connections: Option<usize>) -> Result<usize> {
    let points = parse(input)?;
    let mut distances = vec![];
    for pair in points.iter().combinations(2) {
        let a = pair[0].clone();
        let b = pair[1].clone();
        distances.push((a.distance(&b), a, b));
    }
    distances.sort_unstable_by_key(|(distance, _, _)| *distance);
    let mut circuits: Vec<_> = points.into_iter().map(|p| HashSet::from([p])).collect();
    let iter = if let Some(max) = max_connections {
        let take: Vec<_> = distances.into_iter().take(max).collect();
        take.into_iter()
    } else {
        distances.into_iter()
    };
    for (_, a, b) in iter {
        let mut pos_a = circuits
            .iter()
            .position(|set| set.contains(&a))
            .ok_or_else(|| anyhow!("failed to find set containing a"))?;
        let mut pos_b = circuits
            .iter()
            .position(|set| set.contains(&b))
            .ok_or_else(|| anyhow!("failed to find set containing b"))?;
        if pos_a != pos_b {
            if pos_a < pos_b {
                swap(&mut pos_a, &mut pos_b);
            }
            let cicruit_a = circuits.swap_remove(pos_a);
            circuits[pos_b].extend(cicruit_a.into_iter());
            if circuits.len() == 1 {
                return Ok(a.x as usize * b.x as usize);
            }
        }
    }

    let mut sizes: Vec<_> = circuits.iter().map(|set| set.len()).collect();
    sizes.sort_unstable();
    Ok(sizes.into_iter().rev().take(3).product())
}

fn part_one(input: &str, max_connections: usize) -> Result<usize> {
    connect_points(input, Some(max_connections))
}

fn part_two(input: &str) -> Result<usize> {
    connect_points(input, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT, 10).unwrap(), 40);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 25272);
    }
}
