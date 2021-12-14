use std::collections::BTreeMap;

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
    NoExpansion(Element, Element),
    NoElement,
}

type Element = u8;

type Rules = BTreeMap<(Element, Element), Element>;

type Depth = usize;

fn parse_input(input: &str) -> Result<(Vec<Element>, Rules), Error> {
    let initial: Vec<_> = input
        .lines()
        .next()
        .ok_or(Error::BadInput)?
        .chars()
        .map(|ch| ch as Element)
        .collect();

    let mut rules = BTreeMap::new();
    for line in input.lines().skip(2) {
        let mut chars = line.chars();
        let a = chars.next().ok_or(Error::BadInput)? as u8;
        let b = chars.next().ok_or(Error::BadInput)? as u8;
        if [chars.next(), chars.next(), chars.next(), chars.next()]
            != [Some(' '), Some('-'), Some('>'), Some(' ')]
        {
            return Err(Error::BadInput);
        }
        let c = chars.next().ok_or(Error::BadInput)? as u8;
        if chars.next().is_some() {
            return Err(Error::BadInput);
        }
        rules.insert((a, b), c);
    }

    Ok((initial, rules))
}

fn add<K: Ord + Copy>(map: &mut BTreeMap<K, usize>, other: &BTreeMap<K, usize>) {
    for (key, value) in other.iter() {
        *map.entry(*key).or_default() += value;
    }
}

fn visit(
    rules: &Rules,
    cache: &mut BTreeMap<(Depth, Element, Element), BTreeMap<Element, usize>>,
    depth: Depth,
    a: Element,
    b: Element,
) -> Result<BTreeMap<Element, usize>, Error> {
    let mut frequency = BTreeMap::new();
    if depth == 0 {
        *frequency.entry(a).or_default() += 1;
    } else {
        let c = *rules.get(&(a, b)).ok_or(Error::NoExpansion(a, b))?;

        for key in [(depth - 1, a, c), (depth - 1, c, b)] {
            if let Some(subset) = cache.get(&key) {
                add(&mut frequency, subset);
            } else {
                let subset = visit(rules, cache, key.0, key.1, key.2)?;
                add(&mut frequency, &subset);
                cache.insert(key, subset);
            }
        }
    }
    Ok(frequency)
}

fn part_x(input: &str, depth: usize) -> Result<usize, Error> {
    let (elements, rules) = parse_input(input)?;

    let mut frequency = BTreeMap::new();
    for (&a, &b) in elements.iter().zip(elements.iter().skip(1)) {
        let subset = visit(&rules, &mut BTreeMap::new(), depth, a, b)?;
        add(&mut frequency, &subset);
    }
    let last = *elements.iter().last().ok_or(Error::NoElement)?;
    *frequency.entry(last).or_default() += 1;

    let max = frequency.values().max().ok_or(Error::NoElement)?;
    let min = frequency.values().min().ok_or(Error::NoElement)?;
    Ok(max - min)
}

fn part_one(input: &str) -> Result<usize, Error> {
    part_x(input, 10)
}

fn part_two(input: &str) -> Result<usize, Error> {
    part_x(input, 40)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), Ok(1588));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), Ok(2188189693529));
    }
}
