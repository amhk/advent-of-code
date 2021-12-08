use std::collections::{HashMap, HashSet};

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
    Segment,
    Number,
}

fn part_one(input: &str) -> Result<u32, Error> {
    let mut sum = 0;
    for line in input.lines() {
        for segments in line
            .split('|')
            .nth(1)
            .ok_or(Error::BadInput)?
            .split_whitespace()
        {
            match segments.len() {
                2 | 3 | 4 | 7 => sum += 1,
                _ => {}
            }
        }
    }
    Ok(sum)
}

// An enum to represent the segments. Separate type from signals (chars) to avoid confusing the
// two.
//
//    AAAA
//   B    C
//   B    C
//    DDDD
//   E    F
//   E    F
//    GGGG
#[derive(Debug, PartialEq, Eq, Clone, Copy, Ord, PartialOrd)]
enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

fn map_signals_to_segments(input: &str) -> Result<HashMap<char, Segment>, Error> {
    let signals: Vec<_> = input
        .split('|')
        .next()
        .ok_or(Error::BadInput)?
        .split_whitespace()
        .collect();
    let mut map = HashMap::<char, Segment>::new();
    let mut freq7 = vec![];
    let mut freq8 = vec![];

    // fix B, E, F based on unique frequencies
    for ch in ['a', 'b', 'c', 'd', 'e', 'f', 'g'] {
        match signals.join("").chars().filter(|ch_| ch == *ch_).count() {
            4 => {
                map.insert(ch, Segment::E);
            }
            6 => {
                map.insert(ch, Segment::B);
            }
            7 => freq7.push(ch),
            8 => freq8.push(ch),
            9 => {
                map.insert(ch, Segment::F);
            }
            _ => return Err(Error::BadInput),
        };
    }
    if freq7.len() != 2 || freq8.len() != 2 {
        return Err(Error::BadInput);
    }

    // fix A, C: both have frequency 8, C is part of the signals that make up '1'
    let mut signals_1: HashSet<char> = signals
        .iter()
        .find(|s| s.len() == 2)
        .ok_or(Error::BadInput)?
        .chars()
        .fold(HashSet::new(), |mut set, ch| {
            set.insert(ch);
            set
        });
    map.keys().for_each(|ch| {
        signals_1.remove(ch);
    });
    let a = *signals_1.iter().next().ok_or(Error::BadInput)?;
    let b = if a == freq8[0] { freq8[1] } else { freq8[0] };
    if map.contains_key(&a) {
        map.insert(a, Segment::A);
        map.insert(b, Segment::C);
    } else {
        map.insert(a, Segment::C);
        map.insert(b, Segment::A);
    };

    // fix D: both D and G have frequency 7, G is not part of signals that make up '1', '4', '7'
    let mut signals_147: HashSet<char> = signals
        .iter()
        .filter(|s| s.len() >= 2 && s.len() <= 4)
        .fold(HashSet::new(), |mut set, s| {
            for ch in s.chars() {
                set.insert(ch);
            }
            set
        });
    map.keys().for_each(|ch| {
        signals_147.remove(ch);
    });
    map.insert(
        *signals_147.iter().next().ok_or(Error::BadInput)?,
        Segment::D,
    );

    // fix G: must be the only signal left
    let ch = *['a', 'b', 'c', 'd', 'e', 'f', 'g']
        .iter()
        .find(|ch| !map.contains_key(ch))
        .unwrap();
    map.insert(ch, Segment::G);

    Ok(map)
}

fn process_line(input: &str) -> Result<u32, Error> {
    type S = Segment;

    let map = map_signals_to_segments(input)?;

    let output: Vec<_> = input
        .split('|')
        .nth(1)
        .ok_or(Error::BadInput)?
        .split_whitespace()
        .collect();

    let mut number = vec![];
    for signals in output {
        let mut segments: Vec<_> = signals.chars().map(|ch| map[&ch]).collect();
        segments.sort();
        let digit = match segments[..] {
            [S::A, S::B, S::C, S::E, S::F, S::G] => '0',
            [S::C, S::F] => '1',
            [S::A, S::C, S::D, S::E, S::G] => '2',
            [S::A, S::C, S::D, S::F, S::G] => '3',
            [S::B, S::C, S::D, S::F] => '4',
            [S::A, S::B, S::D, S::F, S::G] => '5',
            [S::A, S::B, S::D, S::E, S::F, S::G] => '6',
            [S::A, S::C, S::F] => '7',
            [S::A, S::B, S::C, S::D, S::E, S::F, S::G] => '8',
            [S::A, S::B, S::C, S::D, S::F, S::G] => '9',
            _ => return Err(Error::Segment),
        };
        number.push(digit);
    }
    number
        .into_iter()
        .collect::<String>()
        .parse::<u32>()
        .map_err(|_| Error::Number)
}

fn part_two(input: &str) -> Result<u32, Error> {
    let values = input
        .lines()
        .map(process_line)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(values.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), Ok(26));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), Ok(61229));
    }

    #[test]
    fn test_process_line() {
        assert_eq!(
            process_line(
                "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"
            ),
            Ok(5353)
        );
        assert_eq!(
            process_line(
                "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg"
            ),
            Ok(1197)
        );
    }
}
