use std::collections::{BTreeMap, BTreeSet};

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 6902)?;
    aoc::run!(part_two(input), 7697)?;
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
enum Energized {
    Yes(BTreeSet<Direction>),
    No,
}

#[derive(Debug, PartialEq, Eq)]
enum CellType {
    Empty,            // .
    ReflectSlash,     // /
    ReflectBackslash, // \
    SplitHorizontal,  // -
    SplitVertical,    // |
}

impl TryFrom<char> for CellType {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Empty),
            '/' => Ok(Self::ReflectSlash),
            '\\' => Ok(Self::ReflectBackslash),
            '-' => Ok(Self::SplitHorizontal),
            '|' => Ok(Self::SplitVertical),
            _ => Err(anyhow!("unexpected value {value}")),
        }
    }
}

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    cells: BTreeMap<XY, (CellType, Energized)>,
}

impl Grid {
    fn is_inside(&self, xy: &XY) -> bool {
        xy.x >= 0 && xy.y >= 0 && xy.x < self.width as i32 && xy.y < self.height as i32
    }

    fn set_energized(&mut self, xy: &XY, dir: Direction) {
        let (_, energized) = self.cells.get_mut(xy).expect("out of bounds");
        match energized {
            Energized::Yes(set) => {
                set.insert(dir);
            }
            Energized::No => {
                *energized = Energized::Yes(BTreeSet::from([dir]));
            }
        };
    }

    fn get(&self, xy: &XY) -> &(CellType, Energized) {
        self.cells.get(xy).expect("out of bounds")
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone)]
struct XY {
    x: i32,
    y: i32,
}

impl XY {
    fn advance(&mut self, dir: Direction) {
        match dir {
            Direction::North => self.y -= 1,
            Direction::East => self.x += 1,
            Direction::South => self.y += 1,
            Direction::West => self.x -= 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_right(&mut self) {
        *self = match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    fn turn_left(&mut self) {
        self.turn_right();
        self.turn_right();
        self.turn_right();
    }

    fn is_horizontal(&self) -> bool {
        self == &Self::East || self == &Self::West
    }

    fn is_vertical(&self) -> bool {
        !self.is_horizontal()
    }
}

#[derive(Debug, Clone)]
struct Photon {
    xy: XY,
    direction: Direction,
}

fn parse(input: &str) -> Result<Grid> {
    let mut cells = BTreeMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let cell_type: CellType = ch.try_into()?;
            cells.insert(
                XY {
                    x: x as i32,
                    y: y as i32,
                },
                (cell_type, Energized::No),
            );
        }
    }
    Ok(Grid {
        cells,
        height: input.lines().count(),
        width: input.find('\n').ok_or_else(|| anyhow!("bad input"))?,
    })
}

fn calculate_energized_tiles(input: &str, initial: Photon) -> Result<usize> {
    let mut grid = parse(input)?;
    let mut photons = vec![initial];
    while !photons.is_empty() {
        let mut new_photons = vec![];
        for mut photon in photons.into_iter() {
            photon.xy.advance(photon.direction);
            if !grid.is_inside(&photon.xy) {
                continue;
            }

            let (_, energized) = grid.get(&photon.xy);
            if let Energized::Yes(set) = energized {
                if set.contains(&photon.direction) {
                    // a photon already passed this cell, in this direction: no need to repeat that
                    // photon's path
                    continue;
                }
            };
            grid.set_energized(&photon.xy, photon.direction);

            let (cell_type, _) = grid.get(&photon.xy);

            match cell_type {
                CellType::Empty => {
                    new_photons.push(photon);
                }
                CellType::ReflectSlash => {
                    if photon.direction.is_vertical() {
                        photon.direction.turn_right();
                    } else {
                        photon.direction.turn_left();
                    }
                    new_photons.push(photon);
                }
                CellType::ReflectBackslash => {
                    if photon.direction.is_horizontal() {
                        photon.direction.turn_right();
                    } else {
                        photon.direction.turn_left();
                    }
                    new_photons.push(photon);
                }
                CellType::SplitHorizontal => {
                    if photon.direction.is_horizontal() {
                        new_photons.push(photon);
                    } else {
                        let mut clone = photon.clone();
                        clone.direction.turn_left();
                        new_photons.push(clone);

                        photon.direction.turn_right();
                        new_photons.push(photon);
                    }
                }
                CellType::SplitVertical => {
                    if photon.direction.is_vertical() {
                        new_photons.push(photon);
                    } else {
                        let mut clone = photon.clone();
                        clone.direction.turn_left();
                        new_photons.push(clone);

                        photon.direction.turn_right();
                        new_photons.push(photon);
                    }
                }
            }
        }
        photons = new_photons;
    }

    Ok(grid
        .cells
        .values()
        .map(|(_, energized)| energized)
        .filter(|energized| **energized != Energized::No)
        .count())
}

fn part_one(input: &str) -> Result<usize> {
    calculate_energized_tiles(
        input,
        Photon {
            xy: XY { x: -1, y: 0 },
            direction: Direction::East,
        },
    )
}

fn part_two(input: &str) -> Result<usize> {
    let grid = parse(input)?;
    let mut max = 0;
    for x in -1..=grid.width as i32 {
        max = max.max(calculate_energized_tiles(
            input,
            Photon {
                xy: XY { x, y: -1 },
                direction: Direction::South,
            },
        )?);
        max = max.max(calculate_energized_tiles(
            input,
            Photon {
                xy: XY {
                    x,
                    y: grid.height as i32,
                },
                direction: Direction::North,
            },
        )?);
    }
    for y in -1..=grid.height as i32 {
        max = max.max(calculate_energized_tiles(
            input,
            Photon {
                xy: XY { x: -1, y },
                direction: Direction::East,
            },
        )?);
        max = max.max(calculate_energized_tiles(
            input,
            Photon {
                xy: XY {
                    x: grid.width as i32,
                    y,
                },
                direction: Direction::West,
            },
        )?);
    }
    Ok(max)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 46);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 51);
    }
}
