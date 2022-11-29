use lazy_static::lazy_static;
use regex::{Captures, Regex};

fn main() {
    let input = include_str!("input.txt");

    let answer = part_one(input).expect("no solution for part one");
    println!("part 1: {}", answer);

    let answer = part_two(input).expect("no solution for part two");
    println!("part 2: {}", answer);
}

#[derive(Debug, PartialEq)]
enum Instruction {
    TurnOn((usize, usize), (usize, usize)),
    TurnOff((usize, usize), (usize, usize)),
    Toggle((usize, usize), (usize, usize)),
}

fn parse_input(input: &str) -> Result<Vec<Instruction>, Error> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    }

    fn read_usize(caps: &Captures, i: usize) -> Result<usize, Error> {
        caps.get(i)
            .unwrap()
            .as_str()
            .parse::<usize>()
            .map_err(|_| Error::BadInput)
    }

    let mut instructions = Vec::new();
    for line in input.lines() {
        let caps = RE.captures(line).ok_or(Error::BadInput)?;
        let verb = caps.get(1).unwrap().as_str();
        let a = read_usize(&caps, 2)?;
        let b = read_usize(&caps, 3)?;
        let c = read_usize(&caps, 4)?;
        let d = read_usize(&caps, 5)?;
        let instr = match verb {
            "turn on" => Instruction::TurnOn((a, b), (c, d)),
            "turn off" => Instruction::TurnOff((a, b), (c, d)),
            "toggle" => Instruction::Toggle((a, b), (c, d)),
            _ => return Err(Error::BadInput),
        };
        instructions.push(instr);
    }
    Ok(instructions)
}

struct Grid {
    // Vec instead of array to force heap allocation
    lights: Vec<u32>,
}

impl Grid {
    fn new() -> Grid {
        Grid {
            lights: vec![0; 1000 * 1000],
        }
    }

    fn lights_on_count(&self) -> usize {
        self.lights.iter().filter(|&&l| l != 0).count()
    }

    fn total_brightness(&self) -> u32 {
        self.lights.iter().sum()
    }

    fn toggle(&mut self, begin: (usize, usize), end: (usize, usize)) {
        self.operate(begin, end, |l| (l == 0) as u32);
    }

    fn turn_on(&mut self, begin: (usize, usize), end: (usize, usize)) {
        self.operate(begin, end, |_| 1);
    }

    fn turn_off(&mut self, begin: (usize, usize), end: (usize, usize)) {
        self.operate(begin, end, |_| 0);
    }

    fn increase(&mut self, begin: (usize, usize), end: (usize, usize)) {
        self.operate(begin, end, |l| l + 1);
    }

    fn decrease(&mut self, begin: (usize, usize), end: (usize, usize)) {
        self.operate(begin, end, |l| if l > 0 { l - 1 } else { 0 });
    }

    fn operate<F>(&mut self, begin: (usize, usize), end: (usize, usize), op: F)
    where
        F: Fn(u32) -> u32,
    {
        debug_assert!(begin.0 <= end.0);
        debug_assert!(begin.1 <= end.1);
        debug_assert!(end.0 < 1000);
        debug_assert!(end.1 < 1000);

        for y in begin.1..=end.1 {
            for x in begin.0..=end.0 {
                let index = y * 1000 + x;
                self.lights[index] = op(self.lights[index]);
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum Error {
    BadInput,
}

fn part_one(input: &str) -> Result<usize, Error> {
    let mut grid = Grid::new();
    for instr in parse_input(input)? {
        match instr {
            Instruction::TurnOn((a, b), (c, d)) => grid.turn_on((a, b), (c, d)),
            Instruction::TurnOff((a, b), (c, d)) => grid.turn_off((a, b), (c, d)),
            Instruction::Toggle((a, b), (c, d)) => grid.toggle((a, b), (c, d)),
        }
    }
    Ok(grid.lights_on_count())
}

fn part_two(input: &str) -> Result<u32, Error> {
    let mut grid = Grid::new();
    for instr in parse_input(input)? {
        match instr {
            Instruction::TurnOn((a, b), (c, d)) => grid.increase((a, b), (c, d)),
            Instruction::TurnOff((a, b), (c, d)) => grid.decrease((a, b), (c, d)),
            Instruction::Toggle((a, b), (c, d)) => {
                grid.increase((a, b), (c, d));
                grid.increase((a, b), (c, d));
            }
        }
    }
    Ok(grid.total_brightness())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_on_off_toggle() {
        let mut grid = Grid::new();
        grid.turn_on((0, 0), (999, 999));
        assert_eq!(grid.lights_on_count(), 1000 * 1000);

        let mut grid = Grid::new();
        grid.turn_on((0, 0), (999, 0));
        assert_eq!(grid.lights_on_count(), 1000);

        let mut grid = Grid::new();
        grid.turn_on((499, 499), (500, 500));
        assert_eq!(grid.lights_on_count(), 4);
        grid.turn_off((499, 499), (500, 500));
        assert_eq!(grid.lights_on_count(), 0);
        grid.toggle((499, 499), (500, 500));
        assert_eq!(grid.lights_on_count(), 4);
        grid.toggle((0, 0), (999, 999));
        assert_eq!(grid.lights_on_count(), 1000 * 1000 - 4);
    }

    #[test]
    fn test_grid_inc_dec() {
        let mut grid = Grid::new();
        grid.increase((0, 0), (1, 1));
        assert_eq!(grid.total_brightness(), 4);
        grid.decrease((0, 0), (0, 0));
        assert_eq!(grid.total_brightness(), 3);
        grid.decrease((100, 100), (100, 100));
        assert_eq!(grid.total_brightness(), 3);
    }
}
