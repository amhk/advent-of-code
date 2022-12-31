use anyhow::{Context, Result};
use rustc_hash::{FxHashMap, FxHashSet};

fn main() -> Result<()> {
    let input = include_str!("input.txt");

    let answer = part_one(input).context("no solution for part one")?;
    println!("Part 1: {}", answer);
    assert_eq!(answer, 4052);

    let answer = part_two(input).context("no solution for part two")?;
    println!("Part 2: {}", answer);
    assert_eq!(answer, 978);

    Ok(())
}

type XY = (i32, i32);

type Check = dyn Fn(&FxHashSet<XY>, XY) -> Option<XY>;

struct Grid {
    cells: FxHashSet<XY>,
    checks: [Box<Check>; 5],
}

impl Grid {
    fn has_neighbour(&self, (x, y): XY) -> bool {
        for (i, j) in [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ] {
            if self.cells.contains(&(x + i, y + j)) {
                return true;
            }
        }
        false
    }

    fn next(self) -> Self {
        let mut proposals: FxHashMap<XY, XY> = FxHashMap::default();

        for (x, y) in self.cells.iter() {
            if !self.has_neighbour((*x, *y)) {
                proposals.insert((*x, *y), (*x, *y));
                continue;
            }
            for check in self.checks.iter() {
                if let Some(dest) = check(&self.cells, (*x, *y)) {
                    proposals.insert((*x, *y), dest);
                    break;
                }
            }
        }

        let mut next = FxHashSet::default();
        for (src, dest) in proposals.iter() {
            let count = proposals.values().filter(|xy| xy == &dest).count();
            next.insert(if count == 1 { *dest } else { *src });
        }
        debug_assert_eq!(self.cells.len(), next.len());

        let [a, b, c, d, e] = self.checks;
        Grid {
            cells: next,
            checks: [b, c, d, a, e],
        }
    }

    fn score(&self) -> Result<usize> {
        let bb = self.bounding_box()?;
        let width = bb.0 .0.abs_diff(bb.1 .0) as usize + 1;
        let height = bb.0 .1.abs_diff(bb.1 .1) as usize + 1;
        Ok(width * height - self.cells.len())
    }

    fn bounding_box(&self) -> Result<(XY, XY)> {
        let min_x = self
            .cells
            .iter()
            .map(|(x, _)| x)
            .min()
            .context("empty grid")?;
        let min_y = self
            .cells
            .iter()
            .map(|(_, y)| y)
            .min()
            .context("empty grid")?;
        let max_x = self
            .cells
            .iter()
            .map(|(x, _)| x)
            .max()
            .context("empty grid")?;
        let max_y = self
            .cells
            .iter()
            .map(|(_, y)| y)
            .max()
            .context("empty grid")?;
        Ok(((*min_x, *min_y), (*max_x, *max_y)))
    }
}

impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bb = self.bounding_box().unwrap();
        let mut s = String::new();
        for y in bb.0 .1..=bb.1 .1 {
            for x in bb.0 .0..=bb.1 .0 {
                s.push(if self.cells.contains(&(x, y)) {
                    '#'
                } else {
                    '.'
                });
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

impl TryFrom<&str> for Grid {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut cells = FxHashSet::default();
        for (y, line) in value.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch == '#' {
                    cells.insert((x as i32, y as i32));
                }
            }
        }
        Ok(Grid {
            cells,
            checks: [
                // north
                Box::new(|cells, (x, y)| {
                    if !cells.contains(&(x - 1, y - 1))
                        && !cells.contains(&(x, y - 1))
                        && !cells.contains(&(x + 1, y - 1))
                    {
                        Some((x, y - 1))
                    } else {
                        None
                    }
                }),
                // south
                Box::new(|cells, (x, y)| {
                    if !cells.contains(&(x - 1, y + 1))
                        && !cells.contains(&(x, y + 1))
                        && !cells.contains(&(x + 1, y + 1))
                    {
                        Some((x, y + 1))
                    } else {
                        None
                    }
                }),
                // west
                Box::new(|cells, (x, y)| {
                    if !cells.contains(&(x - 1, y - 1))
                        && !cells.contains(&(x - 1, y))
                        && !cells.contains(&(x - 1, y + 1))
                    {
                        Some((x - 1, y))
                    } else {
                        None
                    }
                }),
                // east
                Box::new(|cells, (x, y)| {
                    if !cells.contains(&(x + 1, y - 1))
                        && !cells.contains(&(x + 1, y))
                        && !cells.contains(&(x + 1, y + 1))
                    {
                        Some((x + 1, y))
                    } else {
                        None
                    }
                }),
                // fallback: stay in place
                Box::new(|_, (x, y)| Some((x, y))),
            ],
        })
    }
}

fn part_one(input: &str) -> Result<usize> {
    let mut grid = Grid::try_from(input)?;
    for _ in 0..10 {
        grid = grid.next();
    }
    grid.score()
}

fn part_two(input: &str) -> Result<usize> {
    let mut grid = Grid::try_from(input)?;
    let mut previous = grid.cells.clone();
    for i in 1.. {
        grid = grid.next();
        if previous == grid.cells {
            return Ok(i);
        }
        previous = grid.cells.clone();
    }
    panic!();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_minimal_input() {
        let mut grid = Grid::try_from(".....\n..##.\n..#..\n.....\n..##.\n.....\n").unwrap();
        assert_eq!(grid.score().unwrap(), 3);

        grid = grid.next();
        assert_eq!(grid.score().unwrap(), 5);

        grid = grid.next();
        assert_eq!(grid.score().unwrap(), 15);

        grid = grid.next();
        assert_eq!(grid.score().unwrap(), 25);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 110);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 20);
    }
}
