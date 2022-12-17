use anyhow::{bail, Context, Result};
use aoc::Graph;
use itertools::Itertools;
use regex::Regex;
use rustc_hash::FxHashMap;

fn main() -> Result<()> {
    let input = include_str!("input.txt");

    let answer = part_one(input).context("no solution for part one")?;
    println!("Part 1: {}", answer);
    assert_eq!(answer, 1862);

    let answer = part_two(input).context("no solution for part two")?;
    println!("Part 2: {}", answer);
    assert_eq!(answer, 2422);

    Ok(())
}

fn clone_except<'a>(
    original: &'a FxHashMap<&'a str, u32>,
    exclude: &'a str,
) -> FxHashMap<&'a str, u32> {
    FxHashMap::from_iter(
        original
            .iter()
            .filter(|(k, _)| **k != exclude)
            .map(|(k, v)| (<&str>::clone(k), *v)),
    )
}

fn release_pressure(
    distances: &FxHashMap<(&str, &str), u32>,
    rates: &FxHashMap<&str, u32>,
    current_node: &str,
    current_rate: u32,
    released_so_far: u32,
    time_left: u32,
) -> u32 {
    debug_assert!(
        !rates.contains_key(current_node),
        "current node {:?} in rates {:?}",
        current_node,
        rates
    );

    if time_left == 0 || rates.is_empty() {
        return released_so_far + current_rate * time_left;
    }

    let mut max = 0;
    for (node, rate) in rates.iter() {
        let distance = distances.get(&(current_node, node)).unwrap();
        let x = if (distance + 1) <= time_left {
            release_pressure(
                distances,
                &clone_except(rates, node),
                node,
                current_rate + rate,
                released_so_far + current_rate * (distance + 1),
                time_left - (distance + 1),
            )
        } else {
            released_so_far + current_rate * time_left
        };
        max = max.max(x);
    }
    max
}

#[allow(clippy::type_complexity)]
fn parse(input: &str) -> Result<(FxHashMap<(&str, &str), u32>, FxHashMap<&str, u32>)> {
    let mut graph = Graph::default();
    let mut rates = FxHashMap::default();
    let regex =
        Regex::new(r"Valve (..) has flow rate=(\d+); tunnels? leads? to valves? (.*)").unwrap();
    for line in input.lines() {
        let caps = regex
            .captures(line)
            .with_context(|| format!("'{}' does not match regex", line))?;
        let name = caps.get(1).unwrap().as_str();
        let rate: u32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let edges: Vec<_> = caps.get(3).unwrap().as_str().split(", ").collect();

        graph.add_node(name);
        let node = graph.get_node_mut(&name).unwrap();
        for edge in edges {
            node.add_edge(edge, 1);
        }

        if rate != 0 {
            rates.insert(name, rate);
        }
    }

    let mut distances: FxHashMap<(&str, &str), u32> = FxHashMap::default();
    for start in graph.iter().map(|(name, _)| name) {
        for end in graph.iter().map(|(name, _)| name) {
            let d = match graph.dijkstra(start, end) {
                Some(path) => path.len() as u32 - 1,
                None => bail!("unexpected input: no path between two nodes"),
            };
            distances.insert((start, end), d);
        }
    }
    Ok((distances, rates))
}

fn part_one(input: &str) -> Result<u32> {
    let (distances, rates) = parse(input)?;
    Ok(release_pressure(&distances, &rates, "AA", 0, 0, 30))
}

fn part_two(input: &str) -> Result<u32> {
    let (distances, rates) = parse(input)?;
    let valves = rates.keys();
    let mut max = 0;
    // Assume each individual will handle one half of the valves; brute force through all
    // combinations of who handles what valves (6425 combinations for input.txt) and return the
    // highest result.
    for some_keys in valves.combinations(rates.len() / 2) {
        let subset1 = FxHashMap::from_iter(
            rates
                .iter()
                .filter(|(k, _)| some_keys.contains(k))
                .map(|(k, v)| (*k, *v)),
        );
        let subset2 = FxHashMap::from_iter(
            rates
                .iter()
                .filter(|(k, _)| !some_keys.contains(k))
                .map(|(k, v)| (*k, *v)),
        );
        max = max.max(
            release_pressure(&distances, &subset1, "AA", 0, 0, 26)
                + release_pressure(&distances, &subset2, "AA", 0, 0, 26),
        );
    }
    Ok(max)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_release_pressure() {
        // graph: BB -- AA -- CC
        let mut distances = FxHashMap::default();
        distances.insert(("AA", "BB"), 1);
        distances.insert(("AA", "CC"), 1);
        distances.insert(("BB", "AA"), 1);
        distances.insert(("BB", "CC"), 2);
        distances.insert(("CC", "AA"), 1);
        distances.insert(("CC", "BB"), 2);

        let mut rates = FxHashMap::default();
        rates.insert("BB", 10);
        rates.insert("CC", 3);

        assert_eq!(release_pressure(&distances, &rates, "AA", 0, 0, 0), 0); // AA: no time to move
        assert_eq!(release_pressure(&distances, &rates, "AA", 0, 0, 1), 0); // AA: move to BB
        assert_eq!(release_pressure(&distances, &rates, "AA", 0, 0, 2), 0); // BB: open BB
        assert_eq!(release_pressure(&distances, &rates, "AA", 0, 0, 3), 10); // BB: move to AA
        assert_eq!(release_pressure(&distances, &rates, "AA", 0, 0, 4), 20); // AA: move to CC
        assert_eq!(release_pressure(&distances, &rates, "AA", 0, 0, 5), 30); // CC: open CC
        assert_eq!(release_pressure(&distances, &rates, "AA", 0, 0, 6), 43); // C: all valves are open
        assert_eq!(release_pressure(&distances, &rates, "AA", 0, 0, 7), 56); // C: all valves are open
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 1651);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 1707);
    }
}
