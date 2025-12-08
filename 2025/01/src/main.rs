use anyhow::{bail, ensure, Result};

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 1195)?;
    aoc::run!(part_two(input), 6770)?;
    Ok(())
}

enum Instruction {
    Left(i32),
    Right(i32),
}

fn parse(input: &str) -> Result<Vec<Instruction>> {
    let mut out = vec![];
    for line in input.lines() {
        ensure!(line.len() > 1);
        let value = line[1..].parse::<i32>()?;
        let instr = match &line[0..1] {
            "L" => Instruction::Left(value),
            "R" => Instruction::Right(value),
            _ => bail!("unexpected input"),
        };
        out.push(instr);
    }
    Ok(out)
}

fn part_one(input: &str) -> Result<usize> {
    let mut counter = 50;
    let mut zeros = 0;
    for instr in parse(input)? {
        match instr {
            Instruction::Left(value) => {
                counter = (counter - value) % 100;
            }
            Instruction::Right(value) => {
                counter = (counter + value) % 100;
            }
        }
        if counter == 0 {
            zeros += 1;
        }
    }
    Ok(zeros)
}

fn part_two(input: &str) -> Result<i32> {
    let mut counter = 50;
    let mut zeros = 0;
    for instr in parse(input)? {
        debug_assert!(counter >= 0);
        debug_assert!(counter < 100);
        match instr {
            Instruction::Left(mut value) => {
                zeros += value / 100;
                value %= 100;
                if counter != 0 && counter - value <= 0 {
                    zeros += 1;
                }
                counter = (counter - value) % 100;
                if counter < 0 {
                    counter += 100;
                }
            }
            Instruction::Right(mut value) => {
                zeros += value / 100;
                value %= 100;
                if counter + value >= 100 {
                    zeros += 1;
                }
                counter = (counter + value) % 100;
            }
        }
    }
    Ok(zeros)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 3);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("L50\nR50").unwrap(), 1);
        assert_eq!(part_two("L50\nL50").unwrap(), 1);
        assert_eq!(part_two("R50\nL50").unwrap(), 1);
        assert_eq!(part_two("R50\nR50").unwrap(), 1);
        assert_eq!(part_two("R200\nL400").unwrap(), 6);
        assert_eq!(part_two("R49\nR200").unwrap(), 2);
        assert_eq!(part_two("L49\nL200").unwrap(), 2);
        assert_eq!(part_two("R1000").unwrap(), 10);
        assert_eq!(part_two("L50\nR1000").unwrap(), 11);
        assert_eq!(part_two("R50\nR1000").unwrap(), 11);
        assert_eq!(part_two("L50\nR101").unwrap(), 2);
        assert_eq!(part_two(INPUT).unwrap(), 6);
    }
}
