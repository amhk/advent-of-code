use std::collections::HashMap;

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

#[derive(Debug, Clone, PartialEq)]
enum State {
    Vacant,
    Occupied,
}

#[derive(Debug, Clone)]
struct Seat {
    state: State,
    neighbors: Vec<usize>,
}

fn parse_input(input: &str, horizon: usize) -> Result<Vec<Seat>, Error> {
    const VECTORS: &[(isize, isize)] = &[
        (-1, -1), // nw
        (0, -1),  // n
        (1, -1),  // ne
        (-1, 0),  // w
        (1, 0),   // e
        (-1, 1),  // sw
        (0, 1),   // s
        (1, 1),   // se
    ];

    // the seats
    let count = input.chars().filter(|&ch| ch == 'L').count();
    let mut seats = vec![
        Seat {
            state: State::Vacant,
            neighbors: Vec::new()
        };
        count
    ];

    // coordinates -> index into seats vec
    let mut map = HashMap::<(isize, isize), usize>::new();
    let mut index = 0;
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == 'L' {
                map.insert((x as isize, y as isize), index);
                index += 1;
            }
        }
    }

    // fixup Seat.neighbors
    let x_max: isize = input.lines().next().unwrap().len() as isize;
    let y_max: isize = input.lines().count() as isize;
    for (&(x0, y0), &index) in map.iter() {
        let seat = seats.get_mut(index).expect("internal error");
        for (dx, dy) in VECTORS {
            let (mut x, mut y) = (x0, y0);
            let mut h = horizon as isize;
            loop {
                x += dx;
                y += dy;
                h -= 1;
                if h < 0 || x >= x_max || y < 0 || y >= y_max {
                    break;
                }
                if let Some(&index) = map.get(&(x, y)) {
                    seat.neighbors.push(index);
                    break;
                }
            }
        }
    }

    // invariant: each neighboring pair of seats can see each other
    // so the total number of neighbors must be even
    assert!(seats.iter().fold(0, |acc, seat| seat.neighbors.len() + acc) % 2 == 0);

    Ok(seats)
}

fn solve(input: &str, horizon: usize, croweded_threshold: usize) -> Result<usize, Error> {
    fn neighbor_count(seats: &[Seat], index: usize) -> usize {
        seats[index]
            .neighbors
            .iter()
            .filter_map(|&index| match seats[index].state {
                State::Occupied => Some(()),
                _ => None,
            })
            .count()
    }

    let mut seats = parse_input(input, horizon)?;
    let mut new_states: Vec<State> = vec![State::Vacant; seats.len()];

    loop {
        let mut change = false;
        for (index, seat) in seats.iter().enumerate() {
            let n = neighbor_count(&seats, index);
            if seat.state == State::Vacant && n == 0 {
                new_states[index] = State::Occupied;
                change = true;
            } else if seat.state == State::Occupied && n >= croweded_threshold {
                new_states[index] = State::Vacant;
                change = true;
            } else {
                new_states[index] = seat.state.clone();
            }
        }
        for (index, state) in new_states.iter().enumerate() {
            seats[index].state = state.clone();
        }
        if !change {
            break;
        }
    }

    Ok(seats
        .iter()
        .filter(|seat| seat.state == State::Occupied)
        .count())
}

fn part_one(input: &str) -> Result<usize, Error> {
    solve(input, 1, 4)
}

fn part_two(input: &str) -> Result<usize, Error> {
    let cols = input.lines().next().ok_or(Error::BadInput)?.chars().count();
    let rows = input.lines().count();
    solve(input, usize::max(cols, rows) + 1, 5)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_parse_input() {
        // input:
        //   . 0 . 1
        //   . . . .
        //   . 2 . .
        //   . . . 3
        //   4 . . .
        let input = ".L.L\n....\n.L..\n...L\nL...\n";

        let seats = parse_input(input, 10).unwrap();
        assert_eq!(seats.len(), 5);
        assert_eq!(seats[0].neighbors, vec![1, 2]);
        assert_eq!(seats[1].neighbors, vec![0, 2, 3]);
        assert_eq!(seats[2].neighbors, vec![0, 1]);
        assert_eq!(seats[3].neighbors, vec![1]);
        assert_eq!(seats[4].neighbors, vec![]);

        let seats = parse_input(input, 2).unwrap();
        assert_eq!(seats.len(), 5);
        assert_eq!(seats[0].neighbors, vec![1, 2]);
        assert_eq!(seats[1].neighbors, vec![0, 2]);
        assert_eq!(seats[2].neighbors, vec![0, 1]);
        assert_eq!(seats[3].neighbors, vec![]);
        assert_eq!(seats[4].neighbors, vec![]);
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
