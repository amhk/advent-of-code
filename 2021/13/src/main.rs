use std::{collections::HashSet, fmt::Display};

fn main() {
    let input = include_str!("input.txt");

    let answer = part_one(input).expect("no solution for part one");
    println!("part 1: {}", answer);

    let answer = part_two(input).expect("no solution for part two");
    println!("part 2:\n{}", answer);
}

#[derive(Debug, PartialEq)]
enum Error {
    BadInput,
    IntConversion,
    BadFoldX(u32),
    BadFoldY(u32),
}

#[derive(Debug)]
enum Instruction {
    FoldAlongX(u32),
    FoldAlongY(u32),
}

#[derive(Debug)]
struct Paper {
    dots: HashSet<(u32, u32)>,
    width: u32,
    height: u32,
}

impl Paper {
    fn fold_along_x(&mut self, pivot: u32) -> Result<(), Error> {
        // can't handle folds that extend beyond the edge of the paper
        // no dots allowed on crease line
        if pivot < self.width / 2 || self.dots.iter().any(|(x, _)| x == &pivot) {
            return Err(Error::BadFoldX(pivot));
        }

        // drain_filter is nightly only
        let removed: Vec<_> = self
            .dots
            .iter()
            .filter(|(x, _)| x > &pivot)
            .cloned()
            .collect();
        self.dots.retain(|coord| !removed.contains(coord));

        for (x, y) in removed {
            self.dots.insert((2 * pivot - x, y));
        }
        self.width = pivot;

        Ok(())
    }

    fn fold_along_y(&mut self, pivot: u32) -> Result<(), Error> {
        // can't handle folds that extend beyond the edge of the paper
        // no dots allowed on crease line
        if pivot < self.height / 2 || self.dots.iter().any(|(_, y)| y == &pivot) {
            return Err(Error::BadFoldY(pivot));
        }

        // drain_filter is nightly only
        let removed: Vec<_> = self
            .dots
            .iter()
            .filter(|(_, y)| y > &pivot)
            .cloned()
            .collect();
        self.dots.retain(|coord| !removed.contains(coord));

        for (x, y) in removed {
            self.dots.insert((x, 2 * pivot - y));
        }
        self.height = pivot;

        Ok(())
    }

    fn fold(&mut self, instr: &Instruction) -> Result<(), Error> {
        match *instr {
            Instruction::FoldAlongX(pivot) => self.fold_along_x(pivot),
            Instruction::FoldAlongY(pivot) => self.fold_along_y(pivot),
        }
    }

    fn count(&self) -> usize {
        self.dots.len()
    }
}

impl Display for Paper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.dots.contains(&(x, y)) {
                    s.push('#');
                } else {
                    s.push('.');
                }
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

impl TryFrom<&str> for Paper {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut dots = HashSet::new();
        for line in value.lines() {
            let chunks: Vec<_> = line.split(',').collect();
            if chunks.len() != 2 {
                return Err(Error::BadInput);
            }
            let x: u32 = chunks[0].parse().map_err(|_| Error::IntConversion)?;
            let y: u32 = chunks[1].parse().map_err(|_| Error::IntConversion)?;
            dots.insert((x, y));
        }

        let width = dots.iter().max_by_key(|(x, _)| x).ok_or(Error::BadInput)?.0;
        let height = dots.iter().max_by_key(|(_, y)| y).ok_or(Error::BadInput)?.1;

        Ok(Paper {
            dots,
            width,
            height,
        })
    }
}

fn parse_input(input: &str) -> Result<(Paper, Vec<Instruction>), Error> {
    let chunks: Vec<_> = input.split("\n\n").collect();
    if chunks.len() != 2 {
        return Err(Error::BadInput);
    }

    let paper = Paper::try_from(chunks[0])?;

    let mut instructions = Vec::new();
    for line in chunks[1].lines() {
        if let Some(substring) = line.strip_prefix("fold along x=") {
            let number: u32 = substring.parse().map_err(|_| Error::IntConversion)?;
            instructions.push(Instruction::FoldAlongX(number));
        } else if let Some(substring) = line.strip_prefix("fold along y=") {
            let number: u32 = substring.parse().map_err(|_| Error::IntConversion)?;
            instructions.push(Instruction::FoldAlongY(number));
        } else {
            return Err(Error::BadInput);
        }
    }

    Ok((paper, instructions))
}

fn part_one(input: &str) -> Result<usize, Error> {
    let (mut paper, instructions) = parse_input(input)?;
    let instr = instructions.first().ok_or(Error::BadInput)?;
    paper.fold(instr)?;
    Ok(paper.count())
}

fn part_two(input: &str) -> Result<String, Error> {
    let (mut paper, instructions) = parse_input(input)?;
    for instr in instructions {
        paper.fold(&instr)?;
    }
    Ok(paper.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), Ok(17));
    }

    #[test]
    fn test_part_two() {
        let expected = include_str!("test-expected.txt").to_string();
        assert_eq!(part_two(INPUT), Ok(expected));
    }
}
