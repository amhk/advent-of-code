use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");

    let answer = part_one(&input).expect("no solution for part one");
    println!("part 1: {}", answer);

    let answer = part_two(&input).expect("no solution for part two");
    println!("part 2: {}", answer);
}

#[derive(Debug, PartialEq)]
enum Error {
    BadInput,
}

fn parse_input(input: &str) -> Result<(Vec<usize>, Vec<usize>), Error> {
    fn parse_deck(input: &str) -> Result<Vec<usize>, Error> {
        let mut deck = Vec::new();
        for line in input.lines().skip(1) {
            let value = line.parse::<usize>().map_err(|_| Error::BadInput)?;
            deck.push(value);
        }
        Ok(deck)
    }

    let parts = input.split("\n\n").collect::<Vec<_>>();
    if parts.len() != 2 {
        return Err(Error::BadInput);
    }
    Ok((parse_deck(parts[0])?, parse_deck(parts[1])?))
}

type Card = usize;

#[derive(Debug, PartialEq)]
enum Player {
    P1,
    P2,
}

fn play_normal_game(deck1: &[Card], deck2: &[Card]) -> (Player, Vec<Card>) {
    let mut deck1 = deck1.to_vec();
    let mut deck2 = deck2.to_vec();
    loop {
        if deck1.is_empty() {
            return (Player::P2, deck2);
        }
        if deck2.is_empty() {
            return (Player::P1, deck1);
        }

        let card1 = deck1.remove(0);
        let card2 = deck2.remove(0);
        assert_ne!(card1, card2);

        if card1 > card2 {
            deck1.push(card1);
            deck1.push(card2);
        } else {
            deck2.push(card2);
            deck2.push(card1);
        }
    }
}

fn play_recursive_game(deck1: &[Card], deck2: &[Card]) -> (Player, Vec<Card>) {
    let mut deck1 = deck1.to_vec();
    let mut deck2 = deck2.to_vec();
    let mut history: HashSet<(Vec<Card>, Vec<Card>)> = HashSet::new();
    loop {
        if deck1.is_empty() {
            return (Player::P2, deck2);
        }
        if deck2.is_empty() {
            return (Player::P1, deck1);
        }

        if !history.insert((deck1.to_vec(), deck2.to_vec())) {
            return (Player::P1, deck1);
        }

        let card1 = deck1.remove(0);
        let card2 = deck2.remove(0);
        assert_ne!(card1, card2);

        if deck1.len() >= card1 && deck2.len() >= card2 {
            let (winner, _) = play_recursive_game(&deck1[..card1], &deck2[..card2]);
            match winner {
                Player::P1 => {
                    deck1.push(card1);
                    deck1.push(card2);
                }
                Player::P2 => {
                    deck2.push(card2);
                    deck2.push(card1);
                }
            }
        } else if card1 > card2 {
            deck1.push(card1);
            deck1.push(card2);
        } else {
            deck2.push(card2);
            deck2.push(card1);
        }
    }
}

fn score(deck: &[usize]) -> usize {
    deck.iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, v)| acc + (i + 1) * v)
}

fn part_one(input: &str) -> Result<usize, Error> {
    let (deck1, deck2) = parse_input(input).unwrap();
    let (_, deck) = play_normal_game(&deck1, &deck2);
    Ok(score(&deck))
}

fn part_two(input: &str) -> Result<usize, Error> {
    let (deck1, deck2) = parse_input(input).unwrap();
    let (_, deck) = play_recursive_game(&deck1, &deck2);
    Ok(score(&deck))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_play_normal_game() {
        let (deck1, deck2) = parse_input(INPUT).unwrap();
        let (winner, deck) = play_normal_game(&deck1, &deck2);
        assert_eq!(winner, Player::P2);
        assert_eq!(score(&deck), 306);
    }

    #[test]
    fn test_play_recursive_game() {
        let (deck1, deck2) = parse_input(INPUT).unwrap();
        let (winner, deck) = play_recursive_game(&deck1, &deck2);
        assert_eq!(winner, Player::P2);
        assert_eq!(score(&deck), 291);
    }
}
