use anyhow::Result;
use regex::Regex;

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 173517243)?;
    aoc::run!(part_two(input), 100450138)?;
    Ok(())
}

fn parse(input: &str, support_toggle_instr: bool) -> Result<Vec<(usize, usize)>> {
    let re = if support_toggle_instr {
        Regex::new(r#"do\(\)|don't\(\)|mul\((\d+),(\d+)\)"#).unwrap()
    } else {
        Regex::new(r#"mul\((\d+),(\d+)\)"#).unwrap()
    };
    let mut enabled = true;
    let mut out = vec![];
    for caps in re.captures_iter(input) {
        if caps[0].starts_with("mul(") {
            if enabled {
                let a = caps[1].parse::<usize>()?;
                let b = caps[2].parse::<usize>()?;
                out.push((a, b));
            }
        } else if caps[0] == *"don't()" {
            enabled = false;
        } else if caps[0] == *"do()" {
            enabled = true;
        };
    }
    Ok(out)
}

fn part_one(input: &str) -> Result<usize> {
    let pairs = parse(input, false)?;
    Ok(pairs.into_iter().map(|(a, b)| a * b).sum())
}

fn part_two(input: &str) -> Result<usize> {
    let pairs = parse(input, true)?;
    Ok(pairs.into_iter().map(|(a, b)| a * b).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_PART_ONE: &str = include_str!("test-input-part-one.txt");
    const INPUT_PART_TWO: &str = include_str!("test-input-part-two.txt");

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(INPUT_PART_ONE, false).unwrap(),
            vec![(2, 4), (5, 5), (11, 8), (8, 5)]
        );
        assert_eq!(parse(INPUT_PART_TWO, true).unwrap(), vec![(2, 4), (8, 5)]);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT_PART_ONE).unwrap(), 161);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT_PART_TWO).unwrap(), 48);
    }
}
