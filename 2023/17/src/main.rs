use std::cmp::Reverse;
use std::collections::BTreeMap;
use std::collections::BinaryHeap;

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 1155)?;
    aoc::run!(part_two(input), 1283)?;
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Heading {
    North,
    East,
    South,
    West,
}

type XY = (i32, i32);

struct Grid {
    cells: BTreeMap<XY, u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct DijkstraState {
    heading: Heading,
    steps: usize,
}

impl Grid {
    fn begin(&self) -> XY {
        (0, 0)
    }

    fn end(&self) -> XY {
        let max_x = self.cells.keys().map(|(x, _)| x).max().unwrap();
        let max_y = self.cells.keys().map(|(_, y)| y).max().unwrap();
        (*max_x, *max_y)
    }

    fn dijkstra<F, R>(&self, begin: XY, end: XY, neighbours: F) -> Option<u32>
    where
        F: Fn(XY, DijkstraState) -> R,
        R: Iterator<Item = (XY, u32, DijkstraState)>,
    {
        if !self.cells.contains_key(&begin) || !self.cells.contains_key(&end) {
            return None;
        }

        // Cumulative, minimum cost of moving from begin to XY
        let mut total_costs: BTreeMap<(XY, DijkstraState), u32> = BTreeMap::new();

        // Priority queue, sorted by lowest cost (followed by XY, followed by DijkstraState, in case of a tie)
        let mut prio_queue = BinaryHeap::new();
        prio_queue.push(Reverse((
            0,
            begin,
            DijkstraState {
                heading: Heading::East,
                steps: 0,
            },
        )));
        prio_queue.push(Reverse((
            0,
            begin,
            DijkstraState {
                heading: Heading::South,
                steps: 0,
            },
        )));

        while let Some(Reverse((current_cost, current_xy, current_state))) = prio_queue.pop() {
            if current_xy == end {
                return Some(current_cost);
            }

            for (new_xy, cost, new_state) in neighbours(current_xy, current_state) {
                let proposed_cost = current_cost + cost;
                let best_cost_so_far = total_costs.entry((new_xy, new_state)).or_insert(u32::MAX);
                if proposed_cost < *best_cost_so_far {
                    prio_queue.push(Reverse((proposed_cost, new_xy, new_state)));
                    *best_cost_so_far = proposed_cost;
                }
            }
        }

        None
    }

    fn neighbours_at_most_three(
        &self,
        xy: XY,
        state: DijkstraState,
    ) -> impl Iterator<Item = (XY, u32, DijkstraState)> {
        let mut v = vec![];
        macro_rules! maybe_push {
            ($new_xy:expr, $heading:expr) => {
                if !(state.heading == $heading && state.steps >= 3) {
                    if let Some(cost) = self.cells.get(&$new_xy) {
                        v.push((
                            $new_xy,
                            *cost,
                            DijkstraState {
                                steps: if state.heading == $heading {
                                    state.steps + 1
                                } else {
                                    1
                                },
                                heading: $heading,
                            },
                        ));
                    }
                }
            };
        }
        if state.heading != Heading::South {
            maybe_push!((xy.0, xy.1 - 1), Heading::North);
        }
        if state.heading != Heading::West {
            maybe_push!((xy.0 + 1, xy.1), Heading::East);
        }
        if state.heading != Heading::North {
            maybe_push!((xy.0, xy.1 + 1), Heading::South);
        }
        if state.heading != Heading::East {
            maybe_push!((xy.0 - 1, xy.1), Heading::West);
        }
        v.into_iter()
    }

    fn neighbours_at_least_four_at_most_ten(
        &self,
        xy: XY,
        state: DijkstraState,
    ) -> impl Iterator<Item = (XY, u32, DijkstraState)> {
        let mut v = vec![];
        macro_rules! maybe_push {
            ($new_xy:expr, $heading:expr) => {
                if let Some(cost) = self.cells.get(&$new_xy) {
                    v.push((
                        $new_xy,
                        *cost,
                        DijkstraState {
                            steps: if state.heading == $heading {
                                state.steps + 1
                            } else {
                                1
                            },
                            heading: $heading,
                        },
                    ));
                }
            };
        }
        let x = xy.0;
        let y = xy.1;
        let north = (x, y - 1);
        let east = (x + 1, y);
        let south = (x, y + 1);
        let west = (x - 1, y);
        if state.steps < 4 {
            let steps_left = 4 - state.steps as i32;
            if state.heading == Heading::North && self.cells.contains_key(&(x, y - steps_left)) {
                let mut cost = 0;
                for i in 1..=steps_left {
                    cost += self.cells[&(x, y - i)];
                }
                v.push((
                    (x, y - steps_left),
                    cost,
                    DijkstraState {
                        steps: 4,
                        heading: Heading::North,
                    },
                ));
            }
            if state.heading == Heading::East && self.cells.contains_key(&(x + steps_left, y)) {
                let mut cost = 0;
                for i in 1..=steps_left {
                    cost += self.cells[&(x + i, y)];
                }
                v.push((
                    (x + steps_left, y),
                    cost,
                    DijkstraState {
                        steps: 4,
                        heading: Heading::East,
                    },
                ));
            }
            if state.heading == Heading::South && self.cells.contains_key(&(x, y + steps_left)) {
                let mut cost = 0;
                for i in 1..=steps_left {
                    cost += self.cells[&(x, y + i)];
                }
                v.push((
                    (x, y + steps_left),
                    cost,
                    DijkstraState {
                        steps: 4,
                        heading: Heading::South,
                    },
                ));
            }
            if state.heading == Heading::West && self.cells.contains_key(&(x - steps_left, y)) {
                let mut cost = 0;
                for i in 1..=steps_left {
                    cost += self.cells[&(x - i, y)];
                }
                v.push((
                    (x - steps_left, y),
                    cost,
                    DijkstraState {
                        steps: 4,
                        heading: Heading::West,
                    },
                ));
            }
        } else if state.steps < 10 {
            if state.heading == Heading::North {
                maybe_push!(north, Heading::North);
                v.push((
                    xy,
                    0,
                    DijkstraState {
                        steps: 0,
                        heading: Heading::East,
                    },
                ));
                v.push((
                    xy,
                    0,
                    DijkstraState {
                        steps: 0,
                        heading: Heading::West,
                    },
                ));
            }
            if state.heading == Heading::East {
                maybe_push!(east, Heading::East);
                v.push((
                    xy,
                    0,
                    DijkstraState {
                        steps: 0,
                        heading: Heading::North,
                    },
                ));
                v.push((
                    xy,
                    0,
                    DijkstraState {
                        steps: 0,
                        heading: Heading::South,
                    },
                ));
            }
            if state.heading == Heading::South {
                maybe_push!(south, Heading::South);
                v.push((
                    xy,
                    0,
                    DijkstraState {
                        steps: 0,
                        heading: Heading::East,
                    },
                ));
                v.push((
                    xy,
                    0,
                    DijkstraState {
                        steps: 0,
                        heading: Heading::West,
                    },
                ));
            }
            if state.heading == Heading::West {
                maybe_push!(west, Heading::West);
                v.push((
                    xy,
                    0,
                    DijkstraState {
                        steps: 0,
                        heading: Heading::North,
                    },
                ));
                v.push((
                    xy,
                    0,
                    DijkstraState {
                        steps: 0,
                        heading: Heading::South,
                    },
                ));
            }
        } else {
            if state.heading == Heading::North {
                maybe_push!(east, Heading::East);
                maybe_push!(west, Heading::West);
            }
            if state.heading == Heading::East {
                maybe_push!(north, Heading::North);
                maybe_push!(south, Heading::South);
            }
            if state.heading == Heading::South {
                maybe_push!(east, Heading::East);
                maybe_push!(west, Heading::West);
            }
            if state.heading == Heading::West {
                maybe_push!(north, Heading::North);
                maybe_push!(south, Heading::South);
            }
        }
        v.into_iter()
    }
}

fn parse(input: &str) -> Result<Grid> {
    let mut cells = BTreeMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let y = y as i32;
            let x = x as i32;
            let value = ch
                .to_digit(10)
                .ok_or_else(|| anyhow!("unexpected char {ch}"))?;
            cells.insert((x, y), value);
        }
    }

    Ok(Grid { cells })
}

fn part_one(input: &str) -> Result<u32> {
    let grid = parse(input)?;
    grid.dijkstra(grid.begin(), grid.end(), |a, b| {
        grid.neighbours_at_most_three(a, b)
    })
    .ok_or_else(|| anyhow!("no path found"))
}

fn part_two(input: &str) -> Result<u32> {
    let grid = parse(input)?;
    grid.dijkstra(grid.begin(), grid.end(), |a, b| {
        grid.neighbours_at_least_four_at_most_ten(a, b)
    })
    .ok_or_else(|| anyhow!("no path found"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_dijkstra_part_1() {
        macro_rules! assert_dijkstra {
            ($input:expr, $expected:expr) => {
                let grid = parse($input).unwrap();
                assert_eq!(
                    grid.dijkstra(grid.begin(), grid.end(), |a, b| grid
                        .neighbours_at_most_three(a, b)),
                    $expected
                );
            };
        }
        assert_dijkstra!("911", Some(2));
        assert_dijkstra!("9111", Some(3));
        assert_dijkstra!("91111", None);
        assert_dijkstra!("91\n91", Some(2));
        assert_dijkstra!("91122\n99111", Some(5));
        assert_dijkstra!("91111\n99399\n11111", Some(8));
        assert_dijkstra!("11\n91\n11\n19\n11", Some(7));
        assert_dijkstra!("14999\n23111\n99991", Some(11));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 102);
    }

    #[test]
    fn test_dijkstra_part_2() {
        let grid =
            parse("111111111111\n999999999991\n999999999991\n999999999991\n999999999991").unwrap();
        assert_eq!(
            grid.dijkstra(grid.begin(), grid.end(), |a, b| grid
                .neighbours_at_least_four_at_most_ten(a, b)),
            Some(71)
        );
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 94);
    }
}
