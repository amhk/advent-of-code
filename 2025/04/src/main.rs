use std::collections::HashSet;

use anyhow::Result;
use aoc::{parse_grid, XY};

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 1437)?;
    aoc::run!(part_two(input), 8765)?;
    Ok(())
}

fn parse(input: &str) -> Result<HashSet<XY>> {
    let mut objects = HashSet::new();
    let _ = parse_grid(input, |xy, ch| {
        if ch == '@' {
            objects.insert(xy);
        }
        Ok(())
    });
    Ok(objects)
}

fn part_one(input: &str) -> Result<usize> {
    let objects = parse(input)?;
    let count = objects
        .iter()
        .filter(|xy| {
            xy.eight_neighbours()
                .into_iter()
                .filter_map(|n| objects.get(&n))
                .count()
                < 4
        })
        .count();
    Ok(count)
}

fn part_two(input: &str) -> Result<usize> {
    let mut objects = parse(input)?;
    let mut count = 0;
    loop {
        let to_be_removed: HashSet<_> = objects
            .iter()
            .filter(|xy| {
                xy.eight_neighbours()
                    .into_iter()
                    .filter_map(|n| objects.get(&n))
                    .count()
                    < 4
            })
            .cloned()
            .collect();
        if to_be_removed.is_empty() {
            break;
        }
        count += to_be_removed.len();
        objects = objects.difference(&to_be_removed).cloned().collect();
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 13);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 43);
    }
}
