use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 5837374519342)?;
    aoc::run!(part_two(input), 492383931650959)?;
    Ok(())
}

#[derive(Debug, PartialEq)]
struct Input {
    lhs: u64,
    rhs: Vec<u64>,
}

fn parse(input: &str) -> Result<Vec<Input>> {
    let mut out = vec![];
    for line in input.lines() {
        let (first, second) = line
            .split_once(':')
            .ok_or_else(|| anyhow!("missing delimiter"))?;
        let rhs = second
            .split_whitespace()
            .map(|s| {
                s.parse::<u64>()
                    .map_err(|_| anyhow!("failed to parse '{}' as u64", s))
            })
            .collect::<Result<_>>()?;
        let input = Input {
            lhs: first.parse::<u64>()?,
            rhs,
        };
        out.push(input);
    }
    Ok(out)
}

fn concatenate(a: u64, b: u64) -> u64 {
    debug_assert!(b > 0);
    let mut i = 1;
    let mut j = b;
    while j > 0 {
        i *= 10;
        j /= 10;
    }
    a * i + b
}

fn evaluate(concat: bool, lhs: u64, rhs: &[u64]) -> bool {
    let len = rhs.len();
    match len {
        0 => panic!(),
        1 => lhs == rhs[0],
        _ => {
            let a = rhs[0];
            let b = rhs[1];

            // worth considering this subtree?
            if a > lhs {
                return false;
            }

            // add
            let modded_rhs = [a + b]
                .into_iter()
                .chain(rhs[2..].iter().copied())
                .collect::<Vec<_>>();
            if evaluate(concat, lhs, &modded_rhs) {
                return true;
            }

            // mul
            let modded_rhs = [a * b]
                .into_iter()
                .chain(rhs[2..].iter().copied())
                .collect::<Vec<_>>();
            if evaluate(concat, lhs, &modded_rhs) {
                return true;
            }

            // concat
            if concat {
                let modded_rhs = [concatenate(a, b)]
                    .into_iter()
                    .chain(rhs[2..].iter().copied())
                    .collect::<Vec<_>>();
                if evaluate(concat, lhs, &modded_rhs) {
                    return true;
                }
            }

            false
        }
    }
}

fn part_x(input: &str, concat: bool) -> Result<u64> {
    Ok(parse(input)?
        .into_iter()
        .filter(|input| evaluate(concat, input.lhs, &input.rhs))
        .map(|input| input.lhs)
        .sum())
}

fn part_one(input: &str) -> Result<u64> {
    part_x(input, false)
}

fn part_two(input: &str) -> Result<u64> {
    part_x(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_parse() {
        let input = parse(INPUT).unwrap();
        assert_eq!(input.len(), 9);
        assert_eq!(
            input[0],
            Input {
                lhs: 190,
                rhs: vec![10, 19],
            }
        );
    }

    #[test]
    fn test_concatenate() {
        assert_eq!(concatenate(1, 2), 12);
        assert_eq!(concatenate(12, 34), 1234);
    }

    #[test]
    fn test_evaluate_two_operators() {
        assert_eq!(evaluate(false, 190, &[10, 19]), true);
        assert_eq!(evaluate(false, 3267, &[81, 40, 27]), true);
        assert_eq!(evaluate(false, 83, &[17, 5]), false);
        assert_eq!(evaluate(false, 156, &[15, 6]), false);
        assert_eq!(evaluate(false, 7290, &[6, 8, 6, 15]), false);
        assert_eq!(evaluate(false, 161011, &[16, 10, 13]), false);
        assert_eq!(evaluate(false, 192, &[17, 8, 14]), false);
        assert_eq!(evaluate(false, 21037, &[9, 7, 18, 13]), false);
        assert_eq!(evaluate(false, 292, &[11, 6, 16, 20]), true);
    }

    #[test]
    fn test_evaluate_three_operators() {
        assert_eq!(evaluate(true, 190, &[10, 19]), true);
        assert_eq!(evaluate(true, 3267, &[81, 40, 27]), true);
        assert_eq!(evaluate(true, 83, &[17, 5]), false);
        assert_eq!(evaluate(true, 156, &[15, 6]), true);
        assert_eq!(evaluate(true, 7290, &[6, 8, 6, 15]), true);
        assert_eq!(evaluate(true, 161011, &[16, 10, 13]), false);
        assert_eq!(evaluate(true, 192, &[17, 8, 14]), true);
        assert_eq!(evaluate(true, 21037, &[9, 7, 18, 13]), false);
        assert_eq!(evaluate(true, 292, &[11, 6, 16, 20]), true);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 3749);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 11387);
    }
}
