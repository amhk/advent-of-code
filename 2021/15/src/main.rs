use rustc_hash::FxHashMap;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

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
    CorruptGraph,
    NoPath,
}

type NodeId = (i32, i32);

#[derive(Debug, PartialEq)]
struct Edge {
    dest: NodeId,
    cost: u32,
}

type Graph = FxHashMap<(i32, i32), Vec<Edge>>;

#[derive(Eq, PartialEq)]
struct State {
    node: NodeId,
    cost: u32, // cumulative cost from the start node to self.node
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // assign Ordering::Less to the minumum of self.cost and other.cost -> call cmp on
        // other.cost (as opposed to self.cost)
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(input: &str, expand_x: usize, expand_y: usize) -> Result<Graph, Error> {
    // parse text input
    let mut digits: FxHashMap<(i32, i32), u32> = FxHashMap::default();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let value = ch.to_digit(10).ok_or(Error::BadInput)?;
            digits.insert((x as i32, y as i32), value);
        }
    }

    // (optionally) expand input
    let keys: Vec<_> = digits.keys().cloned().collect();
    let max = keys.iter().max().ok_or(Error::BadInput)?;
    let width = max.0 + 1;
    let height = max.1 + 1;
    for x in 0..expand_x as i32 {
        for y in 0..expand_y as i32 {
            if x == 0 && y == 0 {
                continue;
            }
            for &(old_x, old_y) in keys.iter() {
                let old_value = *digits.get(&(old_x, old_y)).unwrap();
                let new_value = (old_value - 1 + x as u32 + y as u32) % 9 + 1;
                let new_x = old_x + x * width;
                let new_y = old_y + y * height;
                digits.insert((new_x, new_y), new_value);
            }
        }
    }

    // create graph from input
    let mut graph = Graph::default();
    for &(x, y) in digits.keys() {
        let mut edges = vec![];
        for (i, j) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
            if let Some(&cost) = digits.get(&(i, j)) {
                edges.push(Edge { dest: (i, j), cost });
            }
        }
        graph.insert((x, y), edges);
    }

    Ok(graph)
}

fn dijkstra(graph: &Graph, start_node: NodeId, end_node: NodeId) -> Result<u32, Error> {
    let mut total_costs: FxHashMap<NodeId, u32> = FxHashMap::default();

    let mut prio_queue: BinaryHeap<State> = BinaryHeap::new();
    prio_queue.push(State {
        node: start_node,
        cost: 0,
    });

    while let Some(State {
        node: current_node,
        cost: current_cost,
    }) = prio_queue.pop()
    {
        if current_node == end_node {
            return Ok(current_cost);
        }

        for edge in graph.get(&current_node).ok_or(Error::CorruptGraph)?.iter() {
            let proposed_cost = current_cost + edge.cost;
            let best_cost_so_far = total_costs.entry(edge.dest).or_insert(u32::MAX);
            if proposed_cost < *best_cost_so_far {
                prio_queue.push(State {
                    node: edge.dest,
                    cost: proposed_cost,
                });
                *best_cost_so_far = proposed_cost;
            }
        }
    }

    Err(Error::NoPath)
}

fn part_one(input: &str) -> Result<u32, Error> {
    let graph = parse_input(input, 0, 0)?;
    let start_node = *graph.keys().min().ok_or(Error::BadInput)?;
    let end_node = *graph.keys().max().ok_or(Error::BadInput)?;
    dijkstra(&graph, start_node, end_node)
}

fn part_two(input: &str) -> Result<u32, Error> {
    let graph = parse_input(input, 5, 5)?;
    let start_node = *graph.keys().min().ok_or(Error::BadInput)?;
    let end_node = *graph.keys().max().ok_or(Error::BadInput)?;
    dijkstra(&graph, start_node, end_node)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");
    const EXPANDED_INPUT: &str = include_str!("test-expanded-input.txt");

    #[test]
    fn test_parse_input() {
        let graph = parse_input(INPUT, 5, 5).unwrap();
        let expected = parse_input(EXPANDED_INPUT, 0, 0).unwrap();
        assert_eq!(graph, expected);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), Ok(40));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), Ok(315));
    }
}
