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
    NoSolution,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Cell {
    value: i32,
    is_marked: bool,
}

#[derive(Debug)]
struct Board {
    cells: Vec<Cell>,
}

impl Board {
    fn rows(&self) -> Rows<'_> {
        Rows {
            board: self,
            offset: 0,
        }
    }

    fn columns(&self) -> Columns<'_> {
        Columns {
            board: self,
            offset: 0,
        }
    }

    fn cells_mut(&mut self) -> impl Iterator<Item = &mut Cell> {
        self.cells.iter_mut()
    }

    fn has_won(&self) -> bool {
        let winning_rows = self
            .rows()
            .filter(|cells| cells.iter().filter(|cell| cell.is_marked).count() == 5)
            .count();
        let winning_columns = self
            .columns()
            .filter(|cells| cells.iter().filter(|cell| cell.is_marked).count() == 5)
            .count();
        winning_rows > 0 || winning_columns > 0
    }

    fn score(&self, n: i32) -> i32 {
        self.cells
            .iter()
            .filter(|cell| !cell.is_marked)
            .map(|cell| cell.value)
            .sum::<i32>()
            * n
    }
}

impl TryFrom<&str> for Board {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let splits: Vec<_> = value.split_ascii_whitespace().collect();
        if splits.len() != 25 {
            return Err(Error::BadInput);
        }
        let mut cells = Vec::new();
        for s in splits {
            cells.push(Cell {
                value: s.parse::<i32>().map_err(|_| Error::BadInput)?,
                is_marked: false,
            });
        }
        Ok(Board { cells })
    }
}

struct Rows<'a> {
    board: &'a Board,
    offset: usize,
}

impl<'a> Iterator for Rows<'a> {
    type Item = Vec<&'a Cell>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= 25 {
            return None;
        }
        let values = vec![
            &self.board.cells[self.offset],
            &self.board.cells[self.offset + 1],
            &self.board.cells[self.offset + 2],
            &self.board.cells[self.offset + 3],
            &self.board.cells[self.offset + 4],
        ];
        self.offset += 5;
        Some(values)
    }
}

struct Columns<'a> {
    board: &'a Board,
    offset: usize,
}

impl<'a> Iterator for Columns<'a> {
    type Item = Vec<&'a Cell>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= 5 {
            return None;
        }
        let values = vec![
            &self.board.cells[self.offset],
            &self.board.cells[self.offset + 5],
            &self.board.cells[self.offset + 10],
            &self.board.cells[self.offset + 15],
            &self.board.cells[self.offset + 20],
        ];
        self.offset += 1;
        Some(values)
    }
}

fn parse_input(input: &str) -> Result<(Vec<i32>, Vec<Board>), Error> {
    let mut chunks = input.split("\n\n");
    let winning_numbers = chunks
        .next()
        .ok_or(Error::BadInput)?
        .split(',')
        .map(|s| s.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| Error::BadInput)?;
    let mut boards: Vec<Board> = Vec::new();
    for chunk in chunks {
        boards.push(chunk.try_into()?);
    }
    Ok((winning_numbers, boards))
}

fn play_bingo(input: &str, first_board: bool) -> Result<i32, Error> {
    let (winning_numbers, mut boards) = parse_input(input)?;
    for win_num in winning_numbers {
        let n_boards = boards.len();
        for board in boards.iter_mut() {
            let mut marked = false;
            if let Some(cell) = board.cells_mut().find(|cell| cell.value == win_num) {
                debug_assert!(!cell.is_marked);
                cell.is_marked = true;
                marked = true;
            }
            if marked && (first_board || n_boards == 1) && board.has_won() {
                return Ok(board.score(win_num));
            }
        }
        boards.retain(|board| !board.has_won());
    }
    Err(Error::NoSolution)
}

fn part_one(input: &str) -> Result<i32, Error> {
    play_bingo(input, true)
}

fn part_two(input: &str) -> Result<i32, Error> {
    play_bingo(input, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_board() {
        let input = "
         1  2  3  4  5
         6  7  8  9 10
        11 12 13 14 15
        16 17 18 19 20
        21 22 23 24 25
        ";
        let board: Board = input.try_into().unwrap();

        macro_rules! assert_row_col {
            ($row:expr, $expected:expr) => {
                let row = $row.unwrap();
                assert_eq!(row.len(), 5);
                for i in 0..5 {
                    assert_eq!(
                        *row[i],
                        Cell {
                            value: $expected[i],
                            is_marked: false
                        }
                    );
                }
            };
        }

        let mut rows = board.rows();
        assert_row_col!(rows.next(), [1, 2, 3, 4, 5]);
        assert_row_col!(rows.next(), [6, 7, 8, 9, 10]);
        assert_row_col!(rows.next(), [11, 12, 13, 14, 15]);
        assert_row_col!(rows.next(), [16, 17, 18, 19, 20]);
        assert_row_col!(rows.next(), [21, 22, 23, 24, 25]);
        assert_eq!(rows.next(), None);

        let mut columns = board.columns();
        assert_row_col!(columns.next(), [1, 6, 11, 16, 21]);
        assert_row_col!(columns.next(), [2, 7, 12, 17, 22]);
        assert_row_col!(columns.next(), [3, 8, 13, 18, 23]);
        assert_row_col!(columns.next(), [4, 9, 14, 19, 24]);
        assert_row_col!(columns.next(), [5, 10, 15, 20, 25]);
        assert_eq!(columns.next(), None);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), Ok(4512));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), Ok(1924));
    }
}
