use anyhow::{bail, ensure, Context, Result};
use itertools::Itertools;
use std::collections::BTreeSet;

fn main() -> Result<()> {
    let input = include_str!("input.txt");

    let answer = part_one(input).context("no solution for part one")?;
    println!("Part 1: {}", answer);

    let answer = part_two(input).context("no solution for part two")?;
    println!("Part 2: {}", answer);

    Ok(())
}

fn letters_to_int(s: &str) -> Result<Vec<usize>> {
    let mut ints = vec![];
    for ch in s.chars() {
        // 'a'-'z': 1-26
        // 'A'-'Z': 27-52
        let int = match ch {
            'a'..='z' => ch as usize - 'a' as usize + 1,
            'A'..='Z' => ch as usize - 'A' as usize + 27,
            _ => bail!("unexpected input '{}'", ch),
        };
        ints.push(int);
    }
    Ok(ints)
}

fn part_one(input: &str) -> Result<usize> {
    let mut sum = 0;
    for line in input.lines() {
        let integers = letters_to_int(line)?;
        let pivot = integers.len() / 2;
        let left = BTreeSet::from_iter(&integers[0..pivot]);
        let right = BTreeSet::from_iter(&integers[pivot..]);
        sum += left.intersection(&right).copied().sum::<usize>();
    }
    Ok(sum)
}

fn part_two(input: &str) -> Result<usize> {
    ensure!(
        input.lines().count() % 3 == 0,
        "number of input lines must be a multiple of 3"
    );
    let mut sum = 0;
    for group in &input.lines().chunks(3) {
        let group = group.collect::<Vec<_>>();
        let a = BTreeSet::from_iter(letters_to_int(group[0])?);
        let b = BTreeSet::from_iter(letters_to_int(group[1])?);
        let c = BTreeSet::from_iter(letters_to_int(group[2])?);
        let x = BTreeSet::from_iter(a.intersection(&b).copied());
        sum += c.intersection(&x).copied().sum::<usize>();
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 157);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 70);
    }
}
