fn main() {
    let input = include_str!("input.txt");

    let answer = part_one(input).expect("no solution for part one");
    println!("part 1: {}", answer);
}

#[derive(Debug, PartialEq)]
enum Error {
    BadInput,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Cell {
    Empty,
    Eastbound,
    Southbound,
}

impl TryFrom<char> for Cell {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Cell::Empty),
            '>' => Ok(Cell::Eastbound),
            'v' => Ok(Cell::Southbound),
            _ => Err(Error::BadInput),
        }
    }
}

struct Grid {
    cells: Vec<Vec<Cell>>,
}

impl Grid {
    fn rows(&self) -> usize {
        self.cells.len()
    }

    fn columns(&self) -> usize {
        self.cells[0].len()
    }

    fn east_target(&self, coordinate: (usize, usize)) -> (usize, usize) {
        let x = (coordinate.0 + 1) % self.columns();
        (x, coordinate.1)
    }

    fn south_target(&self, coordinate: (usize, usize)) -> (usize, usize) {
        let y = (coordinate.1 + 1) % self.rows();
        (coordinate.0, y)
    }

    fn get(&self, coordinate: (usize, usize)) -> Cell {
        self.cells[coordinate.1][coordinate.0]
    }

    pub fn advance(&mut self) -> bool {
        let move_east: Vec<_> = self
            .coord_iter()
            .filter(|&coord| self.get(coord) == Cell::Eastbound)
            .filter(|&coord| self.get(self.east_target(coord)) == Cell::Empty)
            .collect();
        for src in move_east.iter() {
            let dest = self.east_target(*src);
            self.cells[src.1][src.0] = Cell::Empty;
            self.cells[dest.1][dest.0] = Cell::Eastbound;
        }

        let move_south: Vec<_> = self
            .coord_iter()
            .filter(|&coord| self.get(coord) == Cell::Southbound)
            .filter(|&coord| self.get(self.south_target(coord)) == Cell::Empty)
            .collect();
        for src in move_south.iter() {
            let dest = self.south_target(*src);
            self.cells[src.1][src.0] = Cell::Empty;
            self.cells[dest.1][dest.0] = Cell::Southbound;
        }

        move_east.len() + move_south.len() > 0
    }

    fn coord_iter(&self) -> impl Iterator<Item = (usize, usize)> {
        let mut v = vec![];
        for x in 0..self.columns() {
            for y in 0..self.rows() {
                v.push((x, y));
            }
        }
        v.into_iter()
    }
}

impl TryFrom<&str> for Grid {
    type Error = Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut cells = Vec::new();
        let mut len = None;
        for line in input.lines() {
            let mut row = Vec::new();
            for ch in line.chars() {
                row.push(Cell::try_from(ch)?);
            }
            if let Some(l) = len {
                if row.len() != l {
                    return Err(Error::BadInput);
                }
            } else {
                len = Some(row.len());
            }
            cells.push(row);
        }
        if cells.is_empty() {
            return Err(Error::BadInput);
        }
        Ok(Grid { cells })
    }
}

fn part_one(input: &str) -> Result<usize, Error> {
    let mut grid = Grid::try_from(input)?;
    let mut count = 0;
    loop {
        count += 1;
        if !grid.advance() {
            return Ok(count);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_grid() {
        let mut grid = Grid::try_from(INPUT).unwrap();
        assert_eq!(grid.cells.len(), 9);
        assert_eq!(grid.cells[0].len(), 10);
        assert_eq!(grid.cells[1][0], Cell::Empty);
        assert_eq!(grid.cells[2][1], Cell::Eastbound);
        assert_eq!(grid.cells[0][0], Cell::Southbound);
        assert_eq!(grid.rows(), 9);
        assert_eq!(grid.columns(), 10);

        assert_eq!(grid.east_target((0, 0)), (1, 0));
        assert_eq!(grid.east_target((9, 0)), (0, 0));

        assert_eq!(grid.south_target((0, 0)), (0, 1));
        assert_eq!(grid.south_target((0, 8)), (0, 0));

        assert!(grid.advance());
        let expected = Grid::try_from(
            r#"....>.>v.>
v.v>.>v.v.
>v>>..>v..
>>v>v>.>.v
.>v.v...v.
v>>.>vvv..
..v...>>..
vv...>>vv.
>.v.v..v.v"#,
        )
        .unwrap();
        assert_eq!(grid.cells, expected.cells);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), Ok(58));
    }
}
