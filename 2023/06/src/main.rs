use anyhow::{ensure, Context, Result};
use regex::Regex;

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 440000)?;
    aoc::run!(part_two(input), 26187338)?;
    Ok(())
}

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

fn parse(input: &str) -> Result<Vec<Race>> {
    let (first, second) = input.split_once('\n').context("bad input")?;
    let re = Regex::new(r"\d+").unwrap();
    let times: Vec<usize> = re
        .find_iter(first)
        .map(|m| m.as_str().parse().unwrap())
        .collect();
    let distances: Vec<usize> = re
        .find_iter(second)
        .map(|m| m.as_str().parse().unwrap())
        .collect();
    ensure!(times.len() == distances.len());

    let races: Vec<_> = times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| Race { time, distance })
        .collect();
    Ok(races)
}

fn f(t: usize, total_t: usize) -> usize {
    debug_assert!(t <= total_t);
    t * (total_t - t)
}

fn number_of_winning_races(total_t: usize, distance_to_beat: usize) -> usize {
    let mut n = 0;
    for t in 0..=total_t {
        if f(t, total_t) > distance_to_beat {
            n += 1;
        }
    }
    n
}

fn solve(races: &[Race]) -> usize {
    let mut counts = vec![];
    for Race { time, distance } in races.iter() {
        counts.push(number_of_winning_races(*time, *distance));
    }
    counts.into_iter().product()
}

fn part_one(input: &str) -> Result<usize> {
    let races = parse(input)?;
    Ok(solve(&races))
}

fn part_two(input: &str) -> Result<usize> {
    let races = parse(&input.replace(' ', ""))?;
    Ok(solve(&races))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_f() {
        assert_eq!(f(0, 7), 0);
        assert_eq!(f(1, 7), 6);
        assert_eq!(f(2, 7), 10);
        assert_eq!(f(6, 7), 6);
        assert_eq!(f(7, 7), 0);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 288);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 71503);
    }
}
