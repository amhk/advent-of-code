use anyhow::{bail, ensure, Result};

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 40745)?;
    aoc::run!(part_two(input), 90_111_113_594_927)?;
    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum Op {
    U(usize),
    R(usize),
    D(usize),
    L(usize),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct XY {
    x: i64,
    y: i64,
}

impl From<(i64, i64)> for XY {
    fn from(value: (i64, i64)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

fn parse_part_one(input: &str) -> Result<Vec<Op>> {
    let mut ops = vec![];
    for line in input.lines() {
        let parts = line.split(' ').collect::<Vec<_>>();
        ensure!(parts.len() == 3);
        let steps: usize = parts[1].parse()?;
        let op = match parts[0] {
            "U" => Op::U(steps),
            "R" => Op::R(steps),
            "D" => Op::D(steps),
            "L" => Op::L(steps),
            _ => {
                bail!("unexpected op code {}", parts[0]);
            }
        };
        ops.push(op);
    }
    Ok(ops)
}

fn parse_part_two(input: &str) -> Result<Vec<Op>> {
    let mut ops = vec![];
    for line in input.lines() {
        let parts = line.split(' ').collect::<Vec<_>>();
        ensure!(parts.len() == 3);
        let steps = usize::from_str_radix(&parts[2][2..7], 16)?;
        let op = match &parts[2][7..=7] {
            "0" => Op::R(steps),
            "1" => Op::D(steps),
            "2" => Op::L(steps),
            "3" => Op::U(steps),
            _ => {
                bail!("unexpected op code {:?}", &parts[2][7..=7]);
            }
        };
        ops.push(op);
    }
    Ok(ops)
}

fn solve(ops: Vec<Op>) -> Result<usize> {
    let mut xy: XY = (0, 0).into();
    let mut x = vec![];
    let mut y = vec![];
    let mut perimiter = 0;
    for op in ops {
        x.push(xy.x);
        y.push(xy.y);
        match op {
            Op::U(steps) => {
                xy.y -= steps as i64;
                perimiter += steps;
            }
            Op::R(steps) => {
                xy.x += steps as i64;
                perimiter += steps;
            }
            Op::D(steps) => {
                xy.y += steps as i64;
                perimiter += steps;
            }
            Op::L(steps) => {
                xy.x -= steps as i64;
                perimiter += steps;
            }
        }
    }

    // The shoelace algorithm calculates the area of a polygon [1], given a sorted list of
    // vertices. In this puzzle, we also need to include (half of) the circumference. See [2] for
    // details.
    //
    // 1. https://en.wikipedia.org/wiki/Shoelace_formula
    // 2. https://observablehq.com/@jwolondon/advent-of-code-2023-day-18

    let mut iter = y.iter().cycle();
    let _ = iter.next();
    let x_sum: i64 = x.iter().zip(iter).map(|(a, b)| a * b).sum();

    let mut iter = x.iter().cycle();
    let _ = iter.next();
    let y_sum: i64 = y.iter().zip(iter).map(|(a, b)| a * b).sum();

    let area = (x_sum - y_sum).abs() / 2;
    Ok(area as usize + perimiter / 2 + 1)
}

fn part_one(input: &str) -> Result<usize> {
    solve(parse_part_one(input)?)
}

fn part_two(input: &str) -> Result<usize> {
    solve(parse_part_two(input)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 62);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 952_408_144_115);
    }
}
