use std::collections::HashMap;

use aoc::Graph;

use anyhow::{ensure, Context, Result};

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 462)?;
    aoc::run!(part_two(input), 451)?;
    Ok(())
}

type XY = (i32, i32);

fn parse(input: &str) -> Result<(Graph<XY>, XY, XY, Vec<XY>)> {
    fn char_to_usize(ch: char) -> Result<usize> {
        ensure!(ch.is_ascii_lowercase(), "unexpected char {}", ch);
        Ok(ch as usize - 'a' as usize)
    }

    let mut start = None;
    let mut end = None;
    let mut heights = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let x = x as i32;
            let y = y as i32;
            match ch {
                'S' => {
                    start = Some((x, y));
                    heights.insert((x, y), char_to_usize('a')?);
                }
                'E' => {
                    end = Some((x, y));
                    heights.insert((x, y), char_to_usize('z')?);
                }
                _ => {
                    heights.insert((x, y), char_to_usize(ch)?);
                }
            };
        }
    }
    let start = start.context("bad input: missing S")?;
    let end = end.context("bad input: missing S")?;

    let mut graph = Graph::default();
    for ((x, y), height) in &heights {
        let x = *x;
        let y = *y;
        graph.add_node((x, y));
        let node = graph.get_node_mut(&(x, y)).unwrap();
        for (dist_x, dist_y) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
            if let Some(&dist_height) = heights.get(&(dist_x, dist_y)) {
                if height + 1 >= dist_height {
                    node.add_edge((dist_x, dist_y), 1);
                }
            }
        }
    }

    Ok((
        graph,
        start,
        end,
        heights
            .iter()
            .filter(|(_, v)| **v == 0)
            .map(|(k, _)| k)
            .copied()
            .collect(),
    ))
}

fn part_one(input: &str) -> Result<usize> {
    let (graph, start, end, _) = parse(input)?;
    let path = graph
        .dijkstra(&start, &end)
        .context("no path from start to end")?;
    Ok(path.len() - 1)
}

fn part_two(input: &str) -> Result<usize> {
    let (graph, _, end, starts) = parse(input)?;
    let mut min = usize::MAX;
    for start in starts {
        if let Some(path) = graph.dijkstra(&start, &end) {
            min = min.min(path.len() - 1);
        }
    }
    Ok(min)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 31);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 29);
    }
}
