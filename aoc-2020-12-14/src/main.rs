use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
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
    Internal,
}

fn and_mask(value: u64, mask: &str) -> Result<u64, Error> {
    if mask.len() != 36 {
        return Err(Error::Internal);
    }
    let value = format!("{:036b}", value);
    let value: String = mask
        .chars()
        .zip(value.chars())
        .map(|(m, v)| match m {
            '0' => '0',
            '1' => '1',
            _ => v,
        })
        .collect();
    Ok(u64::from_str_radix(&value, 2).map_err(|_| Error::Internal)?)
}

lazy_static! {
    static ref RE_MASK: Regex = Regex::new(r"^mask = ([01X]{36})$").unwrap();
    static ref RE_STORE: Regex = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
}

fn part_one(input: &str) -> Result<u64, Error> {
    let sum: u64 = input
        .lines()
        .fold(
            (
                HashMap::<usize, u64>::new(),
                "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
            ),
            |(mut mem, mut mask), line| {
                if let Some(cap) = RE_MASK.captures(line) {
                    mask = cap.get(1).unwrap().as_str();
                }
                if let Some(cap) = RE_STORE.captures(line) {
                    let address = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
                    let value = cap.get(2).unwrap().as_str().parse::<u64>().unwrap();
                    let value = and_mask(value, mask).unwrap();
                    mem.insert(address, value);
                }
                (mem, mask)
            },
        )
        .0
        .values()
        .sum();
    Ok(sum)
}

fn floating_addresses(address: u64, mask: &str) -> HashSet<u64> {
    assert_eq!(mask.len(), 36);
    let address = format!("{:036b}", address);
    let address: String = mask
        .chars()
        .zip(address.chars())
        .map(|(m, a)| match m {
            '0' => a,
            '1' => '1',
            'X' => 'X',
            _ => panic!(),
        })
        .collect();
    fn expand(address: &str, index: usize) -> Vec<String> {
        for i in index..address.len() {
            if address.chars().nth(i) == Some('X') {
                let mut zero = expand(&address.replacen('X', "0", 1), i + 1);
                let mut one = expand(&address.replacen('X', "1", 1), i + 1);
                zero.append(&mut one);
                return zero;
            }
        }
        vec![address.to_string()]
    }
    expand(&address, 0)
        .iter()
        .map(|a| u64::from_str_radix(a, 2).unwrap())
        .collect()
}

fn part_two(input: &str) -> Result<u64, Error> {
    let sum: u64 = input
        .lines()
        .fold(
            (
                HashMap::<usize, u64>::new(),
                "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
            ),
            |(mut mem, mut mask), line| {
                if let Some(cap) = RE_MASK.captures(line) {
                    mask = cap.get(1).unwrap().as_str();
                }
                if let Some(cap) = RE_STORE.captures(line) {
                    let address = cap.get(1).unwrap().as_str().parse::<u64>().unwrap();
                    let value = cap.get(2).unwrap().as_str().parse::<u64>().unwrap();
                    for a in floating_addresses(address, mask) {
                        mem.insert(a as usize, value);
                    }
                }
                (mem, mask)
            },
        )
        .0
        .values()
        .sum();
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask() {
        assert_eq!(and_mask(0, "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"), Ok(0));
        assert_eq!(and_mask(1, "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX10"), Ok(2));
    }

    #[test]
    fn test_part_one() {
        const INPUT: &str = include_str!("test-input-part-one.txt");
        assert_eq!(part_one(INPUT), Ok(101 + 64));
    }

    #[test]
    fn test_floating_addresses() {
        let mut set = HashSet::new();
        set.insert(26);
        set.insert(27);
        set.insert(58);
        set.insert(59);
        assert_eq!(
            floating_addresses(42, "000000000000000000000000000000X1001X"),
            set
        );
    }

    #[test]
    fn test_part_two() {
        const INPUT: &str = include_str!("test-input-part-two.txt");
        assert_eq!(part_two(INPUT), Ok(208));
    }
}
