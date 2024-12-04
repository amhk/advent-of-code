use std::collections::BTreeMap;

use anyhow::{anyhow, ensure, Result};

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 2447)?;
    aoc::run!(part_two(input), 1868)?;
    Ok(())
}

type CharMap = BTreeMap<(i32, i32), char>;

fn parse(input: &str) -> Result<(CharMap, i32, i32)> {
    let mut matrix: CharMap = BTreeMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            matrix.insert((x as i32, y as i32), ch);
        }
    }

    let max_x = input
        .chars()
        .position(|ch| ch == '\n')
        .ok_or_else(|| anyhow!("missing newline"))? as i32;
    let max_y = input.lines().count() as i32;

    ensure!(max_x <= max_y); // assumption made when finding diagonals

    Ok((matrix, max_x, max_y))
}

fn find_lines(matrix: &CharMap, max_x: i32, max_y: i32) -> Vec<String> {
    let mut out = vec![];

    macro_rules! collect {
        ($start:expr, $delta_x:expr, $delta_y:expr) => {{
            let mut chars: Vec<char> = vec![];
            let (mut x, mut y) = $start;
            loop {
                chars.push(matrix[&(x, y)]);
                x += $delta_x;
                y += $delta_y;
                if x >= max_x || x < 0 || y >= max_y || y < 0 {
                    break;
                }
            }
            chars.into_iter().collect()
        }};
    }

    // right -> left
    for y in 0..max_y {
        let s: String = collect!((0, y), 1, 0);
        out.push(s.chars().rev().collect());
        out.push(s);
    }

    // top -> bottom
    for x in 0..max_x {
        let s: String = collect!((x, 0), 0, 1);
        out.push(s.chars().rev().collect());
        out.push(s);
    }

    // diagonal top left -> bottom right
    for y in 0..max_y {
        let s: String = collect!((0, y), 1, 1);
        out.push(s.chars().rev().collect());
        out.push(s);
    }
    for x in 1..max_x {
        let s: String = collect!((x, 0), 1, 1);
        out.push(s.chars().rev().collect());
        out.push(s);
    }

    // diagonal top right -> bottom left
    for x in 0..max_x {
        let s: String = collect!((x, 0), -1, 1);
        out.push(s.chars().rev().collect());
        out.push(s);
    }
    for y in 1..max_y {
        let s: String = collect!((max_x - 1, y), -1, 1);
        out.push(s.chars().rev().collect());
        out.push(s);
    }

    out
}

fn count_substrings(haystack: &str, needle: &str) -> usize {
    let mut index = 0;
    let mut count = 0;
    while let Some(new_index) = haystack[index..].find(needle) {
        index = index + new_index + needle.len();
        count += 1;
    }
    count
}

fn part_one(input: &str) -> Result<usize> {
    let (matrix, max_x, max_y) = parse(input)?;
    let strings = find_lines(&matrix, max_x, max_y);
    Ok(strings
        .into_iter()
        .map(|s| count_substrings(&s, "XMAS"))
        .sum())
}

fn part_two(input: &str) -> Result<usize> {
    const PATTERNS: [[char; 4]; 4] = [
        ['M', 'M', 'S', 'S'],
        ['M', 'S', 'M', 'S'],
        ['S', 'S', 'M', 'M'],
        ['S', 'M', 'S', 'M'],
    ];

    let (matrix, max_x, max_y) = parse(input)?;
    let mut count = 0;
    for x in 1..max_x - 1 {
        for y in 1..max_y - 1 {
            if matrix[&(x, y)] != 'A' {
                continue;
            }
            let actual = [
                matrix[&(x - 1, y - 1)],
                matrix[&(x + 1, y - 1)],
                matrix[&(x - 1, y + 1)],
                matrix[&(x + 1, y + 1)],
            ];
            if PATTERNS.contains(&actual) {
                count += 1;
            }
        }
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_find_lines() {
        let input = "abc\ndef\nghi\njkl";
        let (matrix, max_x, max_y) = parse(input).unwrap();
        assert_eq!(max_x, 3);
        assert_eq!(max_y, 4);

        let mut actual = find_lines(&matrix, max_x, max_y);
        actual.sort();

        // abc
        // def
        // ghi
        // jkl
        #[rustfmt::skip]
        let mut expected = [
            // left -> right
            "abc",
            "def",
            "ghi",
            "jkl",

            // left -> right (reversed)
            "cba",
            "fed",
            "ihg",
            "lkj",

            // top -> bottom
            "adgj",
            "behk",
            "cfil",

            // top -> bottom (reversed)
            "jgda",
            "kheb",
            "lifc",

            // diagonal top left -> bottom right
            "j",
            "gk",
            "dhl",
            "aei",
            "bf",
            "c",

            // diagonal top left -> bottom right (reversed)
            "j",
            "kg",
            "lhd",
            "iea",
            "fb",
            "c",

            // diagonal top right -> bottom left
            "a",
            "bd",
            "ceg",
            "fhj",
            "ik",
            "l",

            // diagonal top right -> bottom left (reversed)
            "a",
            "db",
            "gec",
            "jhf",
            "ki",
            "l",
        ];
        expected.sort();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 18);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 9);
    }
}
