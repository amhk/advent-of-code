use regex::{Match, Regex};
use std::collections::BTreeSet;
use std::ops::Sub;

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

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
struct Cuboid {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    min_z: i32,
    max_z: i32,
}

impl Cuboid {
    fn new(min_x: i32, max_x: i32, min_y: i32, max_y: i32, min_z: i32, max_z: i32) -> Cuboid {
        debug_assert!(min_x <= max_x, "min_x={} max_x={}", min_x, max_x);
        debug_assert!(min_y <= max_y, "min_y={} max_y={}", min_y, max_y);
        debug_assert!(min_z <= max_z, "min_z={} max_z={}", min_z, max_z);
        Cuboid {
            min_x,
            max_x,
            min_y,
            max_y,
            min_z,
            max_z,
        }
    }

    fn volume(&self) -> usize {
        (1 + self.max_x - self.min_x) as usize
            * (1 + self.max_y - self.min_y) as usize
            * (1 + self.max_z - self.min_z) as usize
    }

    fn are_disjunct(&self, other: &Self) -> bool {
        self.max_x < other.min_x
            || self.min_x > other.max_x
            || self.max_y < other.min_y
            || self.min_y > other.max_y
            || self.max_z < other.min_z
            || self.min_z > other.max_z
    }
}

impl Sub for Cuboid {
    type Output = BTreeSet<Cuboid>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.are_disjunct(&rhs) {
            return BTreeSet::from_iter([self]);
        }

        let mut pieces = BTreeSet::new();
        let overlap = Cuboid::new(
            self.min_x.max(rhs.min_x),
            self.max_x.min(rhs.max_x),
            self.min_y.max(rhs.min_y),
            self.max_y.min(rhs.max_y),
            self.min_z.max(rhs.min_z),
            self.max_z.min(rhs.max_z),
        );

        // slice off x plane
        if self.min_x < overlap.min_x {
            pieces.insert(Cuboid {
                max_x: overlap.min_x - 1,
                ..self
            });
        }
        if self.max_x > overlap.max_x {
            pieces.insert(Cuboid {
                min_x: overlap.max_x + 1,
                ..self
            });
        }

        // slice off y plane
        if self.min_y < overlap.min_y {
            pieces.insert(Cuboid {
                min_x: overlap.min_x,
                max_x: overlap.max_x,
                max_y: overlap.min_y - 1,
                ..self
            });
        }
        if self.max_y > overlap.max_y {
            pieces.insert(Cuboid {
                min_x: overlap.min_x,
                max_x: overlap.max_x,
                min_y: overlap.max_y + 1,
                ..self
            });
        }

        // slice off z plane
        if self.min_z < overlap.min_z {
            pieces.insert(Cuboid {
                min_x: overlap.min_x,
                max_x: overlap.max_x,
                min_y: overlap.min_y,
                max_y: overlap.max_y,
                max_z: overlap.min_z - 1,
                ..self
            });
        }
        if self.max_z > overlap.max_z {
            pieces.insert(Cuboid {
                min_x: overlap.min_x,
                max_x: overlap.max_x,
                min_y: overlap.min_y,
                max_y: overlap.max_y,
                min_z: overlap.max_z + 1,
                ..self
            });
        }

        debug_assert!(
            pieces.iter().map(|c| c.volume()).sum::<usize>() == self.volume() - overlap.volume()
        );
        pieces
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Instruction {
    On,
    Off,
}

impl TryFrom<&str> for Instruction {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "on" => Ok(Instruction::On),
            "off" => Ok(Instruction::Off),
            _ => Err(Error::BadInput),
        }
    }
}

#[derive(Debug, Clone)]
struct Step {
    instruction: Instruction,
    cuboid: Cuboid,
}

fn parse_input(input: &str) -> Result<Vec<Step>, Error> {
    fn parse_i32(m: Option<Match>) -> Result<i32, Error> {
        m.ok_or(Error::BadInput)?
            .as_str()
            .parse()
            .map_err(|_| Error::BadInput)
    }

    let re = Regex::new(r"(on|off) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)")
        .unwrap();
    let mut steps = vec![];
    for line in input.lines() {
        let caps = re.captures(line).ok_or(Error::BadInput)?;
        let instruction = caps.get(1).unwrap().as_str().try_into()?;
        let x = (parse_i32(caps.get(2))?, parse_i32(caps.get(3))?);
        let y = (parse_i32(caps.get(4))?, parse_i32(caps.get(5))?);
        let z = (parse_i32(caps.get(6))?, parse_i32(caps.get(7))?);
        let cuboid = Cuboid::new(
            x.0.min(x.1),
            x.0.max(x.1),
            y.0.min(y.1),
            y.0.max(y.1),
            z.0.min(z.1),
            z.0.max(z.1),
        );
        steps.push(Step {
            instruction,
            cuboid,
        });
    }
    if let Some(first) = steps.first() {
        if first.instruction != Instruction::On {
            return Err(Error::BadInput);
        }
    }
    Ok(steps)
}

fn perform_steps(steps: Vec<Step>) -> BTreeSet<Cuboid> {
    let mut cuboids: BTreeSet<Cuboid> = BTreeSet::new();
    for Step {
        instruction,
        cuboid,
    } in steps.into_iter()
    {
        if cuboids.is_empty() && instruction == Instruction::On {
            cuboids.insert(cuboid);
        } else {
            let mut next: BTreeSet<Cuboid> = BTreeSet::new();
            for c in cuboids.into_iter() {
                next.append(&mut (c - cuboid));
            }
            cuboids = next;
            if instruction == Instruction::On {
                cuboids.insert(cuboid);
            }
        }
    }
    cuboids
}

fn part_one(input: &str) -> Result<usize, Error> {
    let steps = parse_input(input)?
        .into_iter()
        .filter(|step| {
            step.cuboid.min_x >= -50
                && step.cuboid.max_x <= 50
                && step.cuboid.min_y >= -50
                && step.cuboid.max_y <= 50
                && step.cuboid.min_z >= -50
                && step.cuboid.max_z <= 50
        })
        .collect();
    let cuboids = perform_steps(steps);
    Ok(cuboids.into_iter().map(|c| c.volume()).sum())
}

fn part_two(input: &str) -> Result<usize, Error> {
    let steps = parse_input(input)?;
    let cuboids = perform_steps(steps);
    Ok(cuboids.into_iter().map(|c| c.volume()).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = include_str!("test-input-part-one.txt");
    const INPUT2: &str = include_str!("test-input-part-two.txt");

    #[test]
    fn test_parse_input() {
        let steps = parse_input(INPUT1).unwrap();
        assert_eq!(steps.len(), 22);
    }

    #[test]
    fn test_cuboid_volume() {
        assert_eq!(Cuboid::new(0, 0, 0, 0, 0, 0).volume(), 1);
        assert_eq!(Cuboid::new(0, 1, 0, 1, 0, 1).volume(), 2 * 2 * 2);
        assert_eq!(Cuboid::new(-1, 0, 0, 1, -1, 0).volume(), 2 * 2 * 2);
        assert_eq!(Cuboid::new(-1, 1, -1, 1, -1, 1).volume(), 3 * 3 * 3);
    }

    #[test]
    fn test_cuboid_sub() {
        assert_eq!(
            Cuboid::new(0, 0, 0, 0, 0, 0) - Cuboid::new(1, 1, 1, 1, 1, 1),
            BTreeSet::from_iter([Cuboid::new(0, 0, 0, 0, 0, 0)])
        );
        assert_eq!(
            (Cuboid::new(0, 1, 0, 1, 0, 1) - Cuboid::new(1, 1, 1, 1, 1, 1))
                .into_iter()
                .map(|c| c.volume())
                .sum::<usize>(),
            7
        );
        assert_eq!(
            (Cuboid::new(0, 1, 0, 1, 0, 1) - Cuboid::new(1, 2, 1, 3, 1, 4))
                .into_iter()
                .map(|c| c.volume())
                .sum::<usize>(),
            7
        );
        assert_eq!(
            (Cuboid::new(0, 10, 0, 10, 0, 10) - Cuboid::new(2, 3, 4, 5, 6, 7))
                .into_iter()
                .map(|c| c.volume())
                .sum::<usize>(),
            (11 * 11 * 11) - (2 * 2 * 2)
        );
        assert_eq!(
            (Cuboid::new(0, 0, 0, 1, 2, 2) - Cuboid::new(0, 0, 0, 0, 0, 0))
                .into_iter()
                .map(|c| c.volume())
                .sum::<usize>(),
            Cuboid::new(0, 0, 0, 1, 2, 2).volume()
        );
    }

    #[test]
    fn test_perform_steps() {
        let steps = [
            Step {
                instruction: Instruction::On,
                cuboid: Cuboid::new(0, 1, 0, 1, 0, 1),
            },
            Step {
                instruction: Instruction::Off,
                cuboid: Cuboid::new(1, 1, 1, 1, 1, 1),
            },
            Step {
                instruction: Instruction::On,
                cuboid: Cuboid::new(-10, -10, -10, -10, -10, -10),
            },
        ];

        let cuboids = perform_steps(steps.iter().take(1).cloned().collect());
        assert_eq!(cuboids.into_iter().map(|c| c.volume()).sum::<usize>(), 8);

        let cuboids = perform_steps(steps.iter().take(2).cloned().collect());
        assert_eq!(cuboids.into_iter().map(|c| c.volume()).sum::<usize>(), 7);

        let cuboids = perform_steps(steps.iter().take(3).cloned().collect());
        assert_eq!(cuboids.into_iter().map(|c| c.volume()).sum::<usize>(), 8);
    }

    #[test]
    fn test_small_example() {
        let input = r"on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10";
        let steps = parse_input(input).unwrap();

        let cuboids = perform_steps(steps.iter().take(1).cloned().collect());
        assert_eq!(cuboids.into_iter().map(|c| c.volume()).sum::<usize>(), 27);

        let cuboids = perform_steps(steps.iter().take(2).cloned().collect());
        assert_eq!(
            cuboids.into_iter().map(|c| c.volume()).sum::<usize>(),
            27 + 19
        );

        let cuboids = perform_steps(steps.iter().take(3).cloned().collect());
        assert_eq!(
            cuboids.into_iter().map(|c| c.volume()).sum::<usize>(),
            27 + 19 - 8
        );

        let cuboids = perform_steps(steps.iter().take(4).cloned().collect());
        assert_eq!(
            cuboids.into_iter().map(|c| c.volume()).sum::<usize>(),
            27 + 19 - 8 + 1
        );
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT1), Ok(590_784));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT2), Ok(2_758_514_936_282_235));
    }
}
