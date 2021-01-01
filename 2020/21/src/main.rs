use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{BTreeSet, HashMap};

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

type Allergen = String;
type Ingredient = String;

lazy_static! {
    static ref RE_PARTS: Regex = Regex::new(r"(.*?) \(contains (.*)\)").unwrap();
    static ref RE_ALLERGEN: Regex = Regex::new(r"([^,\s]+),?\s?").unwrap();
}

fn parse_input(input: &str) -> Result<HashMap<Allergen, Vec<BTreeSet<Ingredient>>>, Error> {
    let mut map = HashMap::new();

    for line in input.lines() {
        let mut set = BTreeSet::new();

        let caps = RE_PARTS.captures(line).ok_or(Error::BadInput)?;
        for ingredient in caps.get(1).unwrap().as_str().split_whitespace() {
            set.insert(ingredient.to_string());
        }
        let allergen_list = caps.get(2).unwrap().as_str();
        for caps in RE_ALLERGEN.captures_iter(&allergen_list) {
            let allergen = caps.get(1).unwrap().as_str();
            map.entry(allergen.to_string())
                .or_insert_with(Vec::new)
                .push(set.clone());
        }
    }

    Ok(map)
}

fn parse_all_ingredients(input: &str) -> Result<Vec<Ingredient>, Error> {
    let mut all_ingredients = Vec::new();
    for line in input.lines() {
        let caps = RE_PARTS.captures(line).ok_or(Error::BadInput)?;
        for ingredient in caps.get(1).unwrap().as_str().split_whitespace() {
            all_ingredients.push(ingredient.to_string());
        }
    }
    Ok(all_ingredients)
}

fn reduce(map: &mut HashMap<Allergen, Vec<BTreeSet<Ingredient>>>) -> HashMap<Allergen, Ingredient> {
    let mut locked_in: HashMap<Allergen, Ingredient> = HashMap::new();

    // remove ingredients listed in some, but not all, sets
    for (_, sets) in map.iter_mut() {
        if sets.len() == 1 {
            continue;
        }
        let all_ingredients = sets.iter().flat_map(|i| i.clone()).collect::<BTreeSet<_>>();
        for ingredient in all_ingredients {
            let count = sets.iter().flatten().filter(|&i| i == &ingredient).count();
            if count != sets.len() {
                for s in sets.iter_mut() {
                    s.remove(&ingredient);
                }
            }
        }
    }

    // remove duplicate sets
    for (_, sets) in map.iter_mut() {
        sets.sort_unstable();
        sets.dedup();
    }

    // lock in and remove one to one mappings
    loop {
        let mut change = false;
        let one_to_one = map
            .iter()
            .map(|(k, v)| (k, v.iter().flatten().cloned().collect::<BTreeSet<_>>()))
            .filter(|(_, v)| v.len() == 1)
            .map(|(k, v)| (k, v.iter().next().unwrap().clone()))
            .collect::<Vec<_>>();
        for (allergen, ingredient) in one_to_one {
            change = true;
            locked_in.insert(allergen.clone(), ingredient);
        }
        for allergen in locked_in.keys() {
            change = map.remove(allergen).is_some() || change;
        }
        for ingredient in locked_in.values() {
            for sets in map.values_mut() {
                for set in sets.iter_mut() {
                    change = set.remove(ingredient) || change;
                }
            }
        }
        if !change {
            return locked_in;
        }
    }
}

fn part_one(input: &str) -> Result<usize, Error> {
    let mut map = parse_input(input)?;
    let all_ingredients = parse_all_ingredients(input)?;
    let locked_in = reduce(&mut map);

    let bad_ingredients = locked_in.values().collect::<BTreeSet<_>>();
    let good_ingredients = all_ingredients
        .iter()
        .filter(|&i| !bad_ingredients.contains(i));

    Ok(good_ingredients.count())
}

fn part_two(input: &str) -> Result<String, Error> {
    let mut map = parse_input(input)?;
    let locked_in = reduce(&mut map);
    let mut locked_in = locked_in.iter().collect::<Vec<_>>();
    locked_in.sort_by_key(|i| i.0);
    let x = locked_in
        .iter()
        .cloned()
        .map(|(_, ingredient)| ingredient.clone())
        .collect::<Vec<_>>();
    Ok(x.join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), Ok(5));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), Ok("mxmxvkd,sqjhc,fvjkl".to_string()));
    }
}
