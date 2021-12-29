use regex::Regex;
use std::collections::{BTreeMap, BTreeSet};
use std::ops::Sub;

fn main() {
    let input = include_str!("input.txt");

    let (scanners, beacons) = part_one(input).expect("no solution for part one");
    println!("part 1: {}", beacons.len());

    let answer = part_two(&scanners).expect("no solution for part two");
    println!("part 2: {}", answer);
}

#[derive(Debug, PartialEq)]
enum Error {
    BadInput,
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Point {
        Point { x, y, z }
    }

    fn rotate(&self) -> impl Iterator<Item = Point> {
        let mut v = vec![];
        let x = self.x;
        let y = self.y;
        let z = self.z;

        // top: x
        v.push(Point::new(z, -y, x));
        v.push(Point::new(-y, -z, x));
        v.push(Point::new(-z, y, x));
        v.push(Point::new(y, z, x));

        // top: -x
        v.push(Point::new(z, y, -x));
        v.push(Point::new(y, -z, -x));
        v.push(Point::new(-z, -y, -x));
        v.push(Point::new(-y, z, -x));

        // top: y
        v.push(Point::new(x, -z, y));
        v.push(Point::new(-z, -x, y));
        v.push(Point::new(-x, z, y));
        v.push(Point::new(z, x, y));

        // top: -y
        v.push(Point::new(x, z, -y));
        v.push(Point::new(z, -x, -y));
        v.push(Point::new(-x, -z, -y));
        v.push(Point::new(-z, x, -y));

        // top: z
        v.push(Point::new(x, y, z));
        v.push(Point::new(y, -x, z));
        v.push(Point::new(-x, -y, z));
        v.push(Point::new(-y, x, z));

        // top: -z
        v.push(Point::new(x, -y, -z));
        v.push(Point::new(-y, -x, -z));
        v.push(Point::new(-x, y, -z));
        v.push(Point::new(y, x, -z));

        v.into_iter()
    }

    fn translate(&mut self, diff: &Point) {
        self.x += diff.x;
        self.y += diff.y;
        self.z += diff.z;
    }
}

impl Sub for &Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[derive(Debug)]
struct Scanner {
    #[allow(dead_code)]
    id: u32,
    beacons: Vec<Point>,
}

impl Scanner {
    fn rotate(&self) -> Vec<Vec<Point>> {
        let beacons: Vec<_> = self
            .beacons
            .iter()
            .map(|point| point.rotate().collect::<Vec<_>>())
            .collect();
        let mut rotated = vec![];
        for i in 0..24 {
            let mut v = vec![];
            for b in &beacons {
                v.push(b[i].clone());
            }
            rotated.push(v);
        }
        rotated
    }
}

fn place_scanners_and_beacons(
    scanners: &[Scanner],
) -> Result<(BTreeSet<Point>, BTreeSet<Point>), Error> {
    let first = scanners.first().ok_or(Error::BadInput)?;
    let mut global_beacons = BTreeSet::from_iter(first.beacons.iter().cloned());
    let mut global_scanners = BTreeSet::new();
    global_scanners.insert(Point::new(0, 0, 0));
    let mut worklist = vec![];
    let mut discarded = Vec::from_iter(&scanners[1..]);
    let mut stuck = false;
    while !discarded.is_empty() {
        if stuck {
            // failed to remove anything from the worklist during last iteration
            return Err(Error::BadInput);
        }
        worklist.append(&mut discarded);
        'search: while let Some(scanner) = worklist.pop() {
            for beacons in scanner.rotate() {
                let mut diffs: BTreeMap<Point, usize> = BTreeMap::new();
                for a in &global_beacons {
                    for b in &beacons {
                        let diff = a - b;
                        *diffs.entry(diff).or_default() += 1;
                    }
                }
                let (diff, count) = diffs
                    .into_iter()
                    .max_by_key(|(_, value)| *value)
                    .ok_or(Error::BadInput)?;
                if count >= 12 {
                    for mut b in beacons {
                        b.translate(&diff);
                        global_beacons.insert(b);
                    }
                    global_scanners.insert(diff);
                    stuck = false;
                    continue 'search;
                }
            }
            discarded.push(scanner);
        }
    }
    Ok((global_scanners, global_beacons))
}

fn parse_input(input: &str) -> Result<Vec<Scanner>, Error> {
    let re_scanner = Regex::new(r"--- scanner (\d+) ---((\n[-\d,]+)*)").unwrap();
    let re_beacon = Regex::new(r"(-?\d+),(-?\d+),(-?\d+)").unwrap();
    let mut scanners = vec![];
    for cap in re_scanner.captures_iter(input) {
        let id: u32 = cap.get(1).unwrap().as_str().parse().unwrap();
        let mut beacons = vec![];
        for cap in re_beacon.captures_iter(cap.get(2).unwrap().as_str()) {
            beacons.push(Point {
                x: cap.get(1).unwrap().as_str().parse().unwrap(),
                y: cap.get(2).unwrap().as_str().parse().unwrap(),
                z: cap.get(3).unwrap().as_str().parse().unwrap(),
            });
        }
        scanners.push(Scanner { id, beacons });
    }
    Ok(scanners)
}

fn part_one(input: &str) -> Result<(BTreeSet<Point>, BTreeSet<Point>), Error> {
    let scanners = parse_input(input)?;
    let (scanners, beacons) = place_scanners_and_beacons(&scanners)?;
    Ok((scanners, beacons))
}

fn part_two(points: &BTreeSet<Point>) -> Result<i32, Error> {
    points
        .iter()
        .filter_map(|a| {
            points
                .iter()
                .map(|b| {
                    let diff = a - b;
                    diff.x.abs() + diff.y.abs() + diff.z.abs()
                })
                .max()
        })
        .max()
        .ok_or(Error::BadInput)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_parse_input() {
        let scanners = parse_input(INPUT).unwrap();
        assert_eq!(scanners.len(), 5);
        assert_eq!(scanners[0].beacons.len(), 25);
        assert_eq!(scanners[1].beacons.len(), 25);
        assert_eq!(scanners[2].beacons.len(), 26);
        assert_eq!(scanners[3].beacons.len(), 25);
        assert_eq!(scanners[4].beacons.len(), 26);
    }

    #[test]
    fn test_point() {
        let p = Point::new(1, 2, 3);
        assert_eq!(p.rotate().count(), 24);
        let unique = BTreeSet::from_iter(p.rotate());
        assert_eq!(unique.len(), 24);

        assert_eq!(
            &Point::new(0, 0, 0) - &Point::new(1, 2, 3),
            Point::new(-1, -2, -3),
        );
    }

    #[test]
    fn test_scanner() {
        let scanner = Scanner {
            id: 0,
            beacons: vec![
                Point::new(-1, -1, 1),
                Point::new(-2, -2, 2),
                Point::new(-3, -3, 3),
                Point::new(-2, -3, 1),
                Point::new(5, 6, -4),
                Point::new(8, 0, 7),
            ],
        };
        let rotations = scanner.rotate();
        assert_eq!(rotations.len(), 24);

        let unique = BTreeSet::from_iter(scanner.rotate());
        assert_eq!(unique.len(), 24);

        assert!(rotations.contains(&vec![
            Point::new(-1, -1, 1),
            Point::new(-2, -2, 2),
            Point::new(-3, -3, 3),
            Point::new(-2, -3, 1),
            Point::new(5, 6, -4),
            Point::new(8, 0, 7),
        ]));
        assert!(rotations.contains(&vec![
            Point::new(1, -1, 1),
            Point::new(2, -2, 2),
            Point::new(3, -3, 3),
            Point::new(2, -1, 3),
            Point::new(-5, 4, -6),
            Point::new(-8, -7, 0),
        ]));
        assert!(rotations.contains(&vec![
            Point::new(-1, -1, -1),
            Point::new(-2, -2, -2),
            Point::new(-3, -3, -3),
            Point::new(-1, -3, -2),
            Point::new(4, 6, 5),
            Point::new(-7, 0, 8),
        ]));
        assert!(rotations.contains(&vec![
            Point::new(1, 1, -1),
            Point::new(2, 2, -2),
            Point::new(3, 3, -3),
            Point::new(1, 3, -2),
            Point::new(-4, -6, 5),
            Point::new(7, 0, 8),
        ]));
        assert!(rotations.contains(&vec![
            Point::new(1, 1, 1),
            Point::new(2, 2, 2),
            Point::new(3, 3, 3),
            Point::new(3, 1, 2),
            Point::new(-6, -4, -5),
            Point::new(0, 7, -8),
        ]));
    }

    #[test]
    fn test_part_one() {
        let (_, beacons) = part_one(INPUT).unwrap();
        assert_eq!(beacons.len(), 79);
    }

    #[test]
    fn test_part_two() {
        let (scanners, _) = part_one(INPUT).unwrap();
        assert_eq!(part_two(&scanners), Ok(3621));
    }
}
