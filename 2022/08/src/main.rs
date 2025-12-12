use anyhow::{ensure, Context, Result};
use std::collections::HashSet;

struct Grid {
    cells: Vec<Vec<u32>>, // [row][col]
}

impl Grid {
    fn iter_right(&self, row: usize) -> impl Iterator<Item = u32> + '_ {
        self.cells[row].iter().copied()
    }

    fn iter_left(&self, row: usize) -> impl Iterator<Item = u32> + '_ {
        self.cells[row].iter().rev().copied()
    }

    fn iter_down(&self, column: usize) -> impl Iterator<Item = u32> + '_ {
        let mut row: usize = 0;
        std::iter::from_fn(move || {
            if row >= self.rows() {
                return None;
            }
            let value = self.cells[row][column];
            row += 1;
            Some(value)
        })
    }

    fn iter_up(&self, column: usize) -> impl Iterator<Item = u32> + '_ {
        let mut row: usize = 0;
        std::iter::from_fn(move || {
            if row >= self.rows() {
                return None;
            }
            let value = self.cells[self.rows() - row - 1][column];
            row += 1;
            Some(value)
        })
    }

    fn rows(&self) -> usize {
        self.cells.len()
    }

    fn columns(&self) -> usize {
        self.cells[0].len()
    }

    fn scenic_score(&self, col: usize, row: usize) -> usize {
        let height = self.cells[row][col];
        let mut score = 1;

        // look up
        let mut count = 0;
        for r in (0..row).rev() {
            count += 1;
            if self.cells[r][col] >= height {
                break;
            }
        }
        score *= count;

        // look down
        let mut count = 0;
        for r in (row + 1)..self.rows() {
            count += 1;
            if self.cells[r][col] >= height {
                break;
            }
        }
        score *= count;

        // look left
        let mut count = 0;
        for c in (0..col).rev() {
            count += 1;
            if self.cells[row][c] >= height {
                break;
            }
        }
        score *= count;

        // look right
        let mut count = 0;
        for c in (col + 1)..self.columns() {
            count += 1;
            if self.cells[row][c] >= height {
                break;
            }
        }
        score *= count;

        score
    }
}

impl TryFrom<&str> for Grid {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut cells = Vec::new();
        for line in s.lines() {
            let row: Vec<_> = line
                .chars()
                .map(|ch| {
                    ch.to_digit(10)
                        .with_context(|| format!("failed to convert '{}' to usize", ch))
                })
                .collect::<Result<Vec<_>, _>>()
                .with_context(|| format!("failed to convert '{}' to Vec<usize>", line))?;
            cells.push(row);
        }
        ensure!(!cells.is_empty(), "empty grid");
        for row in &cells {
            ensure!(row.len() == cells[0].len(), "cell rows not of equal size");
        }
        Ok(Grid { cells })
    }
}

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 1845)?;
    aoc::run!(part_two(input), 230112)?;
    Ok(())
}

fn visible_trees_indices(iter: impl Iterator<Item = u32>) -> impl Iterator<Item = usize> {
    let mut max: Option<u32> = None;
    iter.enumerate().filter_map(move |(index, value)| {
        if max.is_none_or(|max| max < value) {
            max = Some(value);
            Some(index)
        } else {
            None
        }
    })
}

fn part_one(input: &str) -> Result<usize> {
    let grid: Grid = input.try_into()?;
    let mut distinct_trees: HashSet<(usize, usize)> = HashSet::new();
    for row in 0..grid.rows() {
        visible_trees_indices(grid.iter_right(row)).for_each(|col| {
            distinct_trees.insert((row, col));
        });
        visible_trees_indices(grid.iter_left(row))
            .map(|col| grid.columns() - col - 1)
            .for_each(|col| {
                distinct_trees.insert((row, col));
            });
    }
    for col in 0..grid.columns() {
        visible_trees_indices(grid.iter_down(col)).for_each(|row| {
            distinct_trees.insert((row, col));
        });
        visible_trees_indices(grid.iter_up(col))
            .map(|row| grid.rows() - row - 1)
            .for_each(|row| {
                distinct_trees.insert((row, col));
            });
    }
    Ok(distinct_trees.len())
}

fn part_two(input: &str) -> Result<usize> {
    let grid: Grid = input.try_into()?;
    let mut max = 0;
    for c in 0..grid.columns() {
        for r in 0..grid.rows() {
            let score = grid.scenic_score(c, r);
            if score > max {
                max = score;
            }
        }
    }
    Ok(max)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_grid() {
        let grid: Grid = INPUT.try_into().unwrap();
        assert_eq!(grid.rows(), 5);
        assert_eq!(grid.columns(), 5);
        assert_eq!(grid.iter_right(1).collect::<Vec<_>>(), [2, 5, 5, 1, 2]);
        assert_eq!(grid.iter_left(1).collect::<Vec<_>>(), [2, 1, 5, 5, 2]);
        assert_eq!(grid.iter_down(1).collect::<Vec<_>>(), [0, 5, 5, 3, 5]);
        assert_eq!(grid.iter_up(1).collect::<Vec<_>>(), [5, 3, 5, 5, 0]);
        assert_eq!(grid.scenic_score(2, 1), 4);
        assert_eq!(grid.scenic_score(2, 3), 8);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 21);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 8);
    }
}
