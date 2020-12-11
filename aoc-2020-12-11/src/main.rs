#![allow(dead_code, unused_variables)]

fn main() {
    let input = include_str!("input.txt");

    let answer = part_one(&input).expect("no solution for part one");
    println!("part 1: {}", answer);

    let answer = part_two(&input).expect("no solution for part part_two");
    println!("part 2: {}", answer);
}

#[derive(Debug)]
enum Error {
    BadInput,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Cell {
    Border,
    Floor,
    EmptySeat,
    TakenSeat,
}

struct Area {
    seats: Vec<Cell>,
    row_len: usize,
}

impl Area {
    fn new(input: &str) -> Result<Area, Error> {
        let mut seats = Vec::new();
        let mut row_len = 0;

        for line in input.lines() {
            // sanity check, add padding before first row
            let line_len = line.len();
            assert!(line_len > 0);
            match row_len {
                0 => {
                    row_len = line_len + 2;
                    for _ in 0..row_len {
                        seats.push(Cell::Border);
                    }
                }
                _ => {
                    if line_len + 2 != row_len {
                        return Err(Error::BadInput);
                    }
                }
            }

            // add line, with padding before and after
            seats.push(Cell::Border);
            for ch in line.chars() {
                match ch {
                    '.' => seats.push(Cell::Floor),
                    'L' => seats.push(Cell::EmptySeat),
                    _ => return Err(Error::BadInput),
                }
            }
            seats.push(Cell::Border);
        }

        // add padding after last row
        for _ in 0..row_len {
            seats.push(Cell::Border);
        }

        Ok(Area { seats, row_len })
    }

    fn assign_seats(&mut self, threshold_croweded: usize, horizon: Option<usize>) {
        loop {
            let mut copy = self.seats.clone();
            let mut change = false;
            for (i, cell) in copy.iter_mut().enumerate() {
                if let Some((seat, n)) = self.neighbours(i, horizon) {
                    let count = n.iter().filter(|&&c| c == Cell::TakenSeat).count();
                    if seat == Cell::EmptySeat && count == 0 {
                        change = true;
                        *cell = Cell::TakenSeat;
                    } else if seat == Cell::TakenSeat && count >= threshold_croweded {
                        change = true;
                        *cell = Cell::EmptySeat
                    }
                }
            }
            self.seats = copy;
            if !change {
                return;
            }
        }
    }

    fn neighbours(&self, index: usize, horizon: Option<usize>) -> Option<(Cell, Vec<Cell>)> {
        let cell = self.seats[index];
        if matches!(cell, Cell::Border | Cell::Floor) {
            return None;
        }

        // Default to a horizon larger than the input row/col lengths, but small enough to avoid
        // overflow issues when casting from usize to i64.
        let horizon = horizon.unwrap_or(9999);
        let mut v = Vec::new();

        v.push(self.look_nw(index, horizon));
        v.push(self.look_n(index, horizon));
        v.push(self.look_ne(index, horizon));

        v.push(self.look_w(index, horizon));
        v.push(self.look_e(index, horizon));

        v.push(self.look_sw(index, horizon));
        v.push(self.look_s(index, horizon));
        v.push(self.look_se(index, horizon));

        Some((cell, v))
    }

    fn look<F>(&self, index: usize, horizon: usize, func: F) -> Cell
    where
        F: Fn() -> i64,
    {
        assert!(horizon > 0);
        let mut index = index as i64;
        let mut cell = None;
        for dist in 1..=horizon as i64 {
            index += func();
            cell = Some(self.seats[index as usize]);
            if cell != Some(Cell::Floor) {
                break;
            }
        }
        cell.expect("cell should be something")
    }

    fn look_nw(&self, index: usize, horizon: usize) -> Cell {
        self.look(index, horizon, || -1 - self.row_len as i64)
    }

    fn look_n(&self, index: usize, horizon: usize) -> Cell {
        self.look(index, horizon, || -(self.row_len as i64))
    }

    fn look_ne(&self, index: usize, horizon: usize) -> Cell {
        self.look(index, horizon, || 1 - self.row_len as i64)
    }

    fn look_w(&self, index: usize, horizon: usize) -> Cell {
        self.look(index, horizon, || -1)
    }

    fn look_e(&self, index: usize, horizon: usize) -> Cell {
        self.look(index, horizon, || 1)
    }

    fn look_sw(&self, index: usize, horizon: usize) -> Cell {
        self.look(index, horizon, || -1 + self.row_len as i64)
    }

    fn look_s(&self, index: usize, horizon: usize) -> Cell {
        self.look(index, horizon, || self.row_len as i64)
    }

    fn look_se(&self, index: usize, horizon: usize) -> Cell {
        self.look(index, horizon, || 1 + self.row_len as i64)
    }
}

impl std::fmt::Debug for Area {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = String::new();
        self.seats.chunks(self.row_len).for_each(|chunk| {
            for cell in chunk {
                s.push(match *cell {
                    Cell::Border => 'B',
                    Cell::Floor => '.',
                    Cell::EmptySeat => 'L',
                    Cell::TakenSeat => '#',
                });
            }
            s.push('\n');
        });
        write!(f, "{}", s)
    }
}

fn part_one(input: &str) -> Result<usize, Error> {
    let mut area = Area::new(input)?;
    area.assign_seats(4, Some(1));
    let count = area.seats.iter().filter(|&&c| c == Cell::TakenSeat).count();
    Ok(count)
}

fn part_two(input: &str) -> Result<usize, Error> {
    let mut area = Area::new(input)?;
    area.assign_seats(5, None);
    let count = area.seats.iter().filter(|&&c| c == Cell::TakenSeat).count();
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_look() {
        let area = Area::new(INPUT).unwrap();
        const INF: usize = 1000;
        assert_eq!(area.look_nw(13, INF), Cell::Border);
        assert_eq!(area.look_n(13, INF), Cell::Border);
        assert_eq!(area.look_ne(13, INF), Cell::Border);
        assert_eq!(area.look_w(13, INF), Cell::Border);
        assert_eq!(area.look_e(13, INF), Cell::EmptySeat);
        assert_eq!(area.look_sw(13, INF), Cell::Border);
        assert_eq!(area.look_s(13, INF), Cell::EmptySeat);
        assert_eq!(area.look_se(13, INF), Cell::EmptySeat);

        assert_eq!(area.look_e(13, 1), Cell::Floor);
    }

    #[test]
    fn test_neighbours_infinite_horizon() {
        let area = Area::new(INPUT).unwrap();
        let (cell, n) = area.neighbours(13, None).unwrap();
        assert_eq!(cell, Cell::EmptySeat);
        assert_eq!(n.iter().filter(|&&c| c == Cell::Border).count(), 5);
        assert_eq!(n.iter().filter(|&&c| c == Cell::EmptySeat).count(), 3);
    }

    #[test]
    fn test_neighbours_horizon_of_1() {
        let area = Area::new(INPUT).unwrap();
        let (cell, n) = area.neighbours(13, Some(1)).unwrap();
        assert_eq!(cell, Cell::EmptySeat);
        assert_eq!(n.iter().filter(|&&c| c == Cell::Border).count(), 5);
        assert_eq!(n.iter().filter(|&&c| c == Cell::EmptySeat).count(), 2);
        assert_eq!(n.iter().filter(|&&c| c == Cell::Floor).count(), 1);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 37);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 26);
    }
}
