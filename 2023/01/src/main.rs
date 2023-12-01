use anyhow::{anyhow, Result};
use std::collections::BTreeMap;

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 53974)?;
    aoc::run!(part_two(input), 52840)?;
    Ok(())
}

fn parse(input: &str) -> Result<Vec<i32>> {
    let mut v: Vec<i32> = vec![];
    for line in input.lines() {
        let first = line
            .chars()
            .find(|x| x.is_ascii_digit())
            .ok_or(anyhow!("bad input: no digits: {}", line))?;
        let last = line
            .chars()
            .rev()
            .find(|x| x.is_ascii_digit())
            .ok_or(anyhow!("bad input: no digits: {}", line))?;
        let s = format!("{}{}", first, last);
        v.push(s.parse()?);
    }
    Ok(v)
}

// overlap is allowed, i.e. "anineightb" simplifies to "a98b"
fn simplify(input: &str) -> String {
    let mapping = BTreeMap::from([
        // note: according to the puzzle, "zero" is not a valid pattern
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    let mut v: Vec<char> = vec![];
    for i in 0..input.len() {
        let substr = &input[i..];
        if let Some((_, replacement)) = mapping
            .iter()
            .find(|(pattern, _)| substr.starts_with(**pattern))
        {
            v.push(*replacement);
        } else {
            v.push(substr.chars().next().unwrap());
        }
    }
    v.into_iter().collect::<String>()
}

fn part_one(input: &str) -> Result<i32> {
    Ok(parse(input)?.into_iter().sum())
}

fn part_two(input: &str) -> Result<i32> {
    let input = simplify(input);
    Ok(parse(&input)?.into_iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_PART_ONE: &str = include_str!("test-input-part-one.txt");
    const INPUT_PART_TWO: &str = include_str!("test-input-part-two.txt");

    #[test]
    fn test_parse() {
        assert_eq!(parse(INPUT_PART_ONE).unwrap(), vec![12, 38, 15, 77]);
    }

    #[test]
    fn test_simplify() {
        assert_eq!(
            simplify(INPUT_PART_TWO).lines().collect::<Vec<_>>(),
            vec![
                "2wo19ine",
                "8igh2wo3hree",
                "abc1ne23hreexyz",
                "x2w1ne34our",
                "49ine8ight7even2",
                "z1n8ight234",
                "7pqrst6ixteen",
            ]
        );
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT_PART_ONE).unwrap(), 142);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT_PART_TWO).unwrap(), 281);
    }
}
