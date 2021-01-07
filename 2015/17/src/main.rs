fn main() {
    let input = include_str!("input.txt");

    let answer = part_one(&input).expect("no solution for part one");
    println!("part 1: {}", answer);

    let answer = part_two(&input).expect("no solution for part two");
    println!("part 2: {}", answer);
}

#[derive(Debug, PartialEq)]
enum Error {
    BadInput,
}

fn parse_input(input: &str) -> Result<Vec<u32>, Error> {
    let mut v = Vec::new();
    for line in input.lines() {
        v.push(line.parse::<u32>().map_err(|_| Error::BadInput)?);
    }
    Ok(v)
}

fn find_combinations(numbers: &[u32], target_sum: u32) -> Vec<Vec<u32>> {
    fn gen(fixed: &[u32], pending: &[u32], target_sum: u32, out: &mut Vec<Vec<u32>>) {
        debug_assert_ne!(pending.len(), 0);

        let fixed_sum = fixed.iter().sum::<u32>();
        if fixed_sum >= target_sum {
            return;
        }

        let i = pending[0];
        let pending = &pending[1..];
        if fixed_sum + i == target_sum {
            let mut v = fixed.to_vec();
            v.push(i);
            out.push(v);
        }

        if !pending.is_empty() {
            gen(fixed, pending, target_sum, out);

            let mut fixed = fixed.to_vec();
            fixed.push(i);
            gen(&fixed, pending, target_sum, out);
        }
    }

    debug_assert!(!numbers.iter().any(|&n| n == 0));
    let mut out = Vec::new();
    gen(&[], &numbers, target_sum, &mut out);
    out
}

fn part_one(input: &str) -> Result<usize, Error> {
    let numbers = parse_input(input)?;
    let combinations = find_combinations(&numbers, 150);
    Ok(combinations.len())
}

fn part_two(input: &str) -> Result<usize, Error> {
    let numbers = parse_input(input)?;
    let combinations = find_combinations(&numbers, 150);

    let mut sizes = combinations.iter().map(|v| v.len()).collect::<Vec<_>>();
    sizes.sort_unstable();
    let first = *sizes.get(0).ok_or(Error::BadInput)?;
    Ok(sizes.iter().take_while(|&&i| i == first).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_combinations() {
        let mut combinations = find_combinations(&[20, 15, 10, 5, 5], 25);
        combinations.sort_unstable();
        let mut expected = vec![vec![15, 10], vec![20, 5], vec![20, 5], vec![15, 5, 5]];
        expected.sort_unstable();
        assert_eq!(combinations, expected);
    }
}
