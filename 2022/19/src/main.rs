use anyhow::{Context, Result};
use rayon::prelude::*;
use regex::{Captures, Regex};
use rustc_hash::FxHashMap;

fn main() -> Result<()> {
    let input = include_str!("input.txt");

    let answer = part_one(input).context("no solution for part one")?;
    println!("Part 1: {}", answer);
    assert_eq!(answer, 1624);

    let answer = part_two(input).context("no solution for part two")?;
    println!("Part 2: {}", answer);
    assert_eq!(answer, 12628);

    Ok(())
}

#[derive(PartialEq)]
enum MaterialType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Clone, Debug)]
struct Blueprint {
    id: usize,
    cost_ore_robot: Materials,
    cost_clay_robot: Materials,
    cost_obsidian_robot: Materials,
    cost_geode_robot: Materials,
}

impl Blueprint {
    fn try_create_robot(&self, which: MaterialType, available: &Materials) -> Option<Materials> {
        let cost = match which {
            MaterialType::Ore => &self.cost_ore_robot,
            MaterialType::Clay => &self.cost_clay_robot,
            MaterialType::Obsidian => &self.cost_obsidian_robot,
            MaterialType::Geode => &self.cost_geode_robot,
        };
        if cost.ore <= available.ore
            && cost.clay <= available.clay
            && cost.obsidian <= available.obsidian
            && cost.geodes <= available.geodes
        {
            Some(Materials {
                ore: available.ore - cost.ore,
                clay: available.clay - cost.clay,
                obsidian: available.obsidian - cost.obsidian,
                geodes: available.geodes - cost.geodes,
            })
        } else {
            None
        }
    }

    fn max_needed_of_material(&self, which: MaterialType) -> usize {
        let values = match which {
            MaterialType::Ore => [
                self.cost_ore_robot.ore,
                self.cost_clay_robot.ore,
                self.cost_obsidian_robot.ore,
                self.cost_geode_robot.ore,
            ],
            MaterialType::Clay => [
                self.cost_ore_robot.clay,
                self.cost_clay_robot.clay,
                self.cost_obsidian_robot.clay,
                self.cost_geode_robot.clay,
            ],
            MaterialType::Obsidian => [
                self.cost_ore_robot.obsidian,
                self.cost_clay_robot.obsidian,
                self.cost_obsidian_robot.obsidian,
                self.cost_geode_robot.obsidian,
            ],
            MaterialType::Geode => [usize::MAX, usize::MAX, usize::MAX, usize::MAX],
        };
        *values.iter().max().unwrap()
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
struct Materials {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geodes: usize,
}

impl Materials {
    fn add_mined_materials(&mut self, robots: &Robots) {
        self.ore += robots.ore;
        self.clay += robots.clay;
        self.obsidian += robots.obsidian;
        self.geodes += robots.geodes;
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
struct Robots {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geodes: usize,
}

#[derive(Clone)]
struct State {
    time_left: usize,
    materials: Materials,
    robots: Robots,
    blueprint: Blueprint,
    best_so_far: usize,
}

#[derive(Eq, Hash, PartialEq)]
struct CacheKey {
    time_left: usize,
    materials: Materials,
    robots: Robots,
}

fn parse(input: &str) -> Result<Vec<Blueprint>> {
    fn to_usize(caps: &Captures, index: usize) -> Result<usize> {
        caps.get(index)
            .unwrap()
            .as_str()
            .parse()
            .context("failed to convert to usize")
    }

    let mut blueprints = Vec::new();
    let regex = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore\. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay\. Each geode robot costs (\d+) ore and (\d+) obsidian\.").unwrap();
    for line in input.lines() {
        let caps = regex.captures(line).context("line does not match regex")?;
        let blueprint = Blueprint {
            id: to_usize(&caps, 1)?,
            cost_ore_robot: Materials {
                ore: to_usize(&caps, 2)?,
                ..Default::default()
            },
            cost_clay_robot: Materials {
                ore: to_usize(&caps, 3)?,
                ..Default::default()
            },
            cost_obsidian_robot: Materials {
                ore: to_usize(&caps, 4)?,
                clay: to_usize(&caps, 5)?,
                ..Default::default()
            },
            cost_geode_robot: Materials {
                ore: to_usize(&caps, 6)?,
                obsidian: to_usize(&caps, 7)?,
                ..Default::default()
            },
        };
        blueprints.push(blueprint);
    }
    Ok(blueprints)
}

fn find_best_outcome(mut state: State, cache: &mut FxHashMap<CacheKey, usize>) -> usize {
    debug_assert!(state.time_left > 0);

    state.time_left -= 1;

    if state.time_left == 0 {
        state.materials.add_mined_materials(&state.robots);
        return state.materials.geodes;
    }

    let cache_key = CacheKey {
        time_left: state.time_left,
        materials: state.materials.clone(),
        robots: state.robots.clone(),
    };

    if let Some(cached_value) = cache.get(&(cache_key)) {
        return *cached_value;
    }

    let upper_bound = {
        let t = state.time_left;
        state.materials.geodes + state.robots.geodes * t + (t * (t - 1) / 2)
    };

    let mut max = state.materials.geodes;

    if max + upper_bound < state.best_so_far {
        return 0;
    }

    if let Some(mut m) = state
        .blueprint
        .try_create_robot(MaterialType::Geode, &state.materials)
    {
        m.add_mined_materials(&state.robots);
        let mut robots = state.robots.clone();
        robots.geodes += 1;
        max = max.max(find_best_outcome(
            State {
                materials: m,
                robots,
                best_so_far: max.max(state.best_so_far),
                ..state.clone()
            },
            cache,
        ));
    }

    if state.robots.obsidian
        < state
            .blueprint
            .max_needed_of_material(MaterialType::Obsidian)
    {
        if let Some(mut m) = state
            .blueprint
            .try_create_robot(MaterialType::Obsidian, &state.materials)
        {
            m.add_mined_materials(&state.robots);
            let mut robots = state.robots.clone();
            robots.obsidian += 1;
            max = max.max(find_best_outcome(
                State {
                    materials: m,
                    robots,
                    best_so_far: max.max(state.best_so_far),
                    ..state.clone()
                },
                cache,
            ));
        }
    }

    if state.robots.clay < state.blueprint.max_needed_of_material(MaterialType::Clay) {
        if let Some(mut m) = state
            .blueprint
            .try_create_robot(MaterialType::Clay, &state.materials)
        {
            m.add_mined_materials(&state.robots);
            let mut robots = state.robots.clone();
            robots.clay += 1;
            max = max.max(find_best_outcome(
                State {
                    materials: m,
                    robots,
                    best_so_far: max.max(state.best_so_far),
                    ..state.clone()
                },
                cache,
            ));
        }
    }

    if state.robots.ore < state.blueprint.max_needed_of_material(MaterialType::Ore) {
        if let Some(mut m) = state
            .blueprint
            .try_create_robot(MaterialType::Ore, &state.materials)
        {
            m.add_mined_materials(&state.robots);
            let mut robots = state.robots.clone();
            robots.ore += 1;
            max = max.max(find_best_outcome(
                State {
                    materials: m,
                    robots,
                    best_so_far: max.max(state.best_so_far),
                    ..state.clone()
                },
                cache,
            ));
        }
    }

    state.materials.add_mined_materials(&state.robots);
    state.best_so_far = max.max(state.best_so_far);
    max = max.max(find_best_outcome(state, cache));
    cache.insert(cache_key, max);

    max
}

fn part_one(input: &str) -> Result<usize> {
    let sum = parse(input)?
        .par_iter()
        .map(|blueprint| {
            let id = blueprint.id;
            let score = find_best_outcome(
                State {
                    time_left: 24,
                    materials: Default::default(),
                    robots: Robots {
                        ore: 1,
                        ..Default::default()
                    },
                    blueprint: blueprint.clone(),
                    best_so_far: 0,
                },
                &mut FxHashMap::default(),
            );
            id * score
        })
        .sum();
    Ok(sum)
}

fn part_two(input: &str) -> Result<usize> {
    let blueprints: Vec<_> = parse(input)?.into_iter().take(3).collect();
    let product = blueprints
        .par_iter()
        .map(|blueprint| {
            find_best_outcome(
                State {
                    time_left: 32,
                    materials: Default::default(),
                    robots: Robots {
                        ore: 1,
                        ..Default::default()
                    },
                    blueprint: blueprint.clone(),
                    best_so_far: 0,
                },
                &mut FxHashMap::default(),
            )
        })
        .product();
    Ok(product)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 33);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 56 * 62);
    }
}
