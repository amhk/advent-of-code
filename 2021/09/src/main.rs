use std::collections::BTreeSet;
use std::ops::Index;

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
    TooFewBasins,
}

struct Grid {
    columns: usize,
    rows: usize,
    grid: Vec<u32>,
}

impl TryFrom<&str> for Grid {
    type Error = Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut columns = None;
        let mut rows = 0;
        let mut grid = vec![];
        for line in input.lines() {
            let current_width = line.len();
            if let Some(w) = columns {
                if current_width != w {
                    return Err(Error::BadInput);
                }
            } else {
                columns = Some(current_width);
            }

            rows += 1;

            for ch in line.chars() {
                grid.push(ch.to_digit(10).ok_or(Error::BadInput)?);
            }
        }

        Ok(Grid {
            columns: columns.ok_or(Error::BadInput)?,
            rows,
            grid,
        })
    }
}

impl Index<(i32, i32)> for Grid {
    type Output = u32;

    fn index(&self, index: (i32, i32)) -> &Self::Output {
        if index.0 < 0 || index.0 >= (self.columns as i32) {
            return &u32::MAX;
        }
        if index.1 < 0 || index.1 >= (self.rows as i32) {
            return &u32::MAX;
        }
        let i = index.0 + index.1 * (self.columns as i32);
        self.grid.get(i as usize).unwrap()
    }
}

fn find_lowest_points(grid: &Grid) -> Vec<(i32, i32)> {
    let mut out = vec![];
    for x in 0..grid.columns as i32 {
        for y in 0..grid.rows as i32 {
            let value = grid[(x, y)];
            if value < grid[(x - 1, y)]
                && value < grid[(x + 1, y)]
                && value < grid[(x, y - 1)]
                && value < grid[(x, y + 1)]
            {
                out.push((x, y));
            }
        }
    }
    out
}

// BTreeSet.pop_first is nightly only, so implement our own
fn pop_first<T: std::cmp::Ord + Copy>(set: &mut BTreeSet<T>) -> Option<T> {
    let first = *set.iter().next()?;
    set.remove(&first);
    Some(first)
}

fn part_one(input: &str) -> Result<u32, Error> {
    let grid = Grid::try_from(input)?;
    Ok(find_lowest_points(&grid).iter().map(|p| grid[*p] + 1).sum())
}

fn part_two(input: &str) -> Result<usize, Error> {
    let grid = Grid::try_from(input)?;
    let mut basin_sizes = vec![];

    for p in find_lowest_points(&grid) {
        let mut visited: BTreeSet<(i32, i32)> = BTreeSet::new();
        let mut worklist: BTreeSet<(i32, i32)> = BTreeSet::new();
        worklist.insert(p);
        while !worklist.is_empty() {
            // p: point, pv: point value
            // n: (point's) neighbour, nv: neighbour's value
            let p = pop_first(&mut worklist).unwrap();
            let pv = grid[p];
            visited.insert(p);

            for n in [
                (p.0 - 1, p.1),
                (p.0 + 1, p.1),
                (p.0, p.1 - 1),
                (p.0, p.1 + 1),
            ] {
                let nv = grid[n];
                if nv >= 9 || visited.contains(&n) {
                    continue;
                }
                if pv < nv {
                    worklist.insert(n);
                }
            }
        }
        basin_sizes.push(visited.len());
    }

    if basin_sizes.len() < 3 {
        return Err(Error::TooFewBasins);
    }
    basin_sizes.sort_unstable();
    Ok(basin_sizes.iter().rev().take(3).product())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_grid() {
        let grid = Grid::try_from(INPUT).unwrap();
        assert_eq!(grid.columns, 10);
        assert_eq!(grid.rows, 5);
        assert_eq!(grid.grid.len(), 50);
        assert_eq!(grid[(-1, -1)], u32::MAX);
        assert_eq!(grid[(0, 0)], 2);
        assert_eq!(grid[(9, 0)], 0);
        assert_eq!(grid[(10, 0)], u32::MAX);
        assert_eq!(grid[(0, 1)], 3);
        assert_eq!(grid[(10, 10)], u32::MAX);
    }

    #[test]
    fn test_find_lowest_points() {
        let grid = Grid::try_from(INPUT).unwrap();
        let mut points = find_lowest_points(&grid);
        points.sort_unstable();
        assert_eq!(points, [(1, 0), (2, 2), (6, 4), (9, 0)]);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), Ok(15));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), Ok(1134));
    }
}
