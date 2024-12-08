use anyhow::{bail, Result};
use aoc::{BoundingBox, XY};
use std::collections::{HashMap, HashSet};

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 305)?;
    aoc::run!(part_two(input), 1150)?;
    Ok(())
}

type AntennaMap = HashMap<char, Vec<XY>>;

fn parse(input: &str) -> Result<(AntennaMap, BoundingBox)> {
    let mut antennas: AntennaMap = HashMap::new();
    let bounding_box = aoc::parse_grid(input, |xy, ch| match ch {
        '.' => Ok(()),
        ch if ch.is_ascii_alphanumeric() => {
            antennas.entry(ch).or_default().push(xy);
            Ok(())
        }
        _ => bail!("unexpected char {ch}"),
    })?;
    Ok((antennas, bounding_box))
}

fn part_x(input: &str, part_two_specifics: bool) -> Result<usize> {
    let (antennas, bb) = parse(input)?;
    let mut signals: HashSet<XY> = HashSet::new();
    for v in antennas.values() {
        for a in v {
            if part_two_specifics {
                signals.insert(*a);
            }
            for b in v {
                if a == b {
                    continue;
                }
                let step = a - b;
                let mut xy = *a;
                loop {
                    xy += step;
                    if bb.contains(&xy) {
                        signals.insert(xy);
                    } else {
                        break;
                    }
                    if !part_two_specifics {
                        break;
                    }
                }
            }
        }
    }
    Ok(signals.len())
}

fn part_one(input: &str) -> Result<usize> {
    part_x(input, false)
}

fn part_two(input: &str) -> Result<usize> {
    part_x(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 14);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 34);
    }
}
