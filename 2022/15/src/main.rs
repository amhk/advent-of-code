use std::collections::{HashMap, HashSet};

use anyhow::{bail, Context, Result};
use regex::{Captures, Regex};

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    let space = parse(input)?;

    let answer = part_one(&space, 2_000_000).context("no solution for part one")?;
    println!("Part 1: {}", answer);
    assert_eq!(answer, 6275922);

    let answer =
        part_two(&space, (0, 0), (4_000_000, 4_000_000)).context("no solution for part two")?;
    println!("Part 2: {}", answer);
    assert_eq!(answer, 11747175442119);

    Ok(())
}

type XY = (i32, i32);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct InclusiveRange {
    start: i32,
    end: i32,
}

struct Space {
    ranges: HashMap<i32, Vec<InclusiveRange>>,
    objects: HashSet<XY>,
}

impl InclusiveRange {
    fn new(start: i32, end: i32) -> Self {
        if start <= end {
            Self { start, end }
        } else {
            Self { end, start }
        }
    }

    fn len(&self) -> u32 {
        self.start.abs_diff(self.end) + 1
    }

    fn contains(&self, value: i32) -> bool {
        self.start <= value && value <= self.end
    }

    fn merge(self, other: InclusiveRange) -> InclusiveRange {
        let (a, b) = if self.start <= other.start {
            (&self, &other)
        } else {
            (&other, &self)
        };

        // disjunct
        if a.end < b.start {
            panic!("ranges do not intersect");
        }

        // a overlaps b completely
        if a.end >= b.end {
            return a.clone();
        }

        // partial overlap
        InclusiveRange::new(a.start, b.end)
    }

    fn intersects(&self, other: &InclusiveRange) -> bool {
        if self.start <= other.start {
            self.end >= other.start
        } else {
            other.end >= self.start
        }
    }
}

fn merge_all(ranges: &[InclusiveRange]) -> Vec<InclusiveRange> {
    let mut ranges = Vec::from_iter(ranges.iter().cloned());
    let mut change = true;
    while change {
        change = false;
        'top: for i in 0..ranges.len() {
            for j in (i + 1)..ranges.len() {
                if ranges.get(i).unwrap().intersects(ranges.get(j).unwrap()) {
                    let b = ranges.remove(j);
                    let a = ranges.remove(i);
                    let c = a.merge(b);
                    ranges.push(c);
                    change = true;
                    break 'top;
                }
            }
        }
    }
    ranges
}

fn manhattan_distance(a: &XY, b: &XY) -> u32 {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn parse(input: &str) -> Result<Space> {
    fn to_i32(caps: &Captures, index: usize) -> Result<i32> {
        caps.get(index)
            .unwrap()
            .as_str()
            .parse()
            .context("failed to convert to i32")
    }

    let regex =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();
    let mut objects = HashSet::new();
    let mut ranges: HashMap<i32, Vec<_>> = HashMap::new();
    for line in input.lines() {
        let caps = regex.captures(line).context("line does not match regex")?;
        let sensor = (to_i32(&caps, 1)?, to_i32(&caps, 2)?);
        let beacon = (to_i32(&caps, 3)?, to_i32(&caps, 4)?);
        objects.insert(sensor);
        objects.insert(beacon);

        let distance = manhattan_distance(&sensor, &beacon) as i32;
        for y in (sensor.1 - distance)..=(sensor.1 + distance) {
            let offset = distance - y.abs_diff(sensor.1) as i32;
            let range = InclusiveRange::new(sensor.0 - offset, sensor.0 + offset);
            ranges.entry(y).or_default().push(range);
        }
    }

    Ok(Space { ranges, objects })
}

fn part_one(space: &Space, which_row: i32) -> Result<u32> {
    let ranges = merge_all(space.ranges.get(&which_row).context("invalid row")?);
    let count: u32 = ranges.iter().map(|range| range.len()).sum();
    let count2: u32 = space
        .objects
        .iter()
        .filter(|(x, y)| *y == which_row && ranges.iter().any(|range| range.contains(*x)))
        .count() as u32;
    Ok(count - count2)
}

fn part_two(space: &Space, min: (i32, i32), max: (i32, i32)) -> Result<u128> {
    for current_row in min.1..=max.1 {
        let mut ranges = space
            .ranges
            .get(&current_row)
            .cloned()
            .context("invalid row")?;
        for x in space
            .objects
            .iter()
            .filter(|(_, y)| y == &current_row)
            .map(|(x, _)| x)
        {
            ranges.push(InclusiveRange::new(*x, *x));
        }
        let ranges: Vec<_> = ranges
            .into_iter()
            .filter(|range| range.end >= min.0 && range.start <= max.0)
            .map(|range| {
                let start = if range.start > min.0 {
                    range.start
                } else {
                    min.0
                };
                let end = if range.end < max.0 { range.end } else { max.0 };
                InclusiveRange::new(start, end)
            })
            .collect();
        let mut ranges = merge_all(&ranges);
        ranges.sort();
        let width: u32 = ranges.iter().map(|range| range.len()).sum();
        if width == min.0.abs_diff(max.0) {
            debug_assert!(ranges.len() == 2);
            let x: u128 = ranges.get(0).unwrap().end as u128 + 1;
            return Ok(x * 4_000_000 + current_row as u128);
        }
    }
    bail!("no solution found");
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_range() {
        assert_eq!(InclusiveRange::new(0, 0).len(), 1);
        assert_eq!(InclusiveRange::new(0, 10).len(), 11);

        assert!(InclusiveRange::new(0, 10).contains(0));
        assert!(!InclusiveRange::new(0, 10).contains(11));

        assert!(InclusiveRange::new(0, 10).intersects(&InclusiveRange::new(2, 8)));

        assert_eq!(
            InclusiveRange::new(0, 10).merge(InclusiveRange::new(2, 8)),
            InclusiveRange::new(0, 10)
        );

        assert_eq!(
            InclusiveRange::new(0, 10).merge(InclusiveRange::new(5, 15)),
            InclusiveRange::new(0, 15)
        );

        assert_eq!(
            InclusiveRange::new(10, 25).merge(InclusiveRange::new(5, 15)),
            InclusiveRange::new(5, 25)
        );
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&parse(INPUT).unwrap(), 10).unwrap(), 26);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(&parse(INPUT).unwrap(), (0, 0), (20, 20)).unwrap(),
            56000011
        );
    }
}
