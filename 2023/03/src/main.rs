use anyhow::Result;
use regex::Regex;
use std::collections::BTreeSet;

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 509115)?;
    aoc::run!(part_two(input), 75220503)?;
    Ok(())
}

type XY = (i32, i32);

#[derive(Debug)]
struct PartNumber {
    value: u32,
    positions: BTreeSet<XY>,
}

impl PartNumber {
    fn neighbours(&self) -> impl Iterator<Item = XY> {
        debug_assert!(!self.positions.is_empty());
        let y = self.positions.iter().next().unwrap().1;
        debug_assert!(self.positions.iter().all(|pos| pos.1 == y));
        let min_x = self.positions.iter().map(|(x, _)| x).min().unwrap();
        let max_x = self.positions.iter().map(|(x, _)| x).max().unwrap();
        let mut neighbours = BTreeSet::new();
        for y in y - 1..=y + 1 {
            for x in min_x - 1..=max_x + 1 {
                let xy = (x, y);
                if !self.positions.contains(&xy) {
                    neighbours.insert(xy);
                }
            }
        }
        neighbours.into_iter()
    }
}

fn parse(input: &str) -> Result<(Vec<PartNumber>, BTreeSet<XY>, BTreeSet<XY>)> {
    let mut numbers = vec![]; // the PartNumbers
    let mut symbols = BTreeSet::new(); // positions of all symbols that are not '.'
    let mut gears = BTreeSet::new(); // positions of all '*' symbols
    let regex = Regex::new(r"\d+").unwrap();
    for (y, line) in input.lines().enumerate() {
        for m in regex.find_iter(line) {
            let value = m.as_str().parse::<u32>()?;
            let positions = m
                .range()
                .map(|x| (x as i32, y as i32))
                .collect::<BTreeSet<XY>>();
            numbers.push(PartNumber { value, positions });
        }
        for (x, ch) in line.chars().enumerate() {
            if ch != '.' {
                symbols.insert((x as i32, y as i32));
            }
            if ch == '*' {
                gears.insert((x as i32, y as i32));
            }
        }
    }
    Ok((numbers, symbols, gears))
}

fn part_one(input: &str) -> Result<u32> {
    let (numbers, symbols, _) = parse(input)?;
    let mut sum = 0;
    for num in numbers.into_iter() {
        if num.neighbours().any(|xy| symbols.contains(&xy)) {
            sum += num.value;
        }
    }
    Ok(sum)
}

fn part_two(input: &str) -> Result<u32> {
    let (numbers, _, gears) = parse(input)?;
    let mut sum = 0;
    for xy in gears.into_iter() {
        let neighbouring_numbers = numbers
            .iter()
            .filter(|num| num.neighbours().any(|pos| pos == xy));
        if neighbouring_numbers.clone().count() == 2 {
            sum += neighbouring_numbers.map(|num| num.value).product::<u32>();
        }
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part_number_neighbours() {
        let pn = PartNumber {
            value: 42,
            positions: BTreeSet::from([(0, 0), (1, 0)]),
        };
        let actual = pn.neighbours().collect::<BTreeSet<_>>();
        let expected = BTreeSet::from([
            (-1, -1),
            (0, -1),
            (1, -1),
            (2, -1),
            (-1, 0),
            (2, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
            (2, 1),
        ]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 4361);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 467835);
    }
}
