use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 27502)?;
    aoc::run!(part_two(input), 31947)?;
    Ok(())
}

type Pattern = Vec<String>;

#[derive(Debug)]
struct Mirror {
    horizontal: Pattern,
    vertical: Pattern,
}

fn rotate_90_cw(strings: &[String]) -> Vec<String> {
    if strings.is_empty() {
        return vec![];
    }
    assert!(strings.iter().all(|s| s.len() == strings[0].len()));
    let mut transposed = vec![String::new(); strings[0].len()];
    for y in 0..strings.len() {
        #[allow(clippy::needless_range_loop)]
        for x in 0..strings[0].len() {
            transposed[x].push(strings[y].chars().nth(x).unwrap());
        }
    }
    transposed
        .into_iter()
        .map(|s| s.chars().rev().collect())
        .collect()
}

fn parse(input: &str) -> Result<Vec<Mirror>> {
    let mut v = vec![];
    for chunk in input.split("\n\n") {
        let horizontal = chunk.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        let vertical = rotate_90_cw(&horizontal);
        v.push(Mirror {
            horizontal,
            vertical,
        });
    }
    Ok(v)
}

fn find_reflection(pattern: &Pattern, expected_smudges: usize) -> Option<usize> {
    fn diff_strings(a: &str, b: &str) -> usize {
        assert_eq!(a.len(), b.len());
        a.chars()
            .zip(b.chars())
            .fold(0, |acc, (a, b)| acc + if a == b { 0 } else { 1 })
    }
    let len = pattern.len() - 1;
    for i in 0..len {
        let size = 1 + i.min(len - i - 1);
        let diff = (0..size).fold(0, |acc, offset| {
            acc + diff_strings(&pattern[i - offset], &pattern[i + 1 + offset])
        });
        if diff == expected_smudges {
            return Some(i + 1);
        }
    }
    None
}

fn solve(input: &str, expected_smudges: usize) -> Result<usize> {
    let patterns = parse(input)?;
    patterns.into_iter().try_fold(0, |acc, pattern| {
        if let Some(i) = find_reflection(&pattern.vertical, expected_smudges) {
            Ok(acc + i)
        } else if let Some(i) = find_reflection(&pattern.horizontal, expected_smudges) {
            Ok(acc + 100 * i)
        } else {
            Err(anyhow!("pattern without reflection"))
        }
    })
}

fn part_one(input: &str) -> Result<usize> {
    solve(input, 0)
}

fn part_two(input: &str) -> Result<usize> {
    solve(input, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_rotate_90_cw() {
        assert_eq!(
            rotate_90_cw(&["1234".to_string(), "abcd".to_string(), "ABCD".to_string()]),
            ["Aa1", "Bb2", "Cc3", "Dd4"]
        );
    }

    #[test]
    fn test_find_reflection() {
        assert_eq!(
            find_reflection(
                &vec![
                    "#...##..#".to_string(),
                    "#....#..#".to_string(),
                    "..##..###".to_string(),
                    "#####.##.".to_string(),
                    "#####.##.".to_string(),
                    "..##..###".to_string(),
                    "#....#..#".to_string(),
                ],
                0
            ),
            Some(4)
        );
        assert_eq!(
            find_reflection(
                &vec![
                    "#.##..#".to_string(),
                    "..##...".to_string(),
                    "##..###".to_string(),
                    "#....#.".to_string(),
                    ".#..#.#".to_string(),
                    ".#..#.#".to_string(),
                    "#....#.".to_string(),
                    "##..###".to_string(),
                    "..##...".to_string(),
                ],
                0
            ),
            Some(5)
        );
        assert_eq!(
            find_reflection(
                &vec![
                    "#.##..##.".to_string(),
                    "..#.##.#.".to_string(),
                    "##......#".to_string(),
                    "##......#".to_string(),
                    "..#.##.#.".to_string(),
                    "..##..##.".to_string(),
                    "#.#.##.#.".to_string(),
                ],
                1
            ),
            Some(3)
        );
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 405);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 400);
    }
}
