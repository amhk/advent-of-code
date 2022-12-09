use std::collections::BTreeSet;

use anyhow::{anyhow, Context, Result};

fn main() -> Result<()> {
    let input = include_str!("input.txt");

    let answer = part_one(input).context("no solution for part one")?;
    println!("Part 1: {}", answer);

    let answer = part_two(input).context("no solution for part two")?;
    println!("Part 2: {}", answer);

    Ok(())
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<&str> for Direction {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => Err(anyhow!("failed to parse '{}'", value)),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    repeat: usize,
}

impl TryFrom<&str> for Instruction {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (a, b) = value
            .split_once(' ')
            .with_context(|| format!("failed to split '{}'", value))?;
        Ok(Instruction {
            direction: a.try_into()?,
            repeat: b.parse()?,
        })
    }
}

fn parse(input: &str) -> Result<Vec<Instruction>> {
    let mut instructions = Vec::new();
    for line in input.lines() {
        instructions.push(line.try_into()?);
    }
    Ok(instructions)
}

#[derive(Debug, PartialEq, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn move_in_dir(&mut self, dir: Direction) {
        match dir {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        };
    }

    fn follow(&mut self, other: &Position) {
        let delta_x = other.x.abs_diff(self.x);
        let delta_y = other.y.abs_diff(self.y);

        if delta_x == 0 && delta_y > 1 {
            if self.y < other.y {
                self.y += 1;
            } else {
                self.y -= 1;
            }
        } else if delta_x > 1 && delta_y == 0 {
            if self.x < other.x {
                self.x += 1;
            } else {
                self.x -= 1;
            }
        } else if delta_x > 1 || delta_y > 1 {
            match self.x.cmp(&other.x) {
                std::cmp::Ordering::Less => {
                    self.x += 1;
                    match self.y.cmp(&other.y) {
                        std::cmp::Ordering::Less => {
                            self.y += 1;
                        }
                        std::cmp::Ordering::Equal => {}
                        std::cmp::Ordering::Greater => {
                            self.y -= 1;
                        }
                    }
                }
                std::cmp::Ordering::Equal => {}
                std::cmp::Ordering::Greater => {
                    self.x -= 1;
                    match self.y.cmp(&other.y) {
                        std::cmp::Ordering::Less => {
                            self.y += 1;
                        }
                        std::cmp::Ordering::Equal => {}
                        std::cmp::Ordering::Greater => {
                            self.y -= 1;
                        }
                    }
                }
            };
        }
    }
}

fn part_x<const N: usize>(input: &str) -> Result<usize> {
    let instructions = parse(input)?;
    let mut knots = vec![Position { x: 0, y: 0 }; N];
    let mut visited = BTreeSet::from_iter([(knots[0].x, knots[0].y)]);
    for instr in instructions {
        for _ in 0..instr.repeat {
            knots[0].move_in_dir(instr.direction);
            for i in 1..N {
                // slice.get_many_mut (currently a nightly-only experimental API)
                // would two references into the vector, avoiding the clone
                let head = knots[i - 1].clone();
                knots[i].follow(&head);
            }
            let last = &knots[N - 1];
            visited.insert((last.x, last.y));
        }
    }
    Ok(visited.len())
}

fn part_one(input: &str) -> Result<usize> {
    part_x::<2>(input)
}

fn part_two(input: &str) -> Result<usize> {
    part_x::<10>(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_position() {
        let mut head = Position { x: 0, y: 0 };
        let mut tail = Position { ..head };

        // R 4
        head.move_in_dir(Direction::Right);
        tail.follow(&head);
        assert_eq!(head, Position { x: 1, y: 0 });
        assert_eq!(tail, Position { x: 0, y: 0 });

        head.move_in_dir(Direction::Right);
        tail.follow(&head);
        assert_eq!(head, Position { x: 2, y: 0 });
        assert_eq!(tail, Position { x: 1, y: 0 });

        head.move_in_dir(Direction::Right);
        tail.follow(&head);
        assert_eq!(head, Position { x: 3, y: 0 });
        assert_eq!(tail, Position { x: 2, y: 0 });

        head.move_in_dir(Direction::Right);
        tail.follow(&head);
        assert_eq!(head, Position { x: 4, y: 0 });
        assert_eq!(tail, Position { x: 3, y: 0 });

        // U 4
        head.move_in_dir(Direction::Up);
        tail.follow(&head);
        assert_eq!(head, Position { x: 4, y: -1 });
        assert_eq!(tail, Position { x: 3, y: 0 });

        head.move_in_dir(Direction::Up);
        tail.follow(&head);
        assert_eq!(head, Position { x: 4, y: -2 });
        assert_eq!(tail, Position { x: 4, y: -1 });

        head.move_in_dir(Direction::Up);
        tail.follow(&head);
        assert_eq!(head, Position { x: 4, y: -3 });
        assert_eq!(tail, Position { x: 4, y: -2 });

        head.move_in_dir(Direction::Up);
        tail.follow(&head);
        assert_eq!(head, Position { x: 4, y: -4 });
        assert_eq!(tail, Position { x: 4, y: -3 });

        // L 3
        head.move_in_dir(Direction::Left);
        tail.follow(&head);
        assert_eq!(head, Position { x: 3, y: -4 });
        assert_eq!(tail, Position { x: 4, y: -3 });

        head.move_in_dir(Direction::Left);
        tail.follow(&head);
        assert_eq!(head, Position { x: 2, y: -4 });
        assert_eq!(tail, Position { x: 3, y: -4 });

        head.move_in_dir(Direction::Left);
        tail.follow(&head);
        assert_eq!(head, Position { x: 1, y: -4 });
        assert_eq!(tail, Position { x: 2, y: -4 });

        // D 1
        head.move_in_dir(Direction::Down);
        tail.follow(&head);
        assert_eq!(head, Position { x: 1, y: -3 });
        assert_eq!(tail, Position { x: 2, y: -4 });

        // R 4
        head.move_in_dir(Direction::Right);
        tail.follow(&head);
        assert_eq!(head, Position { x: 2, y: -3 });
        assert_eq!(tail, Position { x: 2, y: -4 });

        head.move_in_dir(Direction::Right);
        tail.follow(&head);
        assert_eq!(head, Position { x: 3, y: -3 });
        assert_eq!(tail, Position { x: 2, y: -4 });

        head.move_in_dir(Direction::Right);
        tail.follow(&head);
        assert_eq!(head, Position { x: 4, y: -3 });
        assert_eq!(tail, Position { x: 3, y: -3 });

        head.move_in_dir(Direction::Right);
        tail.follow(&head);
        assert_eq!(head, Position { x: 5, y: -3 });
        assert_eq!(tail, Position { x: 4, y: -3 });
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 13);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 1);
        assert_eq!(
            part_two("R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20").unwrap(),
            36
        );
    }
}
