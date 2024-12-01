use anyhow::Result;
use regex::Regex;

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 1530215)?;
    aoc::run!(part_two(input), 26800609)?;
    Ok(())
}

fn parse(input: &str) -> Result<(Vec<usize>, Vec<usize>)> {
    let re = Regex::new(r"(\d+)\s+(\d+)").unwrap();
    let mut out = (vec![], vec![]);
    for line in input.lines() {
        let (a, b) = aoc::parse!(&re, line, |s| s.parse::<usize>(), |s| s.parse::<usize>())?;
        out.0.push(a);
        out.1.push(b);
    }
    Ok(out)
}

fn part_one(input: &str) -> Result<usize> {
    let (mut a, mut b) = parse(input)?;
    a.sort();
    b.sort();
    Ok(a.into_iter().zip(b).map(|(i, j)| i.abs_diff(j)).sum())
}

fn part_two(input: &str) -> Result<usize> {
    let (a, b) = parse(input)?;
    let mut score = 0;
    for i in a.into_iter() {
        let j = b.iter().filter(|j| i == **j).count();
        score += i * j;
    }
    Ok(score)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 11);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 31);
    }
}
