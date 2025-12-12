use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 521434)?;
    aoc::run!(part_two(input), 248279)?;
    Ok(())
}

#[derive(Debug, Clone)]
struct Lens<'a> {
    label: &'a str,
    focal_length: usize,
}

#[derive(Debug)]
enum Op<'a> {
    Remove(&'a str),
    Insert(Lens<'a>),
}

fn parse(input: &str) -> Result<Vec<Op<'_>>> {
    input
        .trim()
        .split(',')
        .map(|s| {
            let len = s.len();
            if s.ends_with('-') {
                Ok(Op::Remove(&s[..len - 1]))
            } else if s.chars().nth(len - 2) == Some('=') {
                let lens = Lens {
                    label: &s[..len - 2],
                    focal_length: s[len - 1..].parse()?,
                };
                Ok(Op::Insert(lens))
            } else {
                Err(anyhow!("bad input {s}"))
            }
        })
        .collect::<Result<Vec<_>>>()
}

fn hash(string: &str) -> u8 {
    string.chars().fold(0_u32, |acc, ch| {
        assert!(ch.is_ascii());
        ((acc + ch as u32) * 17) % 256
    }) as u8
}

fn part_one(input: &str) -> Result<usize> {
    Ok(input
        .trim()
        .split(',')
        .map(|s| hash(s) as usize)
        .sum::<usize>())
}

fn part_two(input: &str) -> Result<usize> {
    let mut map: Vec<Vec<Lens>> = vec![vec![]; 256];
    for op in parse(input)? {
        match op {
            Op::Remove(label) => {
                let v = &mut map[hash(label) as usize];
                if let Some(pos) = v.iter().position(|lens| lens.label == label) {
                    v.remove(pos);
                }
            }
            Op::Insert(lens) => {
                let v = &mut map[hash(lens.label) as usize];
                if let Some(l) = v.iter_mut().find(|l| l.label == lens.label) {
                    l.focal_length = lens.focal_length;
                } else {
                    v.push(lens);
                }
            }
        }
    }
    Ok(map
        .into_iter()
        .enumerate()
        .map(|(i, v)| {
            v.into_iter()
                .enumerate()
                .map(|(j, l)| (i + 1) * (j + 1) * l.focal_length)
                .sum::<usize>()
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 1320);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 145);
    }
}
