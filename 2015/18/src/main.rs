use rustc_hash::FxHashMap;
use std::iter;
use std::str::FromStr;

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

#[derive(Debug, PartialEq)]
enum Cell {
    Alive,
    Dead,
}

type Coordinates = (i32, i32);

#[derive(Debug)]
struct Grid {
    side: usize,
    cells: FxHashMap<Coordinates, Cell>,
}

impl FromStr for Grid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cells = FxHashMap::default();
        let side = s.lines().count();
        for (y, line) in s.lines().enumerate() {
            if line.len() != side {
                return Err(Error::BadInput);
            }
            for (x, ch) in line.chars().enumerate() {
                match ch {
                    '.' => cells.insert((x as i32, y as i32), Cell::Dead),
                    '#' => cells.insert((x as i32, y as i32), Cell::Alive),
                    _ => return Err(Error::BadInput),
                };
            }
        }
        Ok(Grid { side, cells })
    }
}

impl Grid {
    fn count_alive(&self) -> usize {
        self.cells.values().filter(|&c| *c == Cell::Alive).count()
    }

    fn step(&mut self) {
        let mut copy = FxHashMap::default();
        for (k, v) in self.cells.iter() {
            let n = self
                .neighbours(k)
                .filter(|c| self.cells.get(c) == Some(&Cell::Alive))
                .count();
            copy.insert(
                *k,
                match v {
                    Cell::Alive => {
                        if n == 2 || n == 3 {
                            Cell::Alive
                        } else {
                            Cell::Dead
                        }
                    }
                    Cell::Dead => {
                        if n == 3 {
                            Cell::Alive
                        } else {
                            Cell::Dead
                        }
                    }
                },
            );
        }
        self.cells = copy;
    }

    fn neighbours(&self, coordinates: &Coordinates) -> impl Iterator<Item = Coordinates> + '_ {
        let side = self.side as i32;
        let mut v = Vec::new();
        for dy in &[-1, 0, 1] {
            for dx in &[-1, 0, 1] {
                if (*dy, *dx) != (0, 0) {
                    let x = coordinates.0 + dx;
                    let y = coordinates.1 + dy;
                    if x >= 0 && x < side && y >= 0 && y < side {
                        v.push((x, y));
                    }
                }
            }
        }
        iter::from_fn(move || v.pop())
    }
}

fn part_one(input: &str) -> Result<usize, Error> {
    let mut grid = Grid::from_str(input)?;
    for _ in 0..100 {
        grid.step();
    }
    Ok(grid.count_alive())
}

fn part_two(input: &str) -> Result<usize, Error> {
    fn awaken_corners(grid: &mut Grid) {
        let s = grid.side as i32 - 1;
        grid.cells.insert((0, 0), Cell::Alive);
        grid.cells.insert((s, 0), Cell::Alive);
        grid.cells.insert((0, s), Cell::Alive);
        grid.cells.insert((s, s), Cell::Alive);
    }

    let mut grid = Grid::from_str(input)?;
    awaken_corners(&mut grid);
    for _ in 0..100 {
        grid.step();
        awaken_corners(&mut grid);
    }
    Ok(grid.count_alive())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_grid_neighbours() {
        let grid = Grid::from_str(INPUT).unwrap();
        assert_eq!(grid.neighbours(&(0, 0)).count(), 3);
        assert_eq!(grid.neighbours(&(1, 1)).count(), 8);
        assert_eq!(grid.neighbours(&(0, 1)).count(), 5);
    }

    #[test]
    fn test_grid_step() {
        let mut grid = Grid::from_str(INPUT).unwrap();
        assert_eq!(grid.count_alive(), 15);

        grid.step();
        assert_eq!(grid.count_alive(), 11);

        grid.step();
        assert_eq!(grid.count_alive(), 8);

        grid.step();
        assert_eq!(grid.count_alive(), 4);

        grid.step();
        assert_eq!(grid.count_alive(), 4);
    }
}
