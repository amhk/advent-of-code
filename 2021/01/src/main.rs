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

fn parse_input(input: &str) -> Result<Vec<usize>, Error> {
    let values: Result<Vec<_>, _> = input
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>())
        .collect();
    values.map_err(|_| Error::BadInput)
}

fn part_one(input: &str) -> Result<usize, Error> {
    let values = parse_input(input)?;
    Ok(values
        .iter()
        .zip(values.iter().skip(1))
        .filter(|(a, b)| a < b)
        .count())
}

fn part_two(input: &str) -> Result<usize, Error> {
    let values = parse_input(input)?;
    let iter1 = values.windows(3).map(|slice| slice.iter().sum::<usize>());
    let iter2 = iter1.clone().skip(1);
    Ok(iter1.zip(iter2).filter(|(a, b)| a < b).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), Ok(7));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), Ok(5));
    }
}
