use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");

    let answer = part_one(&input);
    println!("part 1: {}", answer);

    let answer = part_two(&input);
    println!("part 2: {}", answer);
}

fn process_group_union(input: &str) -> usize {
    input
        .chars()
        .filter(|ch| ch.is_ascii_lowercase())
        .unique()
        .count()
}

fn process_group_intersection(input: &str) -> usize {
    let answers: Vec<HashSet<char>> = input
        .lines()
        .map(|line| {
            line.chars().fold(HashSet::new(), |mut set, ch| {
                set.insert(ch);
                set
            })
        })
        .collect();
    let mut iter = answers.into_iter();
    let mut intersection = iter.next().unwrap();
    for set in iter {
        intersection = intersection.intersection(&set).copied().collect();
    }
    intersection.iter().count()
}

fn part_one(input: &str) -> usize {
    input.split("\n\n").map(|g| process_group_union(g)).sum()
}

fn part_two(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|g| process_group_intersection(g))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_process_group() {
        assert_eq!(process_group_union("abcx\nabcy\nabcz"), 6);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 11);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), 6);
    }
}
