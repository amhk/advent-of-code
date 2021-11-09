use regex::{Captures, Regex};

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
    NoAuntFound,
    TooManyAuntsFound,
}

#[derive(Debug)]
struct AuntSue {
    id: usize,
    children: Option<usize>,
    cats: Option<usize>,
    samoyeds: Option<usize>,
    pomeranians: Option<usize>,
    akitas: Option<usize>,
    vizslas: Option<usize>,
    goldfish: Option<usize>,
    trees: Option<usize>,
    cars: Option<usize>,
    perfumes: Option<usize>,
}

fn parse_input(input: &str) -> Result<Vec<AuntSue>, Error> {
    fn parse_usize(caps: &Captures, index: usize) -> Result<usize, Error> {
        caps.get(index)
            .unwrap()
            .as_str()
            .parse::<usize>()
            .map_err(|_| Error::BadInput)
    }

    fn parse_field(re: &Regex, line: &str) -> Result<Option<usize>, Error> {
        if let Some(caps) = re.captures(line) {
            Ok(Some(parse_usize(&caps, 1)?))
        } else {
            Ok(None)
        }
    }

    let re = Regex::new(r"^Sue (\d+):").unwrap();
    let re_children = Regex::new(r"children: (\d+)").unwrap();
    let re_cats = Regex::new(r"cats: (\d+)").unwrap();
    let re_samoyeds = Regex::new(r"samoyeds: (\d+)").unwrap();
    let re_pomeranians = Regex::new(r"pomeranians: (\d+)").unwrap();
    let re_akitas = Regex::new(r"akitas: (\d+)").unwrap();
    let re_vizslas = Regex::new(r"vizslas: (\d+)").unwrap();
    let re_goldfish = Regex::new(r"goldfish: (\d+)").unwrap();
    let re_trees = Regex::new(r"trees: (\d+)").unwrap();
    let re_cars = Regex::new(r"cars: (\d+)").unwrap();
    let re_perfumes = Regex::new(r"perfumes: (\d+)").unwrap();

    let mut aunts = Vec::new();
    for line in input.lines() {
        let caps = re.captures(line).ok_or(Error::BadInput)?;
        aunts.push(AuntSue {
            id: parse_usize(&caps, 1)?,
            children: parse_field(&re_children, line)?,
            cats: parse_field(&re_cats, line)?,
            samoyeds: parse_field(&re_samoyeds, line)?,
            pomeranians: parse_field(&re_pomeranians, line)?,
            akitas: parse_field(&re_akitas, line)?,
            vizslas: parse_field(&re_vizslas, line)?,
            goldfish: parse_field(&re_goldfish, line)?,
            trees: parse_field(&re_trees, line)?,
            cars: parse_field(&re_cars, line)?,
            perfumes: parse_field(&re_perfumes, line)?,
        });
    }
    Ok(aunts)
}

fn part_one(input: &str) -> Result<usize, Error> {
    let aunts = parse_input(input)?;
    let remaining: Vec<_> = aunts
        .iter()
        .filter(|a| a.children.is_none() || a.children == Some(3))
        .filter(|a| a.cats.is_none() || a.cats == Some(7))
        .filter(|a| a.samoyeds.is_none() || a.samoyeds == Some(2))
        .filter(|a| a.pomeranians.is_none() || a.pomeranians == Some(3))
        .filter(|a| a.akitas.is_none() || a.akitas == Some(0))
        .filter(|a| a.vizslas.is_none() || a.vizslas == Some(0))
        .filter(|a| a.goldfish.is_none() || a.goldfish == Some(5))
        .filter(|a| a.trees.is_none() || a.trees == Some(3))
        .filter(|a| a.cars.is_none() || a.cars == Some(2))
        .filter(|a| a.perfumes.is_none() || a.perfumes == Some(1))
        .collect();
    match remaining.len() {
        0 => Err(Error::NoAuntFound),
        1 => Ok(remaining[0].id),
        _ => Err(Error::TooManyAuntsFound),
    }
}

fn part_two(input: &str) -> Result<usize, Error> {
    let aunts = parse_input(input)?;
    let remaining: Vec<_> = aunts
        .iter()
        .filter(|a| a.children.is_none() || a.children == Some(3))
        .filter(|a| a.cats.is_none() || a.cats > Some(7))
        .filter(|a| a.samoyeds.is_none() || a.samoyeds == Some(2))
        .filter(|a| a.pomeranians.is_none() || a.pomeranians < Some(3))
        .filter(|a| a.akitas.is_none() || a.akitas == Some(0))
        .filter(|a| a.vizslas.is_none() || a.vizslas == Some(0))
        .filter(|a| a.goldfish.is_none() || a.goldfish < Some(5))
        .filter(|a| a.trees.is_none() || a.trees > Some(3))
        .filter(|a| a.cars.is_none() || a.cars == Some(2))
        .filter(|a| a.perfumes.is_none() || a.perfumes == Some(1))
        .collect();
    match remaining.len() {
        0 => Err(Error::NoAuntFound),
        1 => Ok(remaining[0].id),
        _ => Err(Error::TooManyAuntsFound),
    }
}
