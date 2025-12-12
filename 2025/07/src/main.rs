use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 1524)?;
    aoc::run!(part_two(input), 32982105837605)?;
    Ok(())
}

fn part_one(input: &str) -> Result<usize> {
    let columns = input.find('\n').ok_or_else(|| anyhow!("bad input"))?;
    let rows = input.lines().count();
    let mut grid = input.as_bytes().to_vec();
    let mut count = 0;

    // SAFETY: input guaranteed to have an ... line after any line with S or ^, and any S or ^ is
    // padded with .
    for y in 0..(rows - 1) {
        for x in 0..columns {
            let offset = x + y * (columns + 1);
            let next_offset = x + (y + 1) * (columns + 1);
            let ch = grid[offset];
            let next_ch = grid[next_offset];
            if ch == b'|' || ch == b'S' {
                if next_ch == b'^' {
                    grid[next_offset - 1] = b'|';
                    grid[next_offset + 1] = b'|';
                    count += 1;
                } else {
                    grid[next_offset] = b'|';
                }
            }
        }
    }

    Ok(count)
}

fn part_two(input: &str) -> Result<usize> {
    let columns = input.find('\n').ok_or_else(|| anyhow!("bad input"))?;
    let rows = input.lines().count();
    let mut grid = input.as_bytes().to_vec();

    // SAFETY: input guaranteed to have an ... line after any line with S or ^, and any S or ^ is
    // padded with .
    for y in 0..(rows - 1) {
        for x in 0..columns {
            let offset = x + y * (columns + 1);
            let next_offset = x + (y + 1) * (columns + 1);
            let ch = grid[offset];
            let next_ch = grid[next_offset];
            if ch == b'|' || ch == b'S' {
                if next_ch == b'^' {
                    grid[next_offset - 1] = b'|';
                    grid[next_offset + 1] = b'|';
                } else {
                    grid[next_offset] = b'|';
                }
            }
        }
    }

    let mut count = vec![0; columns];
    for x in 0..columns {
        let y = columns - 1;
        if grid[x + y * (columns + 1)] == b'|' {
            count[x] = 1;
        }
    }
    for y in (0..(rows - 1)).rev() {
        for x in 0..columns {
            let ch = grid[x + y * (columns + 1)];
            if ch == b'^' {
                count[x] = count[x - 1] + count[x + 1];
            }
        }
    }
    let start = grid
        .iter()
        .position(|b| *b == b'S')
        .ok_or_else(|| anyhow!("could not find S"))?;
    Ok(count[start])
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 21);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 40);
    }
}
