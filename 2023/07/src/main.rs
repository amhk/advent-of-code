use anyhow::{ensure, Result};
use std::{cmp::Ordering, collections::BTreeMap};

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 248113761)?;
    aoc::run!(part_two(input), 246285222)?;
    Ok(())
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum CardType {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl TryFrom<char> for CardType {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            'X' => Ok(CardType::Joker),
            '2' => Ok(CardType::Two),
            '3' => Ok(CardType::Three),
            '4' => Ok(CardType::Four),
            '5' => Ok(CardType::Five),
            '6' => Ok(CardType::Six),
            '7' => Ok(CardType::Seven),
            '8' => Ok(CardType::Eight),
            '9' => Ok(CardType::Nine),
            'T' => Ok(CardType::Ten),
            'J' => Ok(CardType::Jack),
            'Q' => Ok(CardType::Queen),
            'K' => Ok(CardType::King),
            'A' => Ok(CardType::Ace),
            _ => Err(anyhow::anyhow!("invalid value {}", value)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind,  // AAAAA
    FourOfAKind,  // AAAA.
    FullHouse,    // AAABB
    ThreeOfAKind, // AAA..
    TwoPair,      // AABB.
    OnePair,      // AA...
    HighCard,     // A....
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: Vec<CardType>,
    bet: u64,
}

impl TryFrom<&str> for Hand {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        ensure!(value.len() >= 7);
        let cards: Result<Vec<CardType>> = value[0..5].chars().map(|ch| ch.try_into()).collect();
        let bet: u64 = value[6..].parse()?;
        Ok(Hand { cards: cards?, bet })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let this_type = self.hand_type();
        let other_type = other.hand_type();
        if this_type != other_type {
            return other_type.cmp(&this_type);
        }
        self.cards.cmp(&other.cards)
    }
}

impl Hand {
    fn hand_type(&self) -> HandType {
        debug_assert!(self.cards.len() == 5);
        let mut map: BTreeMap<CardType, usize> = BTreeMap::new();
        for t in self.cards.iter().filter(|&&c| c != CardType::Joker) {
            *map.entry(*t).or_default() += 1;
        }
        let jokers = self.cards.iter().filter(|&&c| c == CardType::Joker).count();
        let mut distribution: Vec<usize> = map.into_values().collect();
        debug_assert_eq!(distribution.iter().sum::<usize>() + jokers, 5);
        distribution.sort_by(|a, b| b.partial_cmp(a).unwrap());
        let first = distribution.first().unwrap_or(&0);
        let other = distribution.get(1);
        match jokers + first {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                if other == Some(&2) {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            2 => {
                if other == Some(&2) {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            }
            1 => HandType::HighCard,
            _ => panic!(),
        }
    }
}

fn parse(input: &str) -> Result<Vec<Hand>> {
    input
        .lines()
        .map(|line| line.try_into())
        .collect::<Result<Vec<_>>>()
}

fn solve(input: &str) -> Result<u64> {
    let mut hands = parse(input)?;
    hands.sort();
    let mut sum = 0;
    for (i, hand) in hands.into_iter().enumerate() {
        let i = (i + 1) as u64;
        sum += i * hand.bet;
    }
    Ok(sum)
}

fn part_one(input: &str) -> Result<u64> {
    solve(input)
}

fn part_two(input: &str) -> Result<u64> {
    solve(&input.replace('J', "X"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_hand_type() {
        macro_rules! assert_hand_type {
            ($cards:expr, $expected:expr) => {
                assert_eq!(
                    Hand::try_from(format!("{} 0", $cards).as_str())
                        .unwrap()
                        .hand_type(),
                    $expected,
                    "cards={:?}",
                    $cards
                );
            };
        }

        assert_hand_type!("AAAAA", HandType::FiveOfAKind);
        assert_hand_type!("AAAA2", HandType::FourOfAKind);
        assert_hand_type!("AAA22", HandType::FullHouse);
        assert_hand_type!("AAA23", HandType::ThreeOfAKind);
        assert_hand_type!("AA223", HandType::TwoPair);
        assert_hand_type!("AA234", HandType::OnePair);
        assert_hand_type!("A2345", HandType::HighCard);

        // jokers
        assert_hand_type!("AAXAA", HandType::FiveOfAKind);
        assert_hand_type!("AAXXX", HandType::FiveOfAKind);
        assert_hand_type!("AAAX2", HandType::FourOfAKind);
        assert_hand_type!("AAA2X", HandType::FourOfAKind);
        assert_hand_type!("XXX23", HandType::FourOfAKind);
        assert_hand_type!("AAX23", HandType::ThreeOfAKind);
        assert_hand_type!("AXA22", HandType::FullHouse);
        assert_hand_type!("AXX23", HandType::ThreeOfAKind);
        assert_hand_type!("XA223", HandType::ThreeOfAKind);
        assert_hand_type!("AAX23", HandType::ThreeOfAKind);
        // not possible to create TwoPair with a joker: the hand will always evaluate to something
        // better, like ThreeOfAKind
        assert_hand_type!("AX234", HandType::OnePair);
        assert_hand_type!("X2345", HandType::OnePair);
        // not possible to create HighCard with a joker: the hand will always evaluate to something
        // better, like OnePair
    }

    #[test]
    fn test_hand_cmp() {
        assert!(Hand::try_from("AAAAA 0").unwrap() > Hand::try_from("AAAA2 0").unwrap());
        assert!(Hand::try_from("22344 0").unwrap() < Hand::try_from("AAKK2 0").unwrap());
        assert!(Hand::try_from("22222 0").unwrap() > Hand::try_from("XXXXX 0").unwrap());
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 6440);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 5905);
    }
}
