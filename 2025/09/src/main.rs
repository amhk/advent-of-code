use std::{mem::swap, ops::Range};

use anyhow::{anyhow, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 4781235324)?;
    aoc::run!(part_two(input), 1566935900)?;
    Ok(())
}

fn parse(input: &str) -> Result<Vec<(i32, i32)>> {
    let mut out = vec![];
    for line in input.lines() {
        let (a, b) = line.split_once(',').ok_or_else(|| anyhow!("bad input"))?;
        let a = a.parse()?;
        let b = b.parse()?;
        out.push((a, b));
    }
    Ok(out)
}

fn find_max_area<T: Fn((i32, i32), (i32, i32)) -> bool>(
    points: &[(i32, i32)],
    predicate: T,
) -> usize {
    let mut max_area = 0;
    for pair in points.iter().combinations(2) {
        if !predicate(*pair[0], *pair[1]) {
            continue;
        }
        let x = (pair[0].0 - pair[1].0).unsigned_abs() as usize + 1;
        let y = (pair[0].1 - pair[1].1).unsigned_abs() as usize + 1;
        let area = x * y;
        if area > max_area {
            max_area = area;
        }
    }
    max_area
}

fn part_one(input: &str) -> Result<usize> {
    let points = parse(input)?;
    Ok(find_max_area(&points, |_, _| true))
}

fn part_two(input: &str) -> Result<usize> {
    #[derive(Debug)]
    enum LineSegment {
        Horizontal(i32, Range<i32>),
        Vertical(i32, Range<i32>),
    }

    let points = parse(input)?;
    let mut outline = vec![];
    for (a, b) in points.iter().circular_tuple_windows() {
        if a.1 == b.1 {
            let y = a.1;
            let ax = a.0;
            let bx = b.0;
            outline.push(LineSegment::Horizontal(y, ax.min(bx)..ax.max(bx)));
        } else {
            let x = a.0;
            let ay = a.1;
            let by = b.1;
            outline.push(LineSegment::Vertical(x, ay.min(by)..ay.max(by)));
        }
    }

    let lambda = |(mut x0, mut y0), (mut x1, mut y1)| {
        if x0 > x1 {
            swap(&mut x0, &mut x1);
        }
        if y0 > y1 {
            swap(&mut y0, &mut y1);
        }

        for (x, y) in [(x0, y0), (x0, y1), (x1, y0), (x1, y1)] {
            #[allow(clippy::if_same_then_else)]
            let inside_polygon: bool = if points.contains(&(x, y)) {
                true
            } else if outline.iter().any(|segment| match segment {
                LineSegment::Horizontal(sy, range) => y == *sy && range.contains(&x),
                LineSegment::Vertical(sx, range) => x == *sx && range.contains(&y),
            }) {
                true
            } else {
                outline
                    .iter()
                    .filter(|segment| match segment {
                        LineSegment::Horizontal(_, _) => false,
                        LineSegment::Vertical(segment_x, range) => {
                            *segment_x > x && range.contains(&y)
                        }
                    })
                    .count()
                    % 2
                    != 0
            };
            // if any of the points is outside the polygon, this combination of coordinates can't
            // be used, so quiet early
            if !inside_polygon {
                return false;
            }
        }

        let intersects_polygon = outline.iter().any(|segment| match segment {
            LineSegment::Horizontal(y, range) => {
                y0 < *y && *y < y1 && //(range.start < x0 && (range.end + 1) >= x1)
                    ((x0 > range.start && x0 < range.end) ||
                     (x1 > range.start && x1 < range.end)
                    )
            }
            LineSegment::Vertical(x, range) => {
                x0 < *x && *x < x1 && //(range.start <= y0 || (range.end + 1) >= y1)
                    ((y0 > range.start && y0 < range.end) ||
                     (y1 > range.start && y1 < range.end))
            }
        });
        !intersects_polygon
    };

    Ok(find_max_area(&points, lambda))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 50);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 24);
    }
}
