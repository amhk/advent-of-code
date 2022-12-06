use anyhow::{Context, Result};
use std::collections::BTreeSet;

fn main() -> Result<()> {
    let input = include_str!("input.txt");

    let answer = part_one(input).context("no solution for part one")?;
    println!("Part 1: {}", answer);

    let answer = part_two(input).context("no solution for part two")?;
    println!("Part 2: {}", answer);

    Ok(())
}

fn scan<const N: usize>(input: &str) -> Option<usize> {
    let mut buffer = Vec::new();
    for (i, ch) in input.chars().enumerate() {
        if buffer.len() >= N {
            buffer.remove(0);
        }
        buffer.push(ch);
        if buffer.len() == N {
            let set = BTreeSet::from_iter(buffer.iter());
            if set.len() == N {
                return Some(i + 1);
            }
        }
    }
    None
}

fn part_one(input: &str) -> Option<usize> {
    scan::<4>(input)
}

fn part_two(input: &str) -> Option<usize> {
    scan::<14>(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("mjqjpqmgbljsphdztnvjfqwrcgsmlb").unwrap(), 7);
        assert_eq!(part_one("bvwbjplbgvbhsrlpgdmjqwftvncz").unwrap(), 5);
        assert_eq!(part_one("nppdvjthqldpwncqszvftbrmjlhg").unwrap(), 6);
        assert_eq!(part_one("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").unwrap(), 10);
        assert_eq!(part_one("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").unwrap(), 11);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("mjqjpqmgbljsphdztnvjfqwrcgsmlb").unwrap(), 19);
        assert_eq!(part_two("bvwbjplbgvbhsrlpgdmjqwftvncz").unwrap(), 23);
        assert_eq!(part_two("nppdvjthqldpwncqszvftbrmjlhg").unwrap(), 23);
        assert_eq!(part_two("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").unwrap(), 29);
        assert_eq!(part_two("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").unwrap(), 26);
    }
}
