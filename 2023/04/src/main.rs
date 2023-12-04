use anyhow::Result;
use regex::Regex;
use std::collections::BTreeSet;

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 24160)?;
    aoc::run!(part_two(input), 5659035)?;
    Ok(())
}

#[derive(Debug)]
struct Card {
    winning: BTreeSet<u32>,
    actual: BTreeSet<u32>,
}

fn parse(input: &str) -> Result<Vec<Card>> {
    let re_card = Regex::new(r"Card\s+\d+:([\s\d]+)\|([\s\d]+)").unwrap();
    let re_numbers = Regex::new(r"\d+").unwrap();
    let mut cards = vec![];
    for line in input.lines() {
        let caps = re_card
            .captures(line)
            .ok_or_else(|| anyhow::anyhow!("bad input"))?;
        let winning: BTreeSet<u32> = re_numbers
            .find_iter(caps.get(1).unwrap().as_str())
            .map(|x| x.as_str().parse::<u32>().unwrap())
            .collect();
        let actual: BTreeSet<u32> = re_numbers
            .find_iter(caps.get(2).unwrap().as_str())
            .map(|x| x.as_str().parse::<u32>().unwrap())
            .collect();
        cards.push(Card { winning, actual });
    }
    Ok(cards)
}

fn part_one(input: &str) -> Result<u32> {
    let cards = parse(input)?;
    let mut sum = 0;
    for card in cards.into_iter() {
        let count = card.winning.intersection(&card.actual).count() as u32;
        let score: u32 = if count == 0 { 0 } else { 2u32.pow(count - 1) };
        sum += score;
    }
    Ok(sum)
}

fn part_two(input: &str) -> Result<usize> {
    let cards = parse(input)?;
    let mut count = vec![1; cards.len()];
    for (i, card) in cards.into_iter().enumerate() {
        let num_wins = card.winning.intersection(&card.actual).count();
        for j in 1 + i..1 + i + num_wins {
            // according to the puzzle description, count[j] is guaranteed to never index out of
            // bounds
            count[j] += count[i];
        }
    }
    Ok(count.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 13);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 30);
    }
}
