use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    let answer = part_one(input).expect("no solution for part one");
    println!("part 1: {}", answer);

    let answer = part_two(input).expect("no solution for part two");
    println!("part 2: {}", answer);
}

fn parse_input(input: &str) -> Result<HashMap<&str, HashMap<&str, i32>>, Error> {
    let mut guests = HashMap::new();
    for line in input.lines() {
        let mut words = line.split_whitespace();
        let who = words.next().ok_or(Error::BadInput)?;
        let sign = match words.nth(1) {
            Some("gain") => 1,
            Some("lose") => -1,
            _ => return Err(Error::BadInput),
        };
        let amount = words
            .next()
            .map(|s| s.parse::<i32>())
            .ok_or(Error::BadInput)?
            .map_err(|_| Error::BadInput)?;
        let neighbour = words
            .nth(6)
            .map(|s| s.strip_suffix('.'))
            .flatten()
            .ok_or(Error::BadInput)?;
        guests
            .entry(who)
            .or_insert_with(HashMap::new)
            .insert(neighbour, sign * amount);
    }
    Ok(guests)
}

fn find_best_arrangement(guests: &HashMap<&str, HashMap<&str, i32>>) -> i32 {
    let mut best = i32::MIN;
    for mut order in guests.keys().permutations(guests.keys().len()) {
        let mut score = 0;
        order.push(order[0]);
        for (a, b) in order.iter().zip(order.iter().skip(1)) {
            score += guests[**a][**b];
            score += guests[**b][**a];
        }
        if score > best {
            best = score;
        }
    }
    best
}

#[derive(Debug, PartialEq)]
enum Error {
    BadInput,
}

fn part_one(input: &str) -> Result<i32, Error> {
    let guests = parse_input(input)?;
    Ok(find_best_arrangement(&guests))
}

fn part_two(input: &str) -> Result<i32, Error> {
    let mut guests = parse_input(input)?;
    let names: Vec<_> = guests.keys().cloned().collect();
    guests.insert("myself", HashMap::new());
    for name in names {
        guests.get_mut("myself").unwrap().insert(name, 0);
        guests.get_mut(name).unwrap().insert("myself", 0);
    }
    Ok(find_best_arrangement(&guests))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_find_best_arrangement() {
        let guests = parse_input(INPUT).unwrap();
        assert_eq!(find_best_arrangement(&guests), 330);
    }
}
