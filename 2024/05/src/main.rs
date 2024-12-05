use anyhow::{anyhow, ensure, Result};
use std::collections::{BTreeMap, BTreeSet};

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 4774)?;
    aoc::run!(part_two(input), 6004)?;
    Ok(())
}

type Rules = BTreeMap<usize, BTreeSet<usize>>;

fn parse(input: &str) -> Result<(Rules, Vec<Vec<usize>>)> {
    let (first, second) = input
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("missing blank line"))?;

    let mut rules = Rules::new();
    for line in first.lines() {
        let (a, b) = line
            .split_once('|')
            .ok_or_else(|| anyhow!("missing rule separator"))?;
        let a = a.parse::<usize>()?;
        let b = b.parse::<usize>()?;
        rules.entry(a).or_default().insert(b);
    }

    let mut all_numbers = vec![];
    for line in second.lines() {
        let numbers = line
            .split(',')
            .map(|s| s.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?;
        ensure!(numbers.len() % 2 == 1);
        all_numbers.push(numbers);
    }

    Ok((rules, all_numbers))
}

fn is_safe(numbers: &[usize], rules: &Rules) -> bool {
    let mut seen = BTreeSet::new();
    for n in numbers {
        if let Some(r) = rules.get(n) {
            if !r.is_disjoint(&seen) {
                return false;
            }
        }
        seen.insert(*n);
    }
    true
}

fn reorder_to_safe(numbers: &[usize], rules: &Rules) -> Result<Vec<usize>> {
    let mut safe = vec![];
    for n in numbers {
        let mut insert_index = 0;
        for (index, m) in safe.iter().enumerate() {
            if let Some(r) = rules.get(m) {
                if r.contains(n) {
                    insert_index = index + 1;
                }
            }
        }
        safe.insert(insert_index, *n);
    }
    ensure!(is_safe(&safe, rules));
    Ok(safe)
}

fn part_one(input: &str) -> Result<usize> {
    let (rules, all_numbers) = parse(input)?;
    let mut count = 0;
    for numbers in all_numbers.into_iter() {
        if is_safe(&numbers, &rules) {
            count += numbers[numbers.len() / 2];
        }
    }
    Ok(count)
}

fn part_two(input: &str) -> Result<usize> {
    let (rules, all_numbers) = parse(input)?;
    let mut count = 0;
    for numbers in all_numbers.into_iter() {
        if !is_safe(&numbers, &rules) {
            let reordered = reorder_to_safe(&numbers, &rules)?;
            count += reordered[numbers.len() / 2];
        }
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_is_safe() {
        let (rules, numbers) = parse(INPUT).unwrap();
        assert!(is_safe(&[], &rules));
        assert!(is_safe(&numbers[0], &rules));
        assert!(is_safe(&numbers[1], &rules));
        assert!(is_safe(&numbers[2], &rules));
        assert!(!is_safe(&numbers[3], &rules));
        assert!(!is_safe(&numbers[4], &rules));
        assert!(!is_safe(&numbers[5], &rules));
    }

    #[test]
    #[rustfmt::skip]
    fn test_reorder_to_safe() {
        let (rules, _) = parse(INPUT).unwrap();
        assert_eq!(reorder_to_safe(&[], &rules).unwrap(), []);
        assert_eq!(reorder_to_safe(&[75, 97, 47, 61, 53], &rules).unwrap(), [97, 75, 47, 61, 53]);
        assert_eq!(reorder_to_safe(&[61, 13, 29], &rules).unwrap(), [61, 29, 13]);
        assert_eq!(reorder_to_safe(&[97, 13, 75, 29, 47], &rules).unwrap(), [97, 75, 47, 29, 13]);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 143);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 123);
    }
}
