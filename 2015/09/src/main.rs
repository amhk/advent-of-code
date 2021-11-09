use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

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

fn parse_input(input: &str) -> Result<HashMap<&str, HashMap<&str, usize>>, Error> {
    let re = Regex::new(r"(\w+) to (\w+) = (\d+)").unwrap();
    let mut map = HashMap::new();
    for line in input.lines() {
        let caps = re.captures(line).ok_or(Error::BadInput)?;
        let city1 = caps.get(1).unwrap().as_str();
        let city2 = caps.get(2).unwrap().as_str();
        let cost = caps
            .get(3)
            .unwrap()
            .as_str()
            .parse::<usize>()
            .map_err(|_| Error::BadInput)?;
        map.entry(city1)
            .or_insert_with(HashMap::new)
            .insert(city2, cost);
        map.entry(city2)
            .or_insert_with(HashMap::new)
            .insert(city1, cost);
    }
    Ok(map)
}

// The input is just 8 cities (8! permutations, and not all valid), so resort to brute force
fn shortest_distance(map: &HashMap<&str, HashMap<&str, usize>>) -> usize {
    let mut shortest = usize::MAX;
    for cities in map.keys().permutations(map.keys().len()) {
        let mut iter = cities.iter();
        iter.next();
        let mut current = 0;
        for (src, dest) in cities.iter().zip(iter) {
            if let Some(leg) = map[*src].get(*dest) {
                current += leg;
            } else {
                current = usize::MAX;
                break;
            }
        }
        if current < shortest {
            shortest = current;
        }
    }
    shortest
}

// The input is just 8 cities (8! permutations, and not all valid), so resort to brute force
fn longest_distance(map: &HashMap<&str, HashMap<&str, usize>>) -> usize {
    let mut longest = 0;
    for cities in map.keys().permutations(map.keys().len()) {
        let mut iter = cities.iter();
        iter.next();
        let mut current = 0;
        for (src, dest) in cities.iter().zip(iter) {
            if let Some(leg) = map[*src].get(*dest) {
                current += leg;
            } else {
                current = 0;
                break;
            }
        }
        if current > longest {
            longest = current;
        }
    }
    longest
}

fn part_one(input: &str) -> Result<usize, Error> {
    let map = parse_input(input)?;
    Ok(shortest_distance(&map))
}

fn part_two(input: &str) -> Result<usize, Error> {
    let map = parse_input(input)?;
    Ok(longest_distance(&map))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_shortest_distance() {
        let map = parse_input(INPUT).unwrap();
        assert_eq!(shortest_distance(&map), 605);
    }

    #[test]
    fn test_longest_distance() {
        let map = parse_input(INPUT).unwrap();
        assert_eq!(longest_distance(&map), 982);
    }
}
