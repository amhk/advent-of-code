use anyhow::{ensure, Context, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 70764)?;
    aoc::run!(part_two(input), 203905)?;
    Ok(())
}

fn parse(input: &str) -> Vec<usize> {
    let mut groups = Vec::new();
    for (key, group) in &input
        .lines()
        .map(|item| item.parse::<usize>().unwrap_or_default())
        .group_by(|item| *item != 0)
    {
        if key {
            groups.push(group.into_iter().sum());
        }
    }
    groups
}

fn part_one(input: &str) -> Result<usize> {
    let groups = parse(input);
    groups.iter().max().context("empty input").copied()
}

fn part_two(input: &str) -> Result<usize> {
    let mut groups = parse(input);
    ensure!(groups.len() > 3, "less than three groups found");
    groups.sort_by(|a, b| b.cmp(a));
    Ok(groups.iter().take(3).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 24_000);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 45_000);
    }
}
