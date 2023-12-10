use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 2101499000)?;
    aoc::run!(part_two(input), 1089)?;
    Ok(())
}

fn parse(input: &str) -> Result<Vec<Vec<i32>>> {
    let mut all_sequences = vec![];
    for line in input.lines() {
        let sequence: Vec<i32> = line
            .split(' ')
            .map(|substr| {
                substr
                    .parse::<i32>()
                    .map_err(|_| anyhow::anyhow!("failed to parse {} as i32", substr))
            })
            .collect::<Result<Vec<i32>>>()?;
        all_sequences.push(sequence);
    }
    Ok(all_sequences)
}

// - Expects numbers to be of length 2 op more.
// - Expects sequence to be converge towards 0 after consecutive diffs, and not run out of numbers.
fn next_number(numbers: &[i32]) -> i32 {
    assert!(numbers.len() > 1);
    if numbers.iter().all(|&i| i == 0) {
        return 0;
    }
    let diffs: Vec<i32> = numbers.windows(2).map(|w| w[1] - w[0]).collect();
    numbers.last().unwrap() + next_number(&diffs)
}

// - Same expectations on input as next_number
fn previous_number(numbers: &[i32]) -> i32 {
    assert!(numbers.len() > 1);
    if numbers.iter().all(|&i| i == 0) {
        return 0;
    }
    let diffs: Vec<i32> = numbers.windows(2).map(|w| w[1] - w[0]).collect();
    numbers.first().unwrap() - previous_number(&diffs)
}

fn part_one(input: &str) -> Result<i32> {
    let sequences = parse(input)?;
    Ok(sequences.into_iter().map(|seq| next_number(&seq)).sum())
}

fn part_two(input: &str) -> Result<i32> {
    let sequences = parse(input)?;
    Ok(sequences.into_iter().map(|seq| previous_number(&seq)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_next_number() {
        assert_eq!(next_number(&[0, 0, 0, 0]), 0);
        assert_eq!(next_number(&[3, 3, 3, 3, 3]), 3);
        assert_eq!(next_number(&[0, 3, 6, 9, 12, 15]), 18);
    }

    #[test]
    fn test_previous_number() {
        assert_eq!(previous_number(&[0, 0]), 0);
        assert_eq!(previous_number(&[2, 2, 2]), 2);
        assert_eq!(previous_number(&[0, 2, 4, 6]), -2);
        assert_eq!(previous_number(&[3, 3, 5, 9, 15]), 5);
        assert_eq!(previous_number(&[10, 13, 16, 21, 30, 45]), 5);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 114);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 2);
    }
}
