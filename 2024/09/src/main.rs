use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 6471961544878)?;
    aoc::run!(part_two(input), 6511178035564)?;
    Ok(())
}

fn parse(input: &str) -> Result<Vec<Option<usize>>> {
    let mut v = vec![];
    let mut id = 0;
    for (i, ch) in input.trim().chars().enumerate() {
        let value = ch
            .to_digit(10)
            .ok_or_else(|| anyhow!("failed to convert char '{ch}' to u32"))?;
        if i % 2 == 0 {
            for _ in 0..value {
                v.push(Some(id));
            }
            id += 1;
        } else {
            for _ in 0..value {
                v.push(None);
            }
        }
    }
    Ok(v)
}

fn checksum(array: &[Option<usize>]) -> usize {
    array
        .iter()
        .enumerate()
        .filter_map(|(i, id)| id.as_ref().map(|id| i * id))
        .sum()
}

fn part_one(input: &str) -> Result<usize> {
    let mut v = parse(input)?;
    let mut left = 0;
    let mut right = v.len() - 1;
    while left < right {
        if v[left].is_none() {
            v.swap(left, right);
            right -= 1;
        } else {
            left += 1;
        }
    }
    Ok(checksum(&v))
}

fn part_two(input: &str) -> Result<usize> {
    fn find_first_empty_chunk(array: &[Option<usize>], chunk_size: usize) -> Option<usize> {
        'outer: for i in 0..array.len() - chunk_size {
            for j in 0..chunk_size {
                if array[i + j].is_some() {
                    continue 'outer;
                }
            }
            return Some(i);
        }
        None
    }

    let mut v = parse(input)?;
    let mut right = v.len() - 1;

    loop {
        while v[right].is_none() {
            if right == 0 {
                break;
            }
            right -= 1;
        }

        // find candidate to move (will be &v[r..=right])
        let mut r = right;
        while r > 0 && v[r - 1] == v[right] {
            r -= 1;
        }

        // we are to move `chunk_size` items
        let chunk_size = right - r + 1;

        // find next chunk of available space of size `len` (will be &v[left..=l])
        if let Some(mut index) = find_first_empty_chunk(&v[..=right], chunk_size) {
            for _ in 0..chunk_size {
                v.swap(index, r);
                index += 1;
                r += 1;
            }
        }
        if right >= chunk_size {
            right -= chunk_size;
        } else {
            break;
        }
    }

    Ok(checksum(&v))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_checksum() {
        fn to_vec(input: &str) -> Vec<Option<usize>> {
            input
                .chars()
                .map(|ch| match ch {
                    ch if ch.is_ascii_digit() => Some(ch.to_digit(10).unwrap() as usize),
                    '.' => None,
                    _ => panic!(),
                })
                .collect()
        }
        assert_eq!(
            checksum(&to_vec("0099811188827773336446555566..............")),
            1928
        );
        assert_eq!(
            checksum(&to_vec("00992111777.44.333....5555.6666.....8888..")),
            2858
        );
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 1928);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 2858);
    }
}
