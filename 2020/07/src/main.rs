use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    let answer = part_one(input);
    println!("part 1: {}", answer);

    let answer = part_two(input);
    println!("part 2: {}", answer);
}

type ID = String;

#[derive(Debug)]
struct Bag {
    children: HashMap<ID, usize>,
    flag: bool,
}

fn parse_input(input: &str) -> HashMap<ID, Bag> {
    lazy_static! {
        static ref RE1: Regex = Regex::new(r"^(\S+ \S+) bags contain (.*)$").unwrap();
        static ref RE2: Regex = Regex::new(r"(\d+) (\S+ \S+) bags?").unwrap();
    }

    let mut bags = HashMap::new();
    for line in input.lines() {
        let caps = RE1.captures(line).unwrap();
        let id = caps.get(1).unwrap().as_str().to_string();
        let mut bag = Bag {
            children: HashMap::new(),
            flag: false,
        };
        for child in RE2.captures_iter(caps.get(2).unwrap().as_str()) {
            let child_count = child.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let child_id = child.get(2).unwrap().as_str().to_string();
            bag.children.insert(child_id, child_count);
        }
        bags.insert(id, bag);
    }

    bags
}

fn part_one(input: &str) -> usize {
    let mut bags = parse_input(input);
    let mut worklist = vec!["shiny gold".to_string()];
    while !worklist.is_empty() {
        let child_id = worklist.pop().unwrap();
        for (id, bag) in bags.iter_mut() {
            if bag.children.contains_key(&child_id) {
                bag.flag = true;
                worklist.push(id.to_string());
            }
        }
    }
    bags.values().filter(|bag| bag.flag).count()
}

fn part_two(input: &str) -> usize {
    let bags = parse_input(input);
    recurse(&bags, "shiny gold") - 1
}

fn recurse(bags: &HashMap<ID, Bag>, id: &str) -> usize {
    let bag = bags.get(id).unwrap();
    let mut count = 0;
    for (child_id, child_count) in bag.children.iter() {
        count += child_count * recurse(bags, child_id);
    }
    count + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_A: &str = include_str!("test-input-a.txt");
    const INPUT_B: &str = include_str!("test-input-b.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT_A), 4);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT_A), 32);
        assert_eq!(part_two(INPUT_B), 126);
    }
}
