// Advent of code 2021 day 05
//
// # Task
// Line segments (vertical, horizontal and diagonal [limited to 45 degrees]) are distributed on a
// grid. All segments have a width of 1 units. Count the number of grid cells where two or more
// line segments overlap.
//
// # Solution
// For each row of the grid, calculate the Begin and End X values of each line segment that
// overlays the row. Horizontal and diagonal segments contribute a segment of length 1 unit per
// row.
//
// For each row:
//   - Sort the row: smaller values first, and Begin values before End values in case of identical
//     values
//   - Create an empty stack
//   - Traverse all values:
//     - if Begin: push value onto stack
//     - if End: pop value from stack; if stack is currently of length 1: the length of the
//       overlapping segment just exited is the value popped minus the top of the stack
use std::{cmp::Ordering, collections::HashMap};

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
    EmptyStack,
}

#[derive(Debug, PartialEq, Eq)]
enum Value {
    Begin(i32),
    End(i32),
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let (t1, v1) = match *self {
            Value::Begin(v) => (0, v),
            Value::End(v) => (1, v),
        };
        let (t2, v2) = match *other {
            Value::Begin(v) => (0, v),
            Value::End(v) => (1, v),
        };
        match v1 - v2 {
            diff if diff < 0 => Some(Ordering::Less),
            0 => {
                if t1 - t2 < 0 {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Greater)
                }
            }
            diff if diff > 0 => Some(Ordering::Greater),
            _ => panic!(),
        }
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        let (t1, v1) = match *self {
            Value::Begin(v) => (0, v),
            Value::End(v) => (1, v),
        };
        let (t2, v2) = match *other {
            Value::Begin(v) => (0, v),
            Value::End(v) => (1, v),
        };
        if t1 != t2 {
            t1.cmp(&t2)
        } else {
            v1.cmp(&v2)
        }
    }
}

fn parse_input(input: &str, include_diagonals: bool) -> Result<HashMap<i32, Vec<Value>>, Error> {
    fn sort(a: i32, b: i32) -> (i32, i32) {
        if a < b {
            (a, b)
        } else {
            (b, a)
        }
    }

    let mut map = HashMap::new();
    for line in input.lines() {
        let line = line.replace("->", "").replace(",", " ");
        let splits = line.split_whitespace().collect::<Vec<_>>();
        if splits.len() != 4 {
            return Err(Error::BadInput);
        }
        let x1 = splits[0].parse::<i32>().map_err(|_| Error::BadInput)?;
        let y1 = splits[1].parse::<i32>().map_err(|_| Error::BadInput)?;
        let x2 = splits[2].parse::<i32>().map_err(|_| Error::BadInput)?;
        let y2 = splits[3].parse::<i32>().map_err(|_| Error::BadInput)?;

        if x1 == x2 {
            // vertical line
            let (y1, y2) = sort(y1, y2);
            for y in y1..=y2 {
                let row = map.entry(y).or_insert_with(Vec::new);
                row.push(Value::Begin(x1));
                row.push(Value::End(x1 + 1));
            }
        } else if y1 == y2 {
            // horizontal line
            let (x1, x2) = sort(x1, x2);
            let row = map.entry(y1).or_insert_with(Vec::new);
            row.push(Value::Begin(x1));
            row.push(Value::End(x2 + 1));
        } else if include_diagonals {
            // diagonal line, guaranteed to be at 45 degrees angle
            let x_step = if x1 < x2 { 1 } else { -1 };
            let y_step = if y1 < y2 { 1 } else { -1 };
            let mut x = x1;
            let mut y = y1;
            loop {
                let row = map.entry(y).or_insert_with(Vec::new);
                row.push(Value::Begin(x));
                row.push(Value::End(x + 1));

                x += x_step;
                y += y_step;
                if x == x2 {
                    let row = map.entry(y).or_insert_with(Vec::new);
                    row.push(Value::Begin(x));
                    row.push(Value::End(x + 1));
                    break;
                }
            }
        }
    }
    Ok(map)
}

fn overlapping_points(input: &str, include_diagonals: bool) -> Result<i32, Error> {
    let mut sum = 0;
    for mut row in parse_input(input, include_diagonals)?.into_values() {
        let mut stack = Vec::new();
        row.sort();
        for value in row {
            match value {
                Value::Begin(v) => {
                    stack.push(v);
                }
                Value::End(v) => {
                    if let Some(b) = stack.pop() {
                        if stack.len() == 1 {
                            sum += v - b;
                        }
                    } else {
                        return Err(Error::EmptyStack);
                    }
                }
            }
        }
    }
    Ok(sum)
}

fn part_one(input: &str) -> Result<i32, Error> {
    overlapping_points(input, false)
}

fn part_two(input: &str) -> Result<i32, Error> {
    overlapping_points(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_value() {
        assert!(Value::Begin(0) < Value::End(0));
        assert!(Value::Begin(0) < Value::End(10));
        assert!(Value::Begin(0) < Value::Begin(10));
        assert!(Value::End(0) < Value::Begin(10));
        assert!(Value::Begin(0) < Value::End(0));
        assert!(Value::End(0) > Value::Begin(0));

        let mut v = vec![
            Value::Begin(4),
            Value::End(6),
            Value::Begin(5),
            Value::End(8),
        ];
        v.sort();
        assert_eq!(
            v,
            vec![
                Value::Begin(4),
                Value::Begin(5),
                Value::End(6),
                Value::End(8)
            ]
        );

        let mut v = vec![
            Value::Begin(0),
            Value::End(2),
            Value::Begin(3),
            Value::End(5),
        ];
        v.sort();
        assert_eq!(
            v,
            vec![
                Value::Begin(0),
                Value::End(2),
                Value::Begin(3),
                Value::End(5),
            ]
        );
    }

    #[test]
    fn test_parse_input() {
        let rows = parse_input(INPUT, false).unwrap();
        assert_eq!(rows.len(), 6);
        assert_eq!(rows[&0], vec![Value::Begin(7), Value::End(8)]);
        assert_eq!(rows[&4].len() % 2, 0);

        let rows = parse_input(INPUT, true).unwrap();
        assert_eq!(rows.len(), 10);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), Ok(5));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), Ok(12));
    }
}
