use anyhow::{anyhow, Result};
use std::ops::RangeInclusive;

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 652)?;
    aoc::run!(part_two(input), 341753674214273)?;
    Ok(())
}

fn parse(input: &str) -> Result<(Vec<RangeInclusive<usize>>, Vec<usize>)> {
    let (first, second) = input
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("bad input"))?;

    let mut ranges: Vec<RangeInclusive<_>> = vec![];
    for line in first.lines() {
        let (lower, upper) = line.split_once("-").ok_or_else(|| anyhow!("bad input"))?;
        let lower = lower.parse::<usize>()?;
        let upper = upper.parse::<usize>()?;
        ranges.push(lower..=upper);
    }

    let ids: Vec<usize> = second
        .lines()
        .map(|line| line.parse::<usize>().map_err(|_| anyhow!("bad input")))
        .collect::<Result<_>>()?;

    Ok((ranges, ids))
}

fn part_one(input: &str) -> Result<usize> {
    let (ranges, ids) = parse(input)?;
    let mut count = 0;
    for id in ids {
        if ranges.iter().any(|range| range.contains(&id)) {
            count += 1;
        }
    }
    Ok(count)
}

fn part_two(input: &str) -> Result<usize> {
    let (original_ranges, _) = parse(input)?;
    let mut ranges: Vec<RangeInclusive<usize>> = vec![];
    for mut new in original_ranges.into_iter() {
        let mut new_ranges = vec![];
        for old in ranges.into_iter() {
            if old.end() < new.start() || new.end() < old.start() {
                // ranges are separate: add old and leave new unmodified
                new_ranges.push(old);
            } else if new.start() <= old.start() && new.end() >= old.end() {
                // new fully eclipses old: drop old and leave new unmodified
            } else if old.start() <= new.start() && old.end() >= new.end() {
                // old fully eclipses new: drop old and update new
                new = old;
            } else if old.start() < new.start() && old.end() <= new.end() {
                new = *old.start()..=*new.end();
            } else if new.start() < old.start() && new.end() <= old.end() {
                new = *new.start()..=*old.end();
            } else {
                unreachable!("old={:?} new={:?}", old, new);
            }
        }
        new_ranges.push(new);
        ranges = new_ranges;
    }
    Ok(ranges
        .into_iter()
        .map(|range| range.end() - range.start() + 1)
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 3);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 14);
    }
}
