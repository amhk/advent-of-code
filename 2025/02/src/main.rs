use std::ops::RangeInclusive;

use anyhow::{anyhow, ensure, Result};

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 38310256125)?;
    aoc::run!(part_two(input), 58961152806)?;
    Ok(())
}

fn parse(input: &str) -> Result<Vec<RangeInclusive<usize>>> {
    let mut out = vec![];
    for pair in input.split(",") {
        let pair = pair
            .trim()
            .split_once("-")
            .ok_or_else(|| anyhow!("bad input"))?;
        let lower = pair.0.parse::<usize>()?;
        let upper = pair.1.parse::<usize>()?;
        ensure!(lower < upper);
        out.push(lower..=upper);
    }
    Ok(out)
}

fn is_valid_id_part_1(id: usize) -> bool {
    let s = id.to_string();
    if s.len() % 2 == 1 {
        return true;
    }
    let a = &s[..s.len() / 2];
    let b = &s[s.len() / 2..];
    a != b
}

fn is_valid_id_part_2(id: usize) -> bool {
    let haystack = id.to_string();
    let haystack_len = haystack.len();
    for pattern_len in 1..=haystack_len / 2 {
        let pattern = &haystack[..pattern_len];
        let mut index = pattern_len;
        while index + pattern_len <= haystack_len {
            if &haystack[index..index + pattern_len] != pattern {
                break;
            }
            index += pattern_len;
        }
        if index >= haystack_len {
            return false;
        }
    }
    true
}

fn part_x<T: Fn(usize) -> bool>(input: &str, predicate: T) -> Result<usize> {
    let mut sum = 0;
    let ranges = parse(input)?;
    for range in ranges {
        for id in range {
            if !predicate(id) {
                sum += id;
            }
        }
    }
    Ok(sum)
}

fn part_one(input: &str) -> Result<usize> {
    part_x(input, is_valid_id_part_1)
}

fn part_two(input: &str) -> Result<usize> {
    part_x(input, is_valid_id_part_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_is_valid_id_part_1() {
        assert!(is_valid_id_part_1(0));
        assert!(!is_valid_id_part_1(11));
        assert!(!is_valid_id_part_1(22));
        assert!(is_valid_id_part_1(101));
    }

    #[test]
    fn test_is_valid_id_part_2() {
        assert!(is_valid_id_part_2(0));
        assert!(!is_valid_id_part_2(11));
        assert!(!is_valid_id_part_2(22));
        assert!(!is_valid_id_part_2(111));
        assert!(is_valid_id_part_2(11011));
        assert!(!is_valid_id_part_2(446446));
        assert!(is_valid_id_part_2(1188511884));
        assert!(!is_valid_id_part_2(1188511885));
        assert!(is_valid_id_part_2(1188511886));
        assert!(is_valid_id_part_2(2121212120));
        assert!(!is_valid_id_part_2(2121212121));
        assert!(is_valid_id_part_2(2121212122));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 1227775554);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 4174379265);
    }
}
