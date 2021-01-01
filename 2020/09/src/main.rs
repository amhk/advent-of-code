fn main() {
    let input = include_str!("input.txt");

    let answer = part_one(&input, 25).expect("no solution found for part one");
    println!("part 1: {}", answer);

    let answer = part_two(&input, 25).expect("no solution found for part one");
    println!("part 2: {}", answer);
}

fn no_two_numbers_add_up_to(input: &(&[u64], &u64)) -> bool {
    let (slice, value) = input;
    for i in 0..(slice.len() - 1) {
        for j in (i + 1)..slice.len() {
            if slice[i] + slice[j] == **value {
                return false;
            }
        }
    }
    true
}

fn find_contiguous_slice_with_sum(slice: &[u64], value: u64) -> Option<&[u64]> {
    for i in 0..(slice.len() - 1) {
        let mut sum = 0;
        for j in (i + 1)..slice.len() {
            sum += slice[j];
            if sum == value {
                return Some(&slice[i + 1..=j]);
            }
        }
    }
    None
}

fn part_one(input: &str, window_size: usize) -> Result<u64, Error> {
    let values = input
        .split_whitespace()
        .map(|s| s.parse::<u64>())
        .filter_map(Result::ok)
        .collect::<Vec<_>>();
    let mut iter = ChunkIterator::new(&values, window_size)?;
    iter.find(no_two_numbers_add_up_to)
        .map(|pair| *pair.1)
        .ok_or(Error::NoSolution)
}

fn part_two(input: &str, window_size: usize) -> Result<u64, Error> {
    let expected_sum = part_one(input, window_size)?;
    let values = input
        .split_whitespace()
        .map(|s| s.parse::<u64>())
        .filter_map(Result::ok)
        .collect::<Vec<_>>();
    let slice = find_contiguous_slice_with_sum(&values, expected_sum).ok_or(Error::NoSolution)?;
    assert_eq!(slice.iter().sum::<u64>(), expected_sum);

    let min = slice.iter().min().ok_or(Error::NoSolution)?;
    let max = slice.iter().max().ok_or(Error::NoSolution)?;
    Ok(min + max)
}

#[derive(Debug)]
enum Error {
    InvalidArguments,
    NoSolution,
}

struct ChunkIterator<'a, T> {
    slice: &'a [T],
    begin: usize,
    end: usize,
}

impl<'a, T> ChunkIterator<'a, T> {
    fn new(slice: &'a [T], window_size: usize) -> Result<ChunkIterator<'a, T>, Error> {
        if window_size < 2 {
            return Err(Error::InvalidArguments);
        }
        Ok(ChunkIterator {
            slice,
            begin: 0,
            end: window_size,
        })
    }
}

impl<'a, T> Iterator for ChunkIterator<'a, T> {
    type Item = (&'a [T], &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.end >= self.slice.len() {
            return None;
        }
        assert!(self.end - self.begin > 1);
        let pair = (&self.slice[self.begin..self.end], &self.slice[self.end]);
        self.begin += 1;
        self.end += 1;
        Some(pair)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_chunk_iter() {
        let v = vec!['a', 'b', 'c', 'd'];
        let mut iter = ChunkIterator::new(&v, 2).unwrap();
        assert_eq!(iter.next(), Some((&v[0..=1], &'c')));
        assert_eq!(iter.next(), Some((&v[1..=2], &'d')));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT, 5).unwrap(), 127);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT, 5).unwrap(), 62);
    }
}
