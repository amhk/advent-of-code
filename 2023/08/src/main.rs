use anyhow::{bail, ensure, Result};
use num::integer::lcm;
use std::collections::BTreeMap;

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 19199)?;
    aoc::run!(part_two(input), 13663968099527)?;
    Ok(())
}

enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(anyhow::anyhow!("unexpected char {}", value)),
        }
    }
}

#[allow(clippy::type_complexity)]
fn parse(input: &str) -> Result<(Vec<Direction>, BTreeMap<String, (String, String)>)> {
    let Some((directions, mapping)) = input.split_once("\n\n") else {
        bail!("bad input: blank line not found");
    };
    let directions: Vec<Direction> = directions
        .chars()
        .map(|ch| ch.try_into())
        .collect::<Result<Vec<_>>>()?;
    let mut map = BTreeMap::new();
    for line in mapping.lines() {
        ensure!(line.len() == 16);
        let src = line[0..3].to_string();
        let left_dest = line[7..10].to_string();
        let right_dest = line[12..15].to_string();
        map.insert(src, (left_dest, right_dest));
    }
    Ok((directions, map))
}

fn solve(input: &str, start: &str, end: &str) -> Result<usize> {
    let (directions, map) = parse(input)?;
    let mut directions = directions.iter().cycle();
    let mut steps = 0;
    let mut current = start;
    while current != end {
        steps += 1;
        let (l, r) = map
            .get(current)
            .ok_or_else(|| anyhow::anyhow!("bad map key {}", current))?;
        current = match directions.next().unwrap() {
            Direction::Left => l,
            Direction::Right => r,
        };
        ensure!(steps < 100_000); // FIXME
    }
    Ok(steps)
}

fn part_one(input: &str) -> Result<usize> {
    solve(input, "AAA", "ZZZ")
}

fn part_two(input: &str) -> Result<usize> {
    // Manually inspecting both the test real input shows that
    //
    //   - There are an equal number of start and end nodes
    //   - For each start node, there is a path to exactly one end node
    //   - The path from the start node to the end node is identical in length as from the end node
    //     back to the end node (regardless of the LR... cycle)
    //   - There are no paths from one end node to another end node
    //
    // Thus we have <number of start nodes> independent loops that need to run until they align at
    // their end states. Calculating the least common multiplier (lcm) for all lengths of will give
    // us the puzzle answer.
    let mut steps = 1;
    let (_, map) = parse(input)?;
    let start_state: Vec<_> = map.keys().filter(|key| key.ends_with('A')).collect();
    let end_state: Vec<_> = map.keys().filter(|key| key.ends_with('Z')).collect();
    for start in start_state.iter() {
        for end in end_state.iter() {
            if let Ok(x) = solve(input, start, end) {
                steps = lcm(steps, x);
            }
        }
    }
    Ok(steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = include_str!("test-input-part-one.txt");
    const INPUT2: &str = include_str!("test-input-part-two.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT1).unwrap(), 6);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT2).unwrap(), 6);
    }
}
