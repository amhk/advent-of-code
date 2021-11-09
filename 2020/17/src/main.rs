use std::collections::HashSet;
use std::hash::Hash;
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

trait Point: PartialEq + Eq + Copy + Clone + Hash {
    fn new(x: i32, y: i32) -> Self;

    fn neighbours(&self) -> Vec<Self>
    where
        Self: Sized;

    fn self_and_neighbours(&self) -> Vec<Self>
    where
        Self: Sized + Copy,
    {
        let mut v = self.neighbours();
        v.push(*self);
        v
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point3d {
    x: i32,
    y: i32,
    z: i32,
}

impl Point for Point3d {
    fn new(x: i32, y: i32) -> Self {
        Point3d { x, y, z: 0 }
    }

    fn neighbours(&self) -> Vec<Self> {
        let mut v = Vec::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    if !(dx == 0 && dy == 0 && dz == 0) {
                        v.push(Self {
                            x: self.x + dx,
                            y: self.y + dy,
                            z: self.z + dz,
                        });
                    }
                }
            }
        }
        v
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point4d {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl Point for Point4d {
    fn new(x: i32, y: i32) -> Self {
        Point4d { x, y, z: 0, w: 0 }
    }

    fn neighbours(&self) -> Vec<Self> {
        let mut v = Vec::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    for dw in -1..=1 {
                        if !(dx == 0 && dy == 0 && dz == 0 && dw == 0) {
                            v.push(Self {
                                x: self.x + dx,
                                y: self.y + dy,
                                z: self.z + dz,
                                w: self.w + dw,
                            });
                        }
                    }
                }
            }
        }
        v
    }
}

struct Space<T>
where
    T: Point,
{
    points: HashSet<T>,
}

impl<T> Space<T>
where
    T: Point,
{
    fn new() -> Space<T> {
        Space {
            points: HashSet::new(),
        }
    }

    fn step(&mut self) {
        let mut copy: HashSet<T> = HashSet::new();

        let potential = self
            .points
            .iter()
            .flat_map(|p| p.self_and_neighbours())
            .collect::<HashSet<_>>();

        for p in potential.iter() {
            let n = p
                .neighbours()
                .iter()
                .filter(|p| self.points.contains(p))
                .count();
            if self.points.contains(p) {
                if n == 2 || n == 3 {
                    copy.insert(*p);
                }
            } else if n == 3 {
                copy.insert(*p);
            }
        }

        self.points = copy;
    }
}

impl<T> FromStr for Space<T>
where
    T: Point,
{
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut space = Space::new();
        for (y, line) in input.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                match ch {
                    '#' => {
                        space.points.insert(T::new(x as i32, y as i32));
                    }
                    '.' => {}
                    _ => return Err(Error::BadInput),
                }
            }
        }
        Ok(space)
    }
}

fn part_one(input: &str) -> Result<usize, Error> {
    let mut space = Space::<Point3d>::from_str(input)?;
    for _ in 0..6 {
        space.step();
    }
    Ok(space.points.len())
}

fn part_two(input: &str) -> Result<usize, Error> {
    let mut space = Space::<Point4d>::from_str(input)?;
    for _ in 0..6 {
        space.step();
    }
    Ok(space.points.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_neighbours() {
        let p = Point3d { x: 0, y: 0, z: 0 };
        assert_eq!(p.self_and_neighbours().len(), 3_usize.pow(3));
        assert_eq!(p.neighbours().len(), 3_usize.pow(3) - 1);

        let p = Point4d {
            x: 0,
            y: 0,
            z: 0,
            w: 0,
        };
        assert_eq!(p.self_and_neighbours().len(), 3_usize.pow(4));
        assert_eq!(p.neighbours().len(), 3_usize.pow(4) - 1);
    }

    #[test]
    fn test_parse_input() {
        let space = Space::<Point3d>::from_str(INPUT).unwrap();
        assert_eq!(space.points.len(), 5);
        assert!(space.points.contains(&Point3d { x: 1, y: 0, z: 0 }));
    }

    #[test]
    fn test_part_one() {
        fn count(space: &Space<Point3d>, z: i32) -> usize {
            space.points.iter().filter(|p| p.z == z).count()
        }

        // no cycles
        let mut space = Space::<Point3d>::from_str(INPUT).unwrap();
        assert_eq!(space.points.len(), 5);

        // after 1 cycle
        space.step();
        assert_eq!(count(&space, -2), 0);
        assert_eq!(count(&space, -1), 3);
        assert_eq!(count(&space, 0), 5);
        assert_eq!(count(&space, 1), 3);
        assert_eq!(count(&space, 2), 0);

        // after 2 cycles
        space.step();
        assert_eq!(count(&space, -2), 1);
        assert_eq!(count(&space, -1), 5);
        assert_eq!(count(&space, 0), 9);
        assert_eq!(count(&space, 1), 5);
        assert_eq!(count(&space, 2), 1);

        // after 3 cycles
        space.step();
        assert_eq!(count(&space, -2), 5);
        assert_eq!(count(&space, -1), 10);
        assert_eq!(count(&space, 0), 8);
        assert_eq!(count(&space, 1), 10);
        assert_eq!(count(&space, 2), 5);

        assert_eq!(part_one(INPUT), Ok(112));
    }

    #[test]
    fn test_part_two() {
        fn count(space: &Space<Point4d>, z: i32, w: i32) -> usize {
            space.points.iter().filter(|p| p.z == z && p.w == w).count()
        }

        // no cycles
        let mut space = Space::<Point4d>::from_str(INPUT).unwrap();
        assert_eq!(space.points.len(), 5);

        // after 1 cycle
        space.step();
        assert_eq!(count(&space, -1, -1), 3);
        assert_eq!(count(&space, 0, -1), 3);
        assert_eq!(count(&space, 1, -1), 3);
        assert_eq!(count(&space, -1, 0), 3);
        assert_eq!(count(&space, 0, 0), 5);
        assert_eq!(count(&space, 1, 0), 3);
        assert_eq!(count(&space, -1, 1), 3);
        assert_eq!(count(&space, 0, 1), 3);
        assert_eq!(count(&space, 1, 1), 3);

        assert_eq!(part_two(INPUT), Ok(848));
    }
}
