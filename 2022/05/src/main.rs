use anyhow::{Context, Error, Result};
use once_cell::sync::Lazy;
use regex::Regex;

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run_custom_check!(part_one(input), |answer| answer == "SPFMVDTZT")?;
    aoc::run_custom_check!(part_two(input), |answer| answer == "ZFSJBPRFP")?;
    Ok(())
}

#[derive(Debug, PartialEq)]
struct Instruction {
    src: usize,
    dest: usize,
    repeat: usize,
}

impl TryFrom<&str> for Instruction {
    type Error = Error;

    fn try_from(s: &str) -> std::result::Result<Self, Self::Error> {
        static REGEX: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap());
        let caps = REGEX
            .captures(s)
            .with_context(|| format!("unexpected input '{}'", s))?;
        Ok(Instruction {
            src: caps.get(2).unwrap().as_str().parse().unwrap(),
            dest: caps.get(3).unwrap().as_str().parse().unwrap(),
            repeat: caps.get(1).unwrap().as_str().parse().unwrap(),
        })
    }
}

fn parse(input: &str) -> Result<(Vec<Vec<char>>, Vec<Instruction>)> {
    let (stack_input, instructions_input) = input
        .split_once("\n\n")
        .context("missing \\n\\n in input")?;
    let stack_input: Vec<_> = stack_input.lines().rev().collect();
    let count = stack_input[0].split_whitespace().count();
    let mut stacks: Vec<Vec<char>> = vec![vec![]; count];
    for line in stack_input.iter().skip(1) {
        for (i, index) in (1..=count * 4).step_by(4).enumerate() {
            if let Some(ch) = line.chars().nth(index) {
                if ch != ' ' {
                    stacks[i].push(ch);
                }
            }
        }
    }

    let mut instructions = Vec::new();
    for line in instructions_input.lines() {
        instructions.push(line.try_into()?);
    }

    Ok((stacks, instructions))
}

fn part_one(input: &str) -> Result<String> {
    let (mut stacks, instructions) = parse(input)?;
    for instr in instructions {
        for _ in 0..instr.repeat {
            let src = stacks
                .get_mut(instr.src - 1)
                .with_context(|| format!("bad src index {}", instr.src))?;
            let ch = src.pop().context("empty stack")?;
            let dest = stacks
                .get_mut(instr.dest - 1)
                .with_context(|| format!("bad dest index {}", instr.src))?;
            dest.push(ch);
        }
    }
    let mut message = String::new();
    for stack in stacks {
        let ch = *stack.last().context("empty stack")?;
        message.push(ch);
    }
    Ok(message)
}

fn part_two(input: &str) -> Result<String> {
    let (mut stacks, instructions) = parse(input)?;
    for instr in instructions {
        let mut tmp = Vec::new();
        let src = stacks
            .get_mut(instr.src - 1)
            .with_context(|| format!("bad src index {}", instr.src))?;
        for _ in 0..instr.repeat {
            let ch = src.pop().context("empty stack")?;
            tmp.push(ch);
        }
        tmp.reverse();
        let dest = stacks
            .get_mut(instr.dest - 1)
            .with_context(|| format!("bad dest index {}", instr.src))?;
        dest.append(&mut tmp);
    }
    let mut message = String::new();
    for stack in stacks {
        let ch = *stack.last().context("empty stack")?;
        message.push(ch);
    }
    Ok(message)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_parse() {
        let (stacks, instructions) = parse(INPUT).unwrap();

        assert_eq!(stacks, vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]);
        assert_eq!(
            instructions,
            vec![
                Instruction {
                    src: 2,
                    dest: 1,
                    repeat: 1,
                },
                Instruction {
                    src: 1,
                    dest: 3,
                    repeat: 3,
                },
                Instruction {
                    src: 2,
                    dest: 1,
                    repeat: 2,
                },
                Instruction {
                    src: 1,
                    dest: 2,
                    repeat: 1,
                },
            ]
        );
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), "CMZ");
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), "MCD");
    }
}
