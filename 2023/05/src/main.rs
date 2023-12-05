use anyhow::{Context, Result};
use regex::Regex;
use std::ops::Range;

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 486613012)?;
    aoc::run!(part_two(input), 56931769)?;
    Ok(())
}

#[derive(Debug)]
struct Mapping {
    ranges: Vec<(Range<usize>, usize)>,
}

impl Mapping {
    fn translate(&self, i: usize) -> usize {
        for (range, offset) in &self.ranges {
            if range.contains(&i) {
                return offset + i - range.start;
            }
        }
        i
    }
}

fn parse(input: &str) -> Result<(Vec<usize>, Vec<Mapping>)> {
    let re_seeds = Regex::new(r"\d+").unwrap();
    let re_mapping = Regex::new(r"(\d+) (\d+) (\d+)").unwrap();

    let (seeds_input, other_input) = input.split_once("\n\n").context("bad start of input")?;
    let seeds: Vec<usize> = re_seeds
        .find_iter(seeds_input)
        .map(|m| m.as_str().parse().unwrap())
        .collect();
    let mut mappings = vec![];
    for chunk in other_input.split("\n\n") {
        let mut mapping = Mapping { ranges: vec![] };
        for caps in re_mapping.captures_iter(chunk) {
            let offset: usize = caps.get(1).unwrap().as_str().parse().unwrap();
            let start: usize = caps.get(2).unwrap().as_str().parse().unwrap();
            let len: usize = caps.get(3).unwrap().as_str().parse().unwrap();
            mapping.ranges.push((start..start + len, offset));
        }
        mappings.push(mapping);
    }
    Ok((seeds, mappings))
}

fn min_location(seeds: &[Range<usize>], mappings: &[Mapping]) -> Result<usize> {
    let mut min = usize::MAX;
    for range in seeds.iter() {
        // FIXME: actually only needs to check lowest number in range, and start of each mapping?
        let x = range
            .clone()
            .map(|mut i| {
                for m in mappings {
                    i = m.translate(i);
                }
                i
            })
            .min()
            .context("no seeds, no minimum location")?;
        min = min.min(x);
    }
    Ok(min)
}

fn part_one(input: &str) -> Result<usize> {
    let (seeds, mappings) = parse(input)?;
    let seeds: Vec<_> = seeds.into_iter().map(|s| s..s + 1).collect();
    min_location(&seeds, &mappings)
}

fn part_two(input: &str) -> Result<usize> {
    let (seeds, mappings) = parse(input)?;
    let seeds: Vec<_> = seeds.chunks(2).map(|a| a[0]..a[0] + a[1]).collect();
    min_location(&seeds, &mappings)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_mapping() {
        let m = Mapping {
            ranges: vec![(50..50 + 2, 98), (52..52 + 48, 50)],
        };
        assert_eq!(m.translate(0), 0);
        assert_eq!(m.translate(49), 49);
        assert_eq!(m.translate(50), 98);
        assert_eq!(m.translate(51), 99);
        assert_eq!(m.translate(52), 50);
        assert_eq!(m.translate(53), 51);
        assert_eq!(m.translate(99), 97);
        assert_eq!(m.translate(100), 100);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 35);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 46);
    }
}
