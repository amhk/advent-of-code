use std::collections::HashMap;

use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 190865)?;
    aoc::run!(part_two(input), 225404711855335)?;
    Ok(())
}

fn parse(input: &str) -> Result<Vec<usize>> {
    Ok(input
        .split_whitespace()
        .map(|w| w.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?)
}

fn maybe_split(n: usize) -> Option<(usize, usize)> {
    let mut digits = 0;
    let mut denominator = 1;
    while n / denominator > 0 {
        digits += 1;
        denominator *= 10;
    }
    if digits % 2 == 1 {
        return None;
    }
    let mut denominator = 1;
    for _ in 0..digits / 2 {
        denominator *= 10;
    }
    Some((n / denominator, n % denominator))
}

fn process(numbers: &[usize], depth: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    if depth == 0 {
        return numbers.len();
    }
    let mut total_count = 0;
    for n in numbers {
        if let Some(count) = cache.get(&(*n, depth)) {
            total_count += count;
        } else {
            let count = if *n == 0 {
                process(&[1], depth - 1, cache)
            } else if let Some((a, b)) = maybe_split(*n) {
                process(&[a], depth - 1, cache) + process(&[b], depth - 1, cache)
            } else {
                process(&[n * 2024], depth - 1, cache)
            };
            cache.insert((*n, depth), count);
            total_count += count;
        }
    }
    total_count
}

fn part_one(input: &str) -> Result<usize> {
    let numbers = parse(input)?;
    Ok(process(&numbers, 25, &mut HashMap::new()))
}

fn part_two(input: &str) -> Result<usize> {
    let numbers = parse(input)?;
    Ok(process(&numbers, 75, &mut HashMap::new()))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_maybe_split() {
        assert_eq!(maybe_split(1), None);
        assert_eq!(maybe_split(12), Some((1, 2)));
        assert_eq!(maybe_split(123), None);
        assert_eq!(maybe_split(1230), Some((12, 30)));
        assert_eq!(maybe_split(12300), None);
        assert_eq!(maybe_split(123000), Some((123, 0)));
    }

    #[test]
    fn test_process() {
        assert_eq!(process(&[0], 0, &mut HashMap::new()), [0].len());
        assert_eq!(process(&[0], 1, &mut HashMap::new()), [1].len());
        assert_eq!(process(&[125], 1, &mut HashMap::new()), [253000].len());
        assert_eq!(process(&[17], 1, &mut HashMap::new()), [1, 7].len());
        assert_eq!(process(&[125], 2, &mut HashMap::new()), [253, 0].len());
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 55312);
    }
}
