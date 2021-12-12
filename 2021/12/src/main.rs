use std::collections::BTreeMap;

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
    NoSuchNode,
    CyclicGraph,
}

type Node = u32;
const NODE_START: Node = 0;
const NODE_END: Node = 1;
type Graph = BTreeMap<Node, Vec<Node>>;

fn parse_input(input: &str) -> Result<Graph, Error> {
    let mut node_ids: BTreeMap<&str, Node> = BTreeMap::new();
    node_ids.insert("start", NODE_START);
    node_ids.insert("end", NODE_END);
    let mut next_id = NODE_END + 1;
    let mut nodes = BTreeMap::new();
    for line in input.lines() {
        macro_rules! parse_node {
            ($split:ident, $next_id:ident) => {{
                let node = $split.next().ok_or(Error::BadInput)?;
                *node_ids.entry(node).or_insert_with(|| {
                    // big caves are assigned an even number, small caves an odd number
                    let odd_or_even = if node.chars().all(|ch| ch.is_ascii_uppercase()) {
                        0
                    } else {
                        1
                    };
                    if next_id % 2 != odd_or_even {
                        next_id += 1;
                    } else {
                        next_id += 2;
                    }
                    $next_id
                })
            }};
        }

        let mut split = line.split('-');
        let a = parse_node!(split, next_id);
        let b = parse_node!(split, next_id);
        if split.next().is_some() {
            return Err(Error::BadInput);
        }
        nodes.entry(a).or_insert_with(Vec::new).push(b);
        nodes.entry(b).or_insert_with(Vec::new).push(a);
    }
    Ok(nodes)
}

fn find_all_paths(graph: &Graph, allowance: usize) -> Result<Vec<Vec<Node>>, Error> {
    fn may_visit_again(current_path: &[Node], node: Node) -> Option<usize> {
        // may never visit the start node again
        if node == NODE_START {
            return None;
        }

        // big caves may be visited any number of times
        if node % 2 == 0 {
            return Some(0);
        }

        // all small caves may be visited once
        if !current_path.contains(&node) {
            return Some(0);
        }

        // exactly one small cave, except for start and end, may be visited twice
        if node != NODE_END {
            return Some(1);
        }

        None
    }

    fn visit(
        graph: &Graph,
        current_path: &[Node],
        allowance: usize,
    ) -> Result<Vec<Vec<Node>>, Error> {
        if current_path.len() > 1000 {
            // max number 1000 arbitrarily chosen
            return Err(Error::CyclicGraph);
        }
        let current_node = *current_path.iter().last().unwrap();

        let mut set: Vec<Vec<Node>> = Vec::new();
        if current_node == NODE_END {
            set.push(current_path.to_vec());
        } else {
            for (node, cost) in graph
                .get(&current_node)
                .ok_or(Error::NoSuchNode)?
                .iter()
                .map(|node| (node, may_visit_again(current_path, *node)))
                .filter(|(_, cost)| cost.is_some() && cost.unwrap() <= allowance)
                .map(|(node, cost)| (node, cost.unwrap()))
            {
                let mut next_current_path = Vec::from_iter(current_path.iter().cloned());
                next_current_path.push(*node);
                set.append(&mut visit(graph, &next_current_path, allowance - cost)?);
            }
        }
        Ok(set)
    }

    visit(graph, &[NODE_START], allowance)
}

fn part_one(input: &str) -> Result<usize, Error> {
    let graph = parse_input(input)?;
    Ok(find_all_paths(&graph, 0)?.len())
}

fn part_two(input: &str) -> Result<usize, Error> {
    let graph = parse_input(input)?;
    Ok(find_all_paths(&graph, 1)?.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = include_str!("test-input1.txt");
    const INPUT2: &str = include_str!("test-input2.txt");
    const INPUT3: &str = include_str!("test-input3.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT1), Ok(10));
        assert_eq!(part_one(INPUT2), Ok(19));
        assert_eq!(part_one(INPUT3), Ok(226));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT1), Ok(36));
        assert_eq!(part_two(INPUT2), Ok(103));
        assert_eq!(part_two(INPUT3), Ok(3509));
    }
}
