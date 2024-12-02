use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 224)?;
    aoc::run!(part_two(input), 293)?;
    Ok(())
}

fn parse(input: &str) -> Result<Vec<Vec<i32>>> {
    let mut out = vec![];
    for line in input.lines() {
        out.push(
            line.split_whitespace()
                .map(|w| w.parse::<i32>())
                .collect::<Result<_, _>>()?,
        );
    }
    Ok(out)
}

fn is_safe(numbers: &[i32]) -> bool {
    let safe_increasing = numbers
        .iter()
        .tuple_windows()
        .all(|(a, b)| [-1, -2, -3].contains(&(a - b)));
    let safe_decreasing = numbers
        .iter()
        .tuple_windows()
        .all(|(a, b)| [1, 2, 3].contains(&(a - b)));
    safe_increasing || safe_decreasing
}

fn part_one(input: &str) -> Result<usize> {
    let all_numbers = parse(input)?;
    let count = all_numbers
        .into_iter()
        .filter(|numbers| is_safe(numbers))
        .count();
    Ok(count)
}

fn part_two(input: &str) -> Result<usize> {
    let all_numbers = parse(input)?;
    let mut count = 0;
    'outer_loop: for numbers in all_numbers.into_iter() {
        if is_safe(&numbers) {
            count += 1;
            continue;
        }
        for i in 0..numbers.len() {
            let mut copy = numbers.clone();
            copy.remove(i);
            if is_safe(&copy) {
                count += 1;
                continue 'outer_loop;
            }
        }
    }
    Ok(count)
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
        assert_eq!(part_two(INPUT).unwrap(), 4)
    }
}
