use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use regex::Regex;

fn main() -> Result<()> {
    let input = include_str!("input.txt");

    let answer = part_one(input).context("no solution for part one")?;
    println!("Part 1: {}", answer);

    let answer = part_two(input).context("no solution for part two")?;
    println!("Part 2: {}", answer);

    Ok(())
}

fn parse(line: &str) -> Result<((usize, usize), (usize, usize))> {
    static REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap());
    let caps = REGEX
        .captures(line)
        .with_context(|| format!("unexpected input '{}'", line))?;
    let b1 = caps.get(1).unwrap().as_str().parse().unwrap();
    let e1 = caps.get(2).unwrap().as_str().parse().unwrap();
    let b2 = caps.get(3).unwrap().as_str().parse().unwrap();
    let e2 = caps.get(4).unwrap().as_str().parse().unwrap();
    Ok(((b1, e1), (b2, e2)))
}

fn count<F>(input: &str, predicate: F) -> Result<usize>
where
    F: Fn((usize, usize), (usize, usize)) -> bool,
{
    let mut count = 0;
    for line in input.lines() {
        let ((b1, e1), (b2, e2)) = parse(line)?;
        if predicate((b1, e1), (b2, e2)) {
            count += 1;
        }
    }
    Ok(count)
}

fn part_one(input: &str) -> Result<usize> {
    count(input, |(b1, e1), (b2, e2)| {
        (b1 <= b2 && e2 <= e1) || (b2 <= b1 && e1 <= e2)
    })
}

fn part_two(input: &str) -> Result<usize> {
    count(input, |(b1, e1), (b2, e2)| {
        (b1 <= b2 && e1 >= b2) || (b1 <= e2 && e1 >= e2) || (b1 >= b2 && e1 <= e2)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 2);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 4);
    }
}
