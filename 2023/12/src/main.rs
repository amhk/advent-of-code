use std::collections::HashMap;

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 8419)?;
    aoc::run!(part_two(input), 160500973317706)?;
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    On,
    Off,
    Unknown,
}

fn parse_states(input: &str) -> Result<Vec<State>> {
    input
        .chars()
        .map(|ch| match ch {
            '#' => Ok(State::On),
            '.' => Ok(State::Off),
            '?' => Ok(State::Unknown),
            _ => Err(anyhow!("unexpected char '{ch}'")),
        })
        .collect::<Result<Vec<_>>>()
}

fn parse(input: &str) -> Result<Vec<(Vec<State>, Vec<usize>)>> {
    let mut out = vec![];
    for line in input.lines() {
        let (template, expected) = line.split_once(' ').ok_or_else(|| anyhow!("no space"))?;
        let template: Vec<State> = parse_states(template)?;
        let expected: Vec<usize> = expected
            .split(',')
            .map(|substr| {
                substr
                    .parse::<usize>()
                    .map_err(|_| anyhow!("failed to parse {} as usize", substr))
            })
            .collect::<Result<Vec<_>>>()?;
        out.push((template, expected));
    }
    Ok(out)
}

fn to_hash_key(states: &[State], group_sizes: &[usize]) -> String {
    let states = states
        .iter()
        .map(|state| match state {
            State::On => '#',
            State::Off => '.',
            State::Unknown => '?',
        })
        .collect::<String>();
    let group_sizes = group_sizes
        .iter()
        .map(|size| size.to_string())
        .collect::<Vec<_>>()
        .join(",");
    format!("{} {}", states, group_sizes)
}

fn possible_arrangements(
    states: &[State],
    group_sizes: &[usize],
    cache: &mut HashMap<String, usize>,
) -> usize {
    if group_sizes.is_empty() {
        let x = if states.contains(&State::On) { 0 } else { 1 };
        return x;
    }
    let key = to_hash_key(states, group_sizes);
    if let Some(value) = cache.get(&key) {
        return *value;
    }

    let size = group_sizes[0];
    assert!(size > 0);
    if size > states.len() {
        return 0;
    }

    let mut sum = 0;
    for begin in 0..=states.len() - size {
        if states[0..begin].contains(&State::On) {
            break;
        }
        if states
            .iter()
            .skip(begin)
            .take(size)
            .any(|&s| s == State::Off)
        {
            continue;
        }
        if states.get(begin + size) == Some(&State::On) {
            continue;
        }
        let end = if begin + size == states.len() {
            begin + size
        } else {
            begin + size + 1
        };
        let sub_states = &states[end..];
        let sub_group_sizes = &group_sizes[1..];
        sum += possible_arrangements(sub_states, sub_group_sizes, cache);
    }
    cache.insert(key, sum);
    sum
}

fn part_one(input: &str) -> Result<usize> {
    let mut sum = 0;
    let mut cache = HashMap::new();
    for (template, expected) in parse(input)?.into_iter() {
        sum += possible_arrangements(&template, &expected, &mut cache);
    }
    Ok(sum)
}

fn part_two(input: &str) -> Result<usize> {
    let mut new_input: Vec<String> = vec![];
    for line in input.lines() {
        let (a, b) = line.split_once(' ').ok_or_else(|| anyhow!("no space"))?;
        let a = [a, a, a, a, a].join("?");
        let b = [b, b, b, b, b].join(",");
        new_input.push(format!("{} {}", a, b));
    }
    part_one(&new_input.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_possible_arrangements() {
        macro_rules! assert_possible_arrangements {
            ($input:expr, $expected:expr) => {
                let tmp = parse($input).unwrap();
                assert_eq!(tmp.len(), 1);
                let (states, groups) = &tmp[0];
                let mut cache = HashMap::new();
                assert_eq!(possible_arrangements(states, groups, &mut cache), $expected);
            };
        }

        // single group left
        assert_possible_arrangements!(". 1", 0);
        assert_possible_arrangements!("# 1", 1);
        assert_possible_arrangements!("? 1", 1);
        assert_possible_arrangements!(".?#. 2", 1);
        assert_possible_arrangements!(".?#. 3", 0);
        assert_possible_arrangements!("??? 1", 3);
        assert_possible_arrangements!(".#. 1", 1);
        assert_possible_arrangements!("?#. 1", 1);
        assert_possible_arrangements!("?#? 1", 1);
        assert_possible_arrangements!("?#?? 1", 1);
        assert_possible_arrangements!("?#?? 2", 2);
        assert_possible_arrangements!("..?#..##.??.. 2", 0);
        assert_possible_arrangements!(".#.? 1", 1);
        assert_possible_arrangements!("...#?# 3", 1);

        // multiple groups left
        assert_possible_arrangements!("#.# 1,1", 1);
        assert_possible_arrangements!("?.# 1,1", 1);
        assert_possible_arrangements!("?.? 1,1", 1);
        assert_possible_arrangements!("##.### 2,3", 1);
        assert_possible_arrangements!("??.### 2,3", 1);
        assert_possible_arrangements!("??.?#? 2,3", 1);
        assert_possible_arrangements!("????? 2,3", 0);
        assert_possible_arrangements!("?????? 2,3", 1);

        // test-input.txt
        assert_possible_arrangements!("???.### 1,1,3", 1);
        assert_possible_arrangements!(".??..??...?##. 1,1,3", 4);
        assert_possible_arrangements!("?#?#?#?#?#?#?#? 1,3,1,6", 1);
        assert_possible_arrangements!("????.#...#... 4,1,1", 1);
        assert_possible_arrangements!("????.######..#####. 1,6,5", 4);
        assert_possible_arrangements!("?###???????? 3,2,1", 10);

        // input.txt
        assert_possible_arrangements!("..???.??.? 1,1,1", 9);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 21);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 525152);
    }
}
