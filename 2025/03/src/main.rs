use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 17443)?;
    aoc::run!(part_two(input), 172167155440541)?;
    Ok(())
}

fn parse(input: &str) -> Result<Vec<Vec<usize>>> {
    let mut out = vec![];
    for line in input.lines() {
        let bank: Result<Vec<_>> = line
            .chars()
            .map(|ch| {
                ch.to_digit(10)
                    .map(|int| int as usize)
                    .ok_or_else(|| anyhow!("bad input"))
            })
            .collect();
        out.push(bank?);
    }
    Ok(out)
}

fn find_largest(values: &[usize]) -> usize {
    let mut index = 0;
    for (i, value) in values.iter().enumerate() {
        if *value > values[index] {
            index = i;
            if *value == 9 {
                break;
            }
        }
    }
    index
}

fn part_x(input: &str, num_batteries: usize) -> Result<usize> {
    let mut sum = 0;
    for bank in parse(input)? {
        let mut joltage = 0;
        let mut index = 0;
        for n in 0..num_batteries {
            let largest_index = find_largest(&bank[index..bank.len() - num_batteries + n + 1]);
            if joltage != 0 {
                joltage *= 10;
            }
            joltage += bank[index + largest_index];
            index += largest_index + 1;
        }
        sum += joltage;
    }
    Ok(sum)
}

fn part_one(input: &str) -> Result<usize> {
    part_x(input, 2)
}

fn part_two(input: &str) -> Result<usize> {
    part_x(input, 12)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_find_largest() {
        assert_eq!(find_largest(&[1, 2, 3, 4, 5, 6, 7, 8, 9]), 8);
        assert_eq!(find_largest(&[4, 5, 3]), 1);
        assert_eq!(find_largest(&[4, 5, 6]), 2);
        assert_eq!(find_largest(&[8, 5, 6]), 0);
        assert_eq!(find_largest(&[8]), 0);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 357);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 3121910778619);
    }
}
