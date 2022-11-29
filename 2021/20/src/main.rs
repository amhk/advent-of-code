use std::collections::BTreeSet;

fn main() {
    let input = include_str!("input.txt");

    let answer = part_one(input).expect("no solution for part one");
    println!("part 1: {}", answer);
    assert_eq!(answer, 5622); // FIXME: rm

    let answer = part_two(input).expect("no solution for part two");
    println!("part 2: {}", answer);
}

#[derive(Debug, PartialEq)]
enum Error {
    BadInput,
    UnknownChar,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Bit {
    Zero,
    One,
}

impl Bit {
    fn into_usize(bits: &[Bit; 9]) -> usize {
        let s = bits
            .iter()
            .map(|b| match b {
                Bit::Zero => '0',
                Bit::One => '1',
            })
            .collect::<String>();
        usize::from_str_radix(&s, 2).unwrap()
    }
}

impl TryFrom<char> for Bit {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Zero),
            '#' => Ok(Self::One),
            _ => Err(Error::UnknownChar),
        }
    }
}

type TranslationString = [Bit; 512];

struct InfiniteGrid {
    grid: BTreeSet<(i32, i32)>,
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
}

impl InfiniteGrid {
    fn new() -> InfiniteGrid {
        InfiniteGrid {
            grid: BTreeSet::new(),
            min_x: 0,
            min_y: 0,
            max_x: 0,
            max_y: 0,
        }
    }

    fn insert(&mut self, (x, y): (i32, i32), bit: Bit) {
        match bit {
            Bit::Zero => {
                self.grid.remove(&(x, y));
            }
            Bit::One => {
                self.grid.insert((x, y));
            }
        }
    }

    // ugly, caller needs to remember to call this after having inserted items
    fn update_bounding_box(&mut self) {
        self.min_x = self.grid.iter().map(|(x, _)| *x).min().unwrap_or_default();
        self.min_y = self.grid.iter().map(|(_, y)| *y).min().unwrap_or_default();
        self.max_x = self.grid.iter().map(|(x, _)| *x).max().unwrap_or_default();
        self.max_y = self.grid.iter().map(|(_, y)| *y).max().unwrap_or_default();
    }

    fn get(&self, (x, y): (i32, i32), inverted: bool) -> Bit {
        if x < self.min_x || y < self.min_y || x > self.max_x || y > self.max_y {
            if inverted {
                return Bit::One;
            } else {
                return Bit::Zero;
            }
        }

        if self.grid.contains(&(x, y)) {
            Bit::One
        } else {
            Bit::Zero
        }
    }

    fn iter(&self) -> impl Iterator<Item = (i32, i32)> {
        let mut v = vec![];
        for y in self.min_y - 1..=self.max_y + 1 {
            for x in self.min_x - 1..=self.max_x + 1 {
                v.push((x, y));
            }
        }
        v.into_iter()
    }

    fn enchance(&self, translation: &TranslationString, inverted: bool) -> InfiniteGrid {
        let mut grid = InfiniteGrid::new();
        for (x, y) in self.iter() {
            let bits = [
                self.get((x - 1, y - 1), inverted),
                self.get((x, y - 1), inverted),
                self.get((x + 1, y - 1), inverted),
                self.get((x - 1, y), inverted),
                self.get((x, y), inverted),
                self.get((x + 1, y), inverted),
                self.get((x - 1, y + 1), inverted),
                self.get((x, y + 1), inverted),
                self.get((x + 1, y + 1), inverted),
            ];
            let index = Bit::into_usize(&bits);
            debug_assert!(index < 512);
            let bit = translation[index];
            grid.insert((x, y), bit);
        }
        grid.update_bounding_box();
        grid
    }

    fn len(&self) -> usize {
        self.grid.len()
    }
}

impl std::fmt::Debug for InfiniteGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min_x = self.grid.iter().map(|(x, _)| *x).min().unwrap_or_default();
        let min_y = self.grid.iter().map(|(_, y)| *y).min().unwrap_or_default();
        let max_x = self.grid.iter().map(|(x, _)| *x).max().unwrap_or_default();
        let max_y = self.grid.iter().map(|(_, y)| *y).max().unwrap_or_default();
        let mut lines = vec!["grid".to_string()];
        for y in min_y..=max_y {
            let mut line = String::new();
            for x in min_x..=max_x {
                match self.get((x, y), false) {
                    Bit::Zero => {
                        line.push('.');
                    }
                    Bit::One => {
                        line.push('#');
                    }
                }
            }
            lines.push(line);
        }
        write!(f, "{}", lines.join("\n"))
    }
}

fn parse_input(input: &str) -> Result<(TranslationString, InfiniteGrid), Error> {
    let mut lines = input.lines();
    let first = lines.next().ok_or(Error::BadInput)?;
    if first.len() != 512 {
        return Err(Error::BadInput);
    }
    let _second = lines.next().ok_or(Error::BadInput)?;
    let mut translation = [Bit::Zero; 512];
    for (i, ch) in first.chars().enumerate() {
        translation[i] = ch.try_into()?;
    }
    let mut grid = InfiniteGrid::new();
    for (y, line) in lines.enumerate() {
        for (x, ch) in line.chars().enumerate() {
            grid.insert((x as i32, y as i32), ch.try_into()?);
        }
    }
    grid.update_bounding_box();
    Ok((translation, grid))
}

fn part_x(input: &str, iterations: usize) -> Result<usize, Error> {
    let (translation, mut grid) = parse_input(input)?;
    let invert_odd = translation[0] == Bit::One;

    for i in 0..iterations {
        let inverted = invert_odd && i % 2 != 0;
        grid = grid.enchance(&translation, inverted);
    }

    Ok(grid.len())
}

fn part_one(input: &str) -> Result<usize, Error> {
    part_x(input, 2)
}

fn part_two(input: &str) -> Result<usize, Error> {
    part_x(input, 50)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_grid() {
        let (translation, grid) = parse_input(INPUT).unwrap();
        assert_eq!(grid.iter().count(), 7 * 7);
        assert_eq!(grid.get((0, 0), false), Bit::One);
        assert_eq!(grid.get((1, 1), false), Bit::Zero);
        assert_eq!(grid.get((1000, 1000), false), Bit::Zero);
        assert_eq!(grid.get((-1000, -1000), false), Bit::Zero);
        assert_eq!(grid.len(), 10);
        dbg!(&grid);

        let grid = grid.enchance(&translation, false);
        dbg!(&grid);
        assert_eq!(grid.len(), 24);

        //let translation = translation_string_inverted(&translation);
        //let grid = grid.inverted();
        let grid = grid.enchance(&translation, false);
        dbg!(&grid);
        assert_eq!(grid.len(), 35);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), Ok(35));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), Ok(3351));
    }
}
