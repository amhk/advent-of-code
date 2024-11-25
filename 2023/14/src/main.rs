use anyhow::{anyhow, Result};
use std::collections::BTreeMap;

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 113486)?;
    aoc::run!(part_two(input), 104409)?;
    Ok(())
}

type XY = (i32, i32);

type Grid = BTreeMap<XY, State>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Empty,
    Cube,
    Sphere,
}

impl TryFrom<char> for State {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Empty),
            '#' => Ok(Self::Cube),
            'O' => Ok(Self::Sphere),
            _ => Err(anyhow!("can not convert '{value}' to State")),
        }
    }
}

fn parse(input: &str) -> Result<Grid> {
    let mut grid = Grid::new();
    let max_x = input
        .lines()
        .next()
        .ok_or_else(|| anyhow!("empty input"))?
        .len() as i32;
    let max_y = (input.lines().count()) as i32;
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            grid.insert((x as i32, y as i32), ch.try_into()?);
        }
        grid.insert((-1, y as i32), State::Cube);
        grid.insert((max_x, y as i32), State::Cube);
    }
    for x in -1..=max_x {
        grid.insert((x, -1), State::Cube);
        grid.insert((x, max_y), State::Cube);
    }
    Ok(grid)
}

fn move_spheres<F>(grid: &mut Grid, func: F)
where
    F: Fn(&XY) -> XY,
{
    let keys: Vec<XY> = grid
        .iter()
        .filter(|(_, state)| **state != State::Cube)
        .map(|(xy, _)| *xy)
        .collect();
    let mut movement = true;
    while movement {
        movement = false;
        for xy in keys.iter() {
            if grid[xy] != State::Sphere {
                continue;
            }
            let next = func(xy);
            if grid[&next] != State::Empty {
                continue;
            }
            movement = true;
            *grid.get_mut(xy).unwrap() = State::Empty;
            *grid.get_mut(&next).unwrap() = State::Sphere;
        }
    }
}

fn move_spheres_north(grid: &mut Grid) {
    move_spheres(grid, |xy| (xy.0, xy.1 - 1))
}

fn move_spheres_east(grid: &mut Grid) {
    move_spheres(grid, |xy| (xy.0 + 1, xy.1))
}

fn move_spheres_south(grid: &mut Grid) {
    move_spheres(grid, |xy| (xy.0, xy.1 + 1))
}

fn move_spheres_west(grid: &mut Grid) {
    move_spheres(grid, |xy| (xy.0 - 1, xy.1))
}

fn score(grid: &Grid) -> usize {
    let mut sum = 0;
    let height = (*grid.keys().map(|(_, y)| y).max().unwrap()) as usize;
    for y in 0..height {
        sum += (height - y)
            * grid
                .iter()
                .filter(|(xy, state)| xy.1 == y as i32 && **state == State::Sphere)
                .count();
    }
    sum
}

fn part_one(input: &str) -> Result<usize> {
    let mut grid = parse(input)?;
    move_spheres_north(&mut grid);
    Ok(score(&grid))
}

fn part_two(input: &str) -> Result<usize> {
    fn move_spheres_four_directions(grid: &mut Grid) {
        move_spheres_north(grid);
        move_spheres_west(grid);
        move_spheres_south(grid);
        move_spheres_east(grid);
    }

    fn fingerprint(grid: &Grid) -> Vec<XY> {
        let mut v = grid
            .iter()
            .filter(|(_, state)| **state == State::Sphere)
            .map(|(xy, _)| xy)
            .cloned()
            .collect::<Vec<_>>();
        v.sort();
        v
    }

    let mut grid = parse(input)?;

    // iterate for a while (100 times, chosen arbitrarily) to get out of initial state
    for _ in 0..100 {
        move_spheres_four_directions(&mut grid);
    }

    // assume the will be a cycle, detect its length
    let fingerprint_at_100 = fingerprint(&grid);
    let mut cycle_len = 0;
    loop {
        cycle_len += 1;
        move_spheres_four_directions(&mut grid);
        if fingerprint(&grid) == fingerprint_at_100 {
            break;
        }
    }

    // skip full cycles, only run the remainder of the final cycle
    let iterations_left = (1_000_000_000 - 100) % cycle_len;
    for _ in 0..iterations_left {
        move_spheres_four_directions(&mut grid);
    }

    Ok(score(&grid))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 136);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 64);
    }
}
