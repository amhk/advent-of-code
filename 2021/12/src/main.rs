use std::collections::{BTreeMap, BTreeSet};

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

type Graph = BTreeMap<String, BTreeSet<String>>;

fn parse_input(input: &str) -> Result<Graph, Error> {
    let mut nodes = BTreeMap::new();
    for line in input.lines() {
        let mut split = line.split('-');
        let a = split.next().ok_or(Error::BadInput)?.to_string();
        let b = split.next().ok_or(Error::BadInput)?.to_string();
        if split.next().is_some() {
            return Err(Error::BadInput);
        }
        nodes
            .entry(a.clone())
            .or_insert_with(BTreeSet::new)
            .insert(b.clone());
        nodes.entry(b).or_insert_with(BTreeSet::new).insert(a);
    }
    Ok(nodes)
}

fn find_all_paths(
    graph: &Graph,
    start: &str,
    end: &str,
    allowance: usize,
) -> Result<BTreeSet<Vec<String>>, Error> {
    #[allow(clippy::ptr_arg)]
    fn may_visit_again(current_path: &[String], node: &String) -> Option<usize> {
        // big caves may be visited any number of times
        if node.chars().all(|ch| ch.is_ascii_uppercase()) {
            return Some(0);
        }

        // all small caves may be visited once
        if !current_path.contains(node) {
            return Some(0);
        }

        // exactly one small cave, except for start and end, may be visited twice
        if node != "start" && node != "end" {
            return Some(1);
        }

        None
    }

    fn visit(
        graph: &Graph,
        current_node: &str,
        end_node: &str,
        current_path: &[String],
        allowance: usize,
    ) -> Result<BTreeSet<Vec<String>>, Error> {
        if current_path.len() > 1000 {
            // max number 1000 arbitrarily chosen
            return Err(Error::CyclicGraph);
        }

        let mut set: BTreeSet<Vec<String>> = BTreeSet::new();
        if current_node == end_node {
            set.insert(current_path.to_vec());
        } else {
            for (node, cost) in graph
                .get(current_node)
                .ok_or(Error::NoSuchNode)?
                .iter()
                .map(|node| (node, may_visit_again(current_path, node)))
                .filter(|(_, cost)| cost.is_some() && cost.unwrap() <= allowance)
                .map(|(node, cost)| (node, cost.unwrap()))
            {
                let mut next_current_path = Vec::from_iter(current_path.iter().cloned());
                next_current_path.push(node.to_string());
                set.append(&mut visit(
                    graph,
                    node,
                    end_node,
                    &next_current_path,
                    allowance - cost,
                )?);
            }
        }
        Ok(set)
    }

    visit(graph, start, end, &[start.to_string()], allowance)
}

fn part_one(input: &str) -> Result<usize, Error> {
    let graph = parse_input(input)?;
    Ok(find_all_paths(&graph, "start", "end", 0)?.len())
}

fn part_two(input: &str) -> Result<usize, Error> {
    let graph = parse_input(input)?;
    Ok(find_all_paths(&graph, "start", "end", 1)?.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = include_str!("test-input1.txt");
    const INPUT2: &str = include_str!("test-input2.txt");
    const INPUT3: &str = include_str!("test-input3.txt");

    #[test]
    fn test_find_all_paths() {
        const INPUT1: &str = include_str!("test-input1.txt");
        let graph = parse_input(INPUT1).unwrap();
        let paths = find_all_paths(&graph, "start", "end", 0).unwrap();
        let mut paths: Vec<_> = paths.iter().collect();
        paths.sort_unstable();
        let paths: Vec<_> = paths.iter().map(|v| v.join(",")).collect();
        assert_eq!(
            paths,
            vec![
                "start,A,b,A,c,A,end",
                "start,A,b,A,end",
                "start,A,b,end",
                "start,A,c,A,b,A,end",
                "start,A,c,A,b,end",
                "start,A,c,A,end",
                "start,A,end",
                "start,b,A,c,A,end",
                "start,b,A,end",
                "start,b,end",
            ]
        );
    }

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
