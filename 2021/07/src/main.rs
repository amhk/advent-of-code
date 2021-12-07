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
}

fn parse_input(input: &str) -> Result<Vec<u32>, Error> {
    let values: Result<Vec<_>, _> = input.trim().split(',').map(|s| s.parse::<u32>()).collect();
    values.map_err(|_| Error::BadInput)
}

fn cost_for_index_part_one(values: &[u32], index: u32) -> u32 {
    // u32::abs_diff is nightly only at the time of writing; explicitly spell it out with an
    // if statement instead
    values.iter().fold(0, |acc, v| {
        acc + if index < *v { *v - index } else { index - *v }
    })
}

fn cost_for_index_part_two(values: &[u32], index: u32) -> u32 {
    values.iter().fold(0, |acc, v| {
        let n = if index < *v { *v - index } else { index - *v };
        // 1 + 2 + 3 + ... + n = (n * (n + 1)) / 2
        acc + (n * (n + 1)) / 2
    })
}

fn find_minumum_cost<F>(values: &[u32], func: F) -> Result<u32, Error>
where
    F: std::ops::Fn(&[u32], u32) -> u32,
{
    let left = *values.iter().min().ok_or(Error::BadInput)?;
    let mut left = (left, func(values, left));

    let right = *values.iter().max().ok_or(Error::BadInput)?;
    let mut right = (right, func(values, right));

    loop {
        let middle = left.0 + (right.0 - left.0) / 2;
        let middle = (middle, func(values, middle));
        if left.1 < right.1 {
            right = middle;
        } else {
            left = middle;
        }

        // `while left.0 < right.1` can get stuck when indices are one apart, e.g. 4 and 5 (left
        // will always update to itself); explicitly return if 1 or less indices apart
        if left.0 + 1 >= right.0 {
            return Ok(std::cmp::min(left.1, right.1));
        }
    }
}

fn part_one(input: &str) -> Result<u32, Error> {
    let values = parse_input(input)?;
    find_minumum_cost(&values, cost_for_index_part_one)
}

fn part_two(input: &str) -> Result<u32, Error> {
    let values = parse_input(input)?;
    find_minumum_cost(&values, cost_for_index_part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_cost_for_index_part_one() {
        let values = parse_input(INPUT).unwrap();
        assert_eq!(cost_for_index_part_one(&values, 1), 41);
        assert_eq!(cost_for_index_part_one(&values, 2), 37);
        assert_eq!(cost_for_index_part_one(&values, 3), 39);
        assert_eq!(cost_for_index_part_one(&values, 10), 71);
    }

    #[test]
    fn test_cost_for_index_part_two() {
        let values = parse_input(INPUT).unwrap();
        assert_eq!(cost_for_index_part_two(&values, 2), 206);
        assert_eq!(cost_for_index_part_two(&values, 5), 168);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), Ok(37));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), Ok(168));
    }
}
