use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    let answer = part_one(input).expect("no solution for part one");
    println!("part 1: {}", answer);

    let answer = part_two(input).expect("no solution for part two");
    println!("part 2: {}", answer);
}

#[derive(Debug, PartialEq)]
enum Error {
    BadInput,
}

struct DeterministicDie {
    next_value: usize,
    times_rolled: usize,
}

impl DeterministicDie {
    fn new() -> Self {
        DeterministicDie {
            next_value: 1,
            times_rolled: 0,
        }
    }

    fn times_rolled(&self) -> usize {
        self.times_rolled
    }
}

impl Iterator for DeterministicDie {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.next_value;
        self.next_value = (self.next_value % 10) + 1;
        self.times_rolled += 1;
        Some(value)
    }
}

fn parse_input(input: &str) -> Result<[usize; 2], Error> {
    fn parse_line(line: Option<&str>) -> Result<usize, Error> {
        if let Some(line) = line {
            if line.len() < 28 {
                return Err(Error::BadInput);
            }
            (&line[28..]).parse::<usize>().map_err(|_| Error::BadInput)
        } else {
            Err(Error::BadInput)
        }
    }
    let mut lines = input.lines();
    let a = parse_line(lines.next())?;
    let b = parse_line(lines.next())?;
    Ok([a, b])
}

fn part_one(input: &str) -> Result<usize, Error> {
    let mut die = DeterministicDie::new();
    let mut player = 0;
    let mut position = parse_input(input)?;
    let mut score = [0, 0];
    loop {
        let other_player = (player + 1) % 2;
        let dots = die.next().unwrap() + die.next().unwrap() + die.next().unwrap();
        position[player] = ((position[player] + dots - 1) % 10) + 1;
        score[player] += position[player];
        if score[player] >= 1000 {
            return Ok(score[other_player] * die.times_rolled());
        }
        player = other_player;
    }
}

type Cache = HashMap<(usize, usize, [usize; 2], [usize; 2], usize), [usize; 2]>;

// The 3^3 possible outcomes of three consecutive Dirac dice rolls, and their sums:
// 1, 1, 1 = 3
// 1, 1, 2 = 4
// 1, 1, 3 = 5
// 1, 2, 1 = 4
// 1, 2, 2 = 5
// ...
// 3, 3, 2 = 8
// 3, 3, 3 = 9
// The sums and their respective frequencies:
const DOTS_FREQUENCY: [(usize, usize); 7] =
    [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

fn part_two(input: &str) -> Result<usize, Error> {
    let position = parse_input(input)?;
    let score = [0, 0];
    let mut cache: Cache = HashMap::new();

    fn play_one_round(
        player: usize,
        dots: usize,
        mut position: [usize; 2],
        mut score: [usize; 2],
        depth: usize,
        cache: &mut Cache,
    ) -> [usize; 2] {
        debug_assert!(player <= 1);

        let key = (player, dots, position, score, depth);
        if let Some(&wins) = cache.get(&key) {
            return wins;
        }

        position[player] = ((position[player] + dots - 1) % 10) + 1;
        score[player] += position[player];
        if score[player] >= 21 {
            let mut wins = [0; 2];
            wins[player] += 1;
            cache.insert(key, wins);
            return wins;
        }

        let mut wins = [0; 2];
        let other_player = (player + 1) % 2;
        for (dots, freq) in DOTS_FREQUENCY {
            let sub = play_one_round(other_player, dots, position, score, depth + 1, cache);
            wins[0] += freq * sub[0];
            wins[1] += freq * sub[1];
        }
        cache.insert(key, wins);
        wins
    }

    let mut wins = [0; 2];
    for (dots, freq) in DOTS_FREQUENCY {
        let sub = play_one_round(0, dots, position, score, 0, &mut cache);
        wins[0] += freq * sub[0];
        wins[1] += freq * sub[1];
    }
    Ok(*wins.iter().max().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_deterministic_die() {
        let mut die = DeterministicDie::new();
        assert_eq!(die.times_rolled(), 0);
        assert_eq!(die.next(), Some(1));
        assert_eq!(die.times_rolled(), 1);
        assert_eq!(
            die.take(11).collect::<Vec<_>>(),
            vec![2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2]
        );
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), Ok(745 * 993));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), Ok(444_356_092_776_315));
    }
}
