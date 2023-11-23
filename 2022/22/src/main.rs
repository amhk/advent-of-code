use anyhow::{bail, ensure, Context, Result};
use regex::Regex;
use std::collections::HashMap;

fn main() -> Result<()> {
    //     +---+---+
    //     | 0 | 1 |
    //     +---+---+
    //     | 2 |
    // +---+---+
    // | 3 | 4 |
    // +---+---+
    // | 5 |
    // +---+
    let offsets = [(1, 0), (2, 0), (1, 1), (0, 2), (1, 2), (0, 3)];
    let input = include_str!("input.txt");

    aoc::run!(
        solve(
            input,
            50,
            offsets,
            [
                (
                    Warp::new(4, 0),
                    Warp::new(1, 0),
                    Warp::new(2, 0),
                    Warp::new(1, 0),
                ),
                (
                    Warp::new(1, 0),
                    Warp::new(0, 0),
                    Warp::new(1, 0),
                    Warp::new(0, 0),
                ),
                (
                    Warp::new(0, 0),
                    Warp::new(2, 0),
                    Warp::new(4, 0),
                    Warp::new(2, 0),
                ),
                (
                    Warp::new(5, 0),
                    Warp::new(4, 0),
                    Warp::new(5, 0),
                    Warp::new(4, 0),
                ),
                (
                    Warp::new(2, 0),
                    Warp::new(3, 0),
                    Warp::new(0, 0),
                    Warp::new(3, 0),
                ),
                (
                    Warp::new(3, 0),
                    Warp::new(5, 0),
                    Warp::new(3, 0),
                    Warp::new(5, 0),
                ),
            ],
        ),
        80_392
    )?;

    aoc::run!(
        solve(
            input,
            50,
            offsets,
            [
                (
                    Warp::new(5, 3),
                    Warp::new(1, 0),
                    Warp::new(2, 0),
                    Warp::new(3, 2),
                ),
                (
                    Warp::new(5, 0),
                    Warp::new(4, 2),
                    Warp::new(2, 3),
                    Warp::new(0, 0),
                ),
                (
                    Warp::new(0, 0),
                    Warp::new(1, 1),
                    Warp::new(4, 0),
                    Warp::new(3, 1),
                ),
                (
                    Warp::new(2, 3),
                    Warp::new(4, 0),
                    Warp::new(5, 0),
                    Warp::new(0, 2),
                ),
                (
                    Warp::new(2, 0),
                    Warp::new(1, 2),
                    Warp::new(5, 3),
                    Warp::new(3, 0),
                ),
                (
                    Warp::new(3, 0),
                    Warp::new(4, 1),
                    Warp::new(1, 0),
                    Warp::new(0, 1),
                ),
            ],
        ),
        19_534
    )?;

    Ok(())
}

#[derive(Debug)]
struct Warp {
    segment: usize,
    left_rotations: usize,
}

impl Warp {
    fn new(segment: usize, left_rotations: usize) -> Self {
        Self {
            segment,
            left_rotations,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        self.turn_left().turn_left().turn_left()
    }

    fn turn_left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Position {
    segment: usize,
    x: usize,
    y: usize,
    direction: Direction,
}

impl Position {
    fn rotate_left_once(&mut self, side: usize) {
        let new_x = self.y;
        self.y = side - self.x;
        self.x = new_x;
        self.direction = self.direction.turn_left();
    }
}

#[derive(PartialEq)]
enum MapCell {
    Wall,
    Space,
}

struct MapSegment {
    cells: HashMap<(usize, usize), MapCell>,
}

impl MapSegment {
    fn new() -> Self {
        Self {
            cells: HashMap::new(),
        }
    }
}

type Map = [MapSegment; 6];

#[derive(Debug)]
enum Instruction {
    TurnLeft,
    TurnRight,
    Move(usize),
}

type Instructions = Vec<Instruction>;

fn parse(input: &str, side: usize, offsets: [(usize, usize); 6]) -> Result<(Map, Instructions)> {
    let (raw_map, raw_instructions) = input.split_once("\n\n").context("failed to split input")?;

    // map
    let mut map = [
        MapSegment::new(),
        MapSegment::new(),
        MapSegment::new(),
        MapSegment::new(),
        MapSegment::new(),
        MapSegment::new(),
    ];
    let lines: Vec<_> = raw_map.lines().collect();
    for (i, (offset_x, offset_y)) in offsets.iter().enumerate() {
        let segment = &mut map[i];
        for y in 0..side {
            let chars: Vec<_> = lines[y + offset_y * side].chars().collect();
            for x in 0..side {
                let ch = &chars[x + offset_x * side];
                segment.cells.insert(
                    (x, y),
                    match ch {
                        '#' => MapCell::Wall,
                        '.' => MapCell::Space,
                        _ => bail!("unknown char '{}'", ch),
                    },
                );
            }
        }
    }

    // instructions
    let mut instructions = vec![];
    let regex = Regex::new(r"\d+|L|R").unwrap();
    for token in regex.find_iter(raw_instructions) {
        instructions.push(match token.as_str() {
            "L" => Instruction::TurnLeft,
            "R" => Instruction::TurnRight,
            _ => {
                let value: usize = token
                    .as_str()
                    .parse()
                    .context("faield to convert to usize")?;
                Instruction::Move(value)
            }
        });
    }

    Ok((map, instructions))
}

fn solve(
    input: &str,
    side: usize,
    offsets: [(usize, usize); 6],
    warps: [(Warp, Warp, Warp, Warp); 6],
) -> Result<usize> {
    let (map, instructions) = parse(input, side, offsets)?;

    let mut pos = Position {
        segment: 0,
        x: 0,
        y: 0,
        direction: Direction::East,
    };
    ensure!(
        map[pos.segment].cells.get(&(pos.x, pos.y)) == Some(&MapCell::Space),
        "bad starting position"
    );
    for instr in instructions {
        match instr {
            Instruction::TurnLeft => pos.direction = pos.direction.turn_left(),
            Instruction::TurnRight => pos.direction = pos.direction.turn_right(),
            Instruction::Move(steps) => {
                for _ in 0..steps {
                    let new_pos = match pos.direction {
                        Direction::North => {
                            if pos.y > 0 {
                                Position {
                                    segment: pos.segment,
                                    x: pos.x,
                                    y: pos.y - 1,
                                    direction: Direction::North,
                                }
                            } else {
                                let warp = &warps[pos.segment].0;
                                let mut new_pos = Position {
                                    segment: warp.segment,
                                    x: pos.x,
                                    y: side - 1,
                                    direction: pos.direction,
                                };
                                for _ in 0..warp.left_rotations {
                                    new_pos.rotate_left_once(side - 1);
                                }
                                new_pos
                            }
                        }
                        Direction::East => {
                            if pos.x < side - 1 {
                                Position {
                                    segment: pos.segment,
                                    x: pos.x + 1,
                                    y: pos.y,
                                    direction: Direction::East,
                                }
                            } else {
                                let warp = &warps[pos.segment].1;
                                let mut new_pos = Position {
                                    segment: warp.segment,
                                    x: 0,
                                    y: pos.y,
                                    direction: pos.direction,
                                };
                                for _ in 0..warp.left_rotations {
                                    new_pos.rotate_left_once(side - 1);
                                }
                                new_pos
                            }
                        }
                        Direction::South => {
                            if pos.y < side - 1 {
                                Position {
                                    segment: pos.segment,
                                    x: pos.x,
                                    y: pos.y + 1,
                                    direction: Direction::South,
                                }
                            } else {
                                let warp = &warps[pos.segment].2;
                                let mut new_pos = Position {
                                    segment: warp.segment,
                                    x: pos.x,
                                    y: 0,
                                    direction: pos.direction,
                                };
                                for _ in 0..warp.left_rotations {
                                    new_pos.rotate_left_once(side - 1);
                                }
                                new_pos
                            }
                        }
                        Direction::West => {
                            if pos.x > 0 {
                                Position {
                                    segment: pos.segment,
                                    x: pos.x - 1,
                                    y: pos.y,
                                    direction: Direction::West,
                                }
                            } else {
                                let warp = &warps[pos.segment].3;
                                let mut new_pos = Position {
                                    segment: warp.segment,
                                    x: side - 1,
                                    y: pos.y,
                                    direction: pos.direction,
                                };
                                for _ in 0..warp.left_rotations {
                                    new_pos.rotate_left_once(side - 1);
                                }
                                new_pos
                            }
                        }
                    };
                    if map[new_pos.segment].cells[&(new_pos.x, new_pos.y)] == MapCell::Wall {
                        break;
                    }
                    pos = new_pos;
                }
            }
        }
    }

    let score_column = (1 + pos.y + side * offsets[pos.segment].1) * 1000;
    let score_row = (1 + pos.x + side * offsets[pos.segment].0) * 4;
    let score_direction = match pos.direction {
        Direction::North => 3,
        Direction::East => 0,
        Direction::South => 1,
        Direction::West => 2,
    };
    Ok(score_column + score_row + score_direction)
}

#[cfg(test)]
mod tests {
    use super::*;

    //         +---+
    //         | 0 |
    // +---+---+---+
    // | 1 | 2 | 3 |
    // +---+---+---+---+
    //         | 4 | 5 |
    //         +---+---+
    const OFFSETS: [(usize, usize); 6] = [(2, 0), (0, 1), (1, 1), (2, 1), (2, 2), (3, 2)];

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(
            solve(
                INPUT,
                4,
                OFFSETS,
                [
                    // moving N/E/S/W from segment[index]
                    (
                        Warp::new(4, 0),
                        Warp::new(0, 0),
                        Warp::new(3, 0),
                        Warp::new(0, 0)
                    ),
                    (
                        Warp::new(1, 0),
                        Warp::new(2, 0),
                        Warp::new(1, 0),
                        Warp::new(3, 0)
                    ),
                    (
                        Warp::new(2, 0),
                        Warp::new(3, 0),
                        Warp::new(2, 0),
                        Warp::new(1, 0)
                    ),
                    (
                        Warp::new(0, 0),
                        Warp::new(1, 0),
                        Warp::new(4, 0),
                        Warp::new(2, 0)
                    ),
                    (
                        Warp::new(3, 0),
                        Warp::new(5, 0),
                        Warp::new(0, 0),
                        Warp::new(5, 0)
                    ),
                    (
                        Warp::new(5, 0),
                        Warp::new(4, 0),
                        Warp::new(5, 0),
                        Warp::new(4, 0)
                    ),
                ]
            )
            .unwrap(),
            6032
        );
    }

    #[test]
    fn test_part_two() {
        assert_eq!(
            solve(
                INPUT,
                4,
                OFFSETS,
                [
                    // moving N/E/S/W from segment[index]
                    (
                        Warp::new(1, 2),
                        Warp::new(5, 2),
                        Warp::new(3, 0),
                        Warp::new(2, 1)
                    ),
                    (
                        Warp::new(0, 2),
                        Warp::new(2, 0),
                        Warp::new(4, 2),
                        Warp::new(5, 3)
                    ),
                    (
                        Warp::new(0, 3),
                        Warp::new(3, 0),
                        Warp::new(4, 1),
                        Warp::new(1, 0)
                    ),
                    (
                        Warp::new(0, 0),
                        Warp::new(5, 3),
                        Warp::new(4, 0),
                        Warp::new(2, 0)
                    ),
                    (
                        Warp::new(3, 0),
                        Warp::new(5, 0),
                        Warp::new(1, 2),
                        Warp::new(2, 3)
                    ),
                    (
                        Warp::new(3, 1),
                        Warp::new(0, 2),
                        Warp::new(1, 1),
                        Warp::new(4, 0)
                    ),
                ]
            )
            .unwrap(),
            5031
        );
    }

    #[test]
    fn test_rotate_left() {
        let mut p = Position {
            segment: 0,
            x: 1,
            y: 2,
            direction: Direction::East,
        };
        p.rotate_left_once(11);
        assert_eq!(
            p,
            Position {
                segment: 0,
                x: 2,
                y: 10,
                direction: Direction::North,
            }
        );
        p.rotate_left_once(11);
        assert_eq!(
            p,
            Position {
                segment: 0,
                x: 10,
                y: 9,
                direction: Direction::West,
            }
        );
        p.rotate_left_once(11);
        assert_eq!(
            p,
            Position {
                segment: 0,
                x: 9,
                y: 1,
                direction: Direction::South,
            }
        );
        p.rotate_left_once(11);
        assert_eq!(
            p,
            Position {
                segment: 0,
                x: 1,
                y: 2,
                direction: Direction::East,
            }
        );
    }
}
