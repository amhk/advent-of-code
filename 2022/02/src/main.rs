use anyhow::{Context, Result};
use std::collections::HashMap;

fn main() -> Result<()> {
    let input = include_str!("input.txt");

    let answer = part_one(input).context("no solution for part one")?;
    println!("Part 1: {}", answer);

    let answer = part_two(input).context("no solution for part two")?;
    println!("Part 2: {}", answer);

    Ok(())
}

fn sum(input: &str, patterns: &HashMap<&str, usize>) -> Result<usize> {
    let mut sum = 0;
    for line in input.lines() {
        sum += patterns
            .get(line)
            .context(format!("unexpected input '{}'", line))?;
    }
    Ok(sum)
}

fn part_one(input: &str) -> Result<usize> {
    #[allow(clippy::identity_op)] // prefer "1 + 2" instead of "3"
    sum(
        input,
        &HashMap::from([
            ("A X", 1 + 3),
            ("A Y", 2 + 6),
            ("A Z", 3 + 0),
            ("B X", 1 + 0),
            ("B Y", 2 + 3),
            ("B Z", 3 + 6),
            ("C X", 1 + 6),
            ("C Y", 2 + 0),
            ("C Z", 3 + 3),
        ]),
    )
}

fn part_two(input: &str) -> Result<usize> {
    #[allow(clippy::identity_op)] // prefer "1 + 2" instead of "3"
    sum(
        input,
        &HashMap::from([
            ("A X", 3 + 0),
            ("A Y", 1 + 3),
            ("A Z", 2 + 6),
            ("B X", 1 + 0),
            ("B Y", 2 + 3),
            ("B Z", 3 + 6),
            ("C X", 2 + 0),
            ("C Y", 3 + 3),
            ("C Z", 1 + 6),
        ]),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 15);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 12);
    }
}
