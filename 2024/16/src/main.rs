use anyhow::{anyhow, bail, ensure, Result};
use aoc::{Direction, Graph, XY};
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
    thread,
};

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 105508)?;
    aoc::run!(part_two(input), 548)?;
    Ok(())
}

type NodeId = (XY, Direction);

fn parse(input: &str) -> Result<(Graph<NodeId>, NodeId, NodeId, HashSet<XY>)> {
    // get set of XY
    let mut spaces = HashSet::new();
    let mut start: Option<XY> = None;
    let mut end: Option<XY> = None;
    let _ = aoc::parse_grid(input, |xy, ch| match ch {
        '#' => Ok(()),
        '.' => {
            spaces.insert(xy);
            Ok(())
        }
        'S' => {
            ensure!(start.is_none(), "duplicate start positions");
            start = Some(xy);
            spaces.insert(xy);
            Ok(())
        }
        'E' => {
            ensure!(end.is_none(), "duplicate end positions");
            end = Some(xy);
            spaces.insert(xy);
            Ok(())
        }
        _ => bail!("unexpected char '{ch}'"),
    })?;
    let Some(start) = start else {
        bail!("missing start position");
    };
    let Some(end) = end else {
        bail!("missing end position");
    };

    // create graph nodes
    let mut graph = Graph::default();
    for &xy in spaces.iter() {
        graph.add_node((xy, Direction::North));
        graph.add_node((xy, Direction::East));
        graph.add_node((xy, Direction::South));
        graph.add_node((xy, Direction::West));
    }

    // add edges between empty spaces to graph
    for &xy in spaces.iter() {
        graph
            .get_node_mut(&(xy, Direction::North))
            .expect("node exists")
            .add_edge((xy, Direction::East), 1000);
        graph
            .get_node_mut(&(xy, Direction::North))
            .expect("node exists")
            .add_edge((xy, Direction::West), 1000);

        graph
            .get_node_mut(&(xy, Direction::East))
            .expect("node exists")
            .add_edge((xy, Direction::North), 1000);
        graph
            .get_node_mut(&(xy, Direction::East))
            .expect("node exists")
            .add_edge((xy, Direction::South), 1000);

        graph
            .get_node_mut(&(xy, Direction::South))
            .expect("node exists")
            .add_edge((xy, Direction::West), 1000);
        graph
            .get_node_mut(&(xy, Direction::South))
            .expect("node exists")
            .add_edge((xy, Direction::East), 1000);

        graph
            .get_node_mut(&(xy, Direction::West))
            .expect("node exists")
            .add_edge((xy, Direction::North), 1000);
        graph
            .get_node_mut(&(xy, Direction::West))
            .expect("node exists")
            .add_edge((xy, Direction::South), 1000);

        if spaces.contains(&xy.north()) {
            graph
                .get_node_mut(&(xy, Direction::North))
                .expect("node exists")
                .add_edge((xy.north(), Direction::North), 1);
        }
        if spaces.contains(&xy.east()) {
            graph
                .get_node_mut(&(xy, Direction::East))
                .expect("node exists")
                .add_edge((xy.east(), Direction::East), 1);
        }
        if spaces.contains(&xy.south()) {
            graph
                .get_node_mut(&(xy, Direction::South))
                .expect("node exists")
                .add_edge((xy.south(), Direction::South), 1);
        }
        if spaces.contains(&xy.west()) {
            graph
                .get_node_mut(&(xy, Direction::West))
                .expect("node exists")
                .add_edge((xy.west(), Direction::West), 1);
        }
    }

    // add 0 cost edges to fake end node so there is a single end node, not one per direction
    let fake_end: NodeId = ((i32::MAX, i32::MAX).into(), Direction::North);
    graph.add_node(fake_end);
    graph
        .get_node_mut(&(end, Direction::North))
        .expect("node exists")
        .add_edge(fake_end, 0);
    graph
        .get_node_mut(&(end, Direction::East))
        .expect("node exists")
        .add_edge(fake_end, 0);
    graph
        .get_node_mut(&(end, Direction::South))
        .expect("node exists")
        .add_edge(fake_end, 0);
    graph
        .get_node_mut(&(end, Direction::West))
        .expect("node exists")
        .add_edge(fake_end, 0);

    Ok((graph, (start, Direction::East), fake_end, spaces))
}

fn part_one(input: &str) -> Result<usize> {
    let (graph, start, end, _) = parse(input)?;
    let steps = graph
        .dijkstra(&start, &end)
        .ok_or_else(|| anyhow!("no path from {:?} to {:?}", start, end))?;
    Ok(steps[steps.len() - 1].1 as usize)
}

fn part_two(input: &str) -> Result<usize> {
    struct Context {
        seats: HashSet<XY>,
        worklist: Vec<XY>,
    }

    let (graph, start, end, spaces) = parse(input)?;
    let steps = graph
        .dijkstra(&start, &end)
        .ok_or_else(|| anyhow!("no path from {:?} to {:?}", start, end))?;
    let optimal_cost = steps[steps.len() - 1].1;

    let context = Arc::new(Mutex::new(Context {
        seats: HashSet::new(),
        worklist: Vec::from_iter(spaces),
    }));

    let graph = &graph;
    thread::scope(|s| {
        for _ in 0..num_cpus::get() {
            let c = context.clone();
            s.spawn(move || loop {
                let Some(xy) = ({
                    let mut c = c.lock().unwrap();
                    c.worklist.pop()
                }) else {
                    return;
                };

                for dir in [
                    Direction::North,
                    Direction::East,
                    Direction::South,
                    Direction::West,
                ] {
                    let Some(a) = graph.dijkstra(&start, &(xy, dir)) else {
                        continue;
                    };
                    let a = a[a.len() - 1].1;
                    if a > optimal_cost {
                        continue;
                    }

                    let Some(b) = graph.dijkstra(&(xy, dir), &end) else {
                        continue;
                    };
                    let b = b[b.len() - 1].1;
                    if a + b != optimal_cost {
                        continue;
                    }

                    {
                        let mut c = c.lock().unwrap();
                        c.seats.insert(xy);
                        break;
                    }
                }
            });
        }
    });

    let x = context.lock().unwrap().seats.len();
    Ok(x)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_SMALL: &str = include_str!("test-input-small.txt");
    const INPUT_LARGE: &str = include_str!("test-input-large.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT_SMALL).unwrap(), 7036);
        assert_eq!(part_one(INPUT_LARGE).unwrap(), 11048);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT_SMALL).unwrap(), 45);
        assert_eq!(part_two(INPUT_LARGE).unwrap(), 64);
    }
}
