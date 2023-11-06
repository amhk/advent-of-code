#![allow(dead_code)]
#![allow(unused_variables)]
use anyhow::{bail, ensure, Context, Result};
use std::collections::BTreeSet;

type Graph = aoc::Graph<(XY, usize)>;

const START_NODE: (XY, usize) = ((0, 0), 0);
const END_NODE: (XY, usize) = ((usize::MAX, usize::MAX), usize::MAX);

fn main() -> Result<()> {
    let input = include_str!("input.txt");

    let answer = part_one(input).context("no solution for part one")?;
    println!("Part 1: {}", answer);
    assert_eq!(answer, 299);

    let answer = part_two(input).context("no solution for part two")?;
    println!("Part 2: {}", answer);
    assert_eq!(answer, 899);

    Ok(())
}

type XY = (usize, usize);

type Spaces = BTreeSet<XY>;

fn calculate_free_spaces(input: &str) -> Result<(Vec<Spaces>, usize, usize)> {
    let mut up = BTreeSet::new();
    let mut right = BTreeSet::new();
    let mut down = BTreeSet::new();
    let mut left = BTreeSet::new();
    let mut wall = BTreeSet::new();

    let width = input
        .lines()
        .take(1)
        .map(|line| line.len())
        .max()
        .context("empty input")?;
    ensure!(width > 2);
    let height = input.lines().count();
    ensure!(height > 2);

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '.' => {}
                '^' => {
                    up.insert((x, y));
                }
                '>' => {
                    right.insert((x, y));
                }
                'v' => {
                    down.insert((x, y));
                }
                '<' => {
                    left.insert((x, y));
                }
                '#' => {
                    wall.insert((x, y));
                }
                _ => bail!("unexpected char {}", ch),
            }
        }
    }

    fn shift_up(old: BTreeSet<XY>, max: usize) -> BTreeSet<XY> {
        let mut new = BTreeSet::new();
        for (x, y) in old.into_iter() {
            new.insert((x, if y == 1 { max - 2 } else { y - 1 }));
        }
        new
    }

    fn shift_right(old: BTreeSet<XY>, max: usize) -> BTreeSet<XY> {
        let mut new = BTreeSet::new();
        for (x, y) in old.into_iter() {
            new.insert((if x == max - 2 { 1 } else { x + 1 }, y));
        }
        new
    }

    fn shift_down(old: BTreeSet<XY>, max: usize) -> BTreeSet<XY> {
        let mut new = BTreeSet::new();
        for (x, y) in old.into_iter() {
            new.insert((x, if y == max - 2 { 1 } else { y + 1 }));
        }
        new
    }

    fn shift_left(old: BTreeSet<XY>, max: usize) -> BTreeSet<XY> {
        let mut new = BTreeSet::new();
        for (x, y) in old.into_iter() {
            new.insert((if x == 1 { max - 2 } else { x - 1 }, y));
        }
        new
    }

    let original = (up.clone(), right.clone(), down.clone(), left.clone());
    let mut spaces = vec![];
    loop {
        let mut space = BTreeSet::new();
        for x in 0..width {
            for y in 0..height {
                if !up.contains(&(x, y))
                    && !right.contains(&(x, y))
                    && !down.contains(&(x, y))
                    && !left.contains(&(x, y))
                    && !wall.contains(&(x, y))
                {
                    space.insert((x, y));
                }
            }
        }
        spaces.push(space);

        up = shift_up(up.clone(), height);
        right = shift_right(right.clone(), width);
        down = shift_down(down.clone(), height);
        left = shift_left(left.clone(), width);
        if (up.clone(), right.clone(), down.clone(), left.clone()) == original {
            break;
        }
    }

    Ok((spaces, width, height))
}

fn parse(input: &str) -> Result<Graph> {
    let (spaces, width, height) = calculate_free_spaces(input)?;
    let period = spaces.len();

    let mut graph = Graph::default();

    // Add special start and end nodes: these have no outgoing edges, and the cost of going to one
    // of these is always 0. They allow the caller of graph.dijkstra to specify an end node without
    // knowing the correct iteration beforehand.
    graph.add_node(START_NODE);
    graph.add_node(END_NODE);

    for iteration in 0..period {
        let next_iteration = (iteration + 1) % period;
        let space = &spaces[iteration];
        let next_space = &spaces[next_iteration];
        for (x, y) in space {
            let (x, y) = (*x, *y);
            graph.add_node(((x, y), iteration));
            let node = graph.get_node_mut(&((x, y), iteration)).unwrap();
            if y > 0 && next_space.contains(&(x, y - 1)) {
                node.add_edge(((x, y - 1), next_iteration), 1);
            }
            if next_space.contains(&(x + 1, y)) {
                node.add_edge(((x + 1, y), next_iteration), 1);
            }
            if next_space.contains(&(x, y + 1)) {
                node.add_edge(((x, y + 1), next_iteration), 1);
            }
            if x > 0 && next_space.contains(&(x - 1, y)) {
                node.add_edge(((x - 1, y), next_iteration), 1);
            }
            if next_space.contains(&(x, y)) {
                node.add_edge(((x, y), next_iteration), 1);
            }

            // fix start and end nodes
            if x == 1 && y == 0 {
                node.add_edge(START_NODE, 0);
            }
            if x == width - 2 && y == height - 1 {
                node.add_edge(END_NODE, 0);
            }
        }
    }
    Ok(graph)
}

fn part_one(input: &str) -> Result<u32> {
    let graph = parse(input)?;
    let mut path = graph
        .dijkstra(&((1, 0), 0), &END_NODE)
        .context("no path found")?;
    let _ = path.pop().unwrap(); // pop END_NODE
    let last = path.pop().unwrap();
    Ok(last.1)
}

fn part_two(input: &str) -> Result<u32> {
    let graph = parse(input)?;
    let mut total = 0;

    let mut path = graph
        .dijkstra(&((1, 0), 0), &END_NODE)
        .context("no path found")?;
    let _ = path.pop().unwrap(); // pop END_NODE
    let last = path.pop().unwrap();
    total += last.1;

    let mut path = graph
        .dijkstra(&last.0, &START_NODE)
        .context("no path found")?;
    let _ = path.pop().unwrap(); // pop START_NODE
    let last = path.pop().unwrap();
    total += last.1;

    let mut path = graph
        .dijkstra(&last.0, &END_NODE)
        .context("no path found")?;
    let _ = path.pop().unwrap(); // pop END_NODE
    let last = path.pop().unwrap();
    total += last.1;

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_calculate_free_spaces() {
        let (spaces, width, height) = calculate_free_spaces(INPUT).unwrap();
        assert_eq!(spaces.len(), 12);
        assert_eq!(width, 8);
        assert_eq!(height, 6);
        assert_eq!(
            spaces[0],
            BTreeSet::from([(1, 0), (3, 1), (1, 2), (3, 2), (4, 2), (3, 3), (6, 5),])
        );
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 18);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 18 + 23 + 13);
    }
}
