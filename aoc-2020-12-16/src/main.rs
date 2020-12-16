use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

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

#[derive(Debug)]
struct Field {
    name: String,
    ranges: ((u32, u32), (u32, u32)),
}

impl Field {
    fn ok(&self, value: u32) -> bool {
        (self.ranges.0 .0 <= value && value <= self.ranges.0 .1)
            || (self.ranges.1 .0 <= value && value <= self.ranges.1 .1)
    }
}

#[derive(Debug)]
struct Ticket {
    values: Vec<u32>,
}

lazy_static! {
    static ref RE_SECTIONS: Regex =
        Regex::new(r"(?ms)(.*?)\n\nyour ticket:\n(.*?)\n\nnearby tickets:\n(.*)").unwrap();
    static ref RE_FIELDS: Regex = Regex::new(r"(.*?): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
}

fn parse_input(input: &str) -> Result<(Vec<Field>, Ticket, Vec<Ticket>), Error> {
    fn parse_u32(caps: &Captures, index: usize) -> Result<u32, Error> {
        caps.get(index)
            .unwrap()
            .as_str()
            .parse::<u32>()
            .map_err(|_| Error::BadInput)
    }

    let sections = RE_SECTIONS.captures(&input).ok_or(Error::BadInput)?;

    let mut fields = Vec::new();
    for caps in RE_FIELDS.captures_iter(sections.get(1).unwrap().as_str()) {
        let name = caps.get(1).unwrap().as_str().to_string();
        let a = parse_u32(&caps, 2)?;
        let b = parse_u32(&caps, 3)?;
        let c = parse_u32(&caps, 4)?;
        let d = parse_u32(&caps, 5)?;
        assert!(a < b);
        assert!(b < c);
        assert!(c < d);
        fields.push(Field {
            name,
            ranges: ((a, b), (c, d)),
        });
    }

    let your_ticket = Ticket {
        values: sections
            .get(2)
            .unwrap()
            .as_str()
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect(),
    };
    assert_eq!(fields.len(), your_ticket.values.len());

    let mut tickets = Vec::new();
    for line in sections.get(3).unwrap().as_str().lines() {
        let t = Ticket {
            values: line.split(',').map(|s| s.parse::<u32>().unwrap()).collect(),
        };
        assert_eq!(fields.len(), t.values.len());
        tickets.push(t);
    }

    Ok((fields, your_ticket, tickets))
}

fn part_one(input: &str) -> Result<u32, Error> {
    let (fields, _, tickets) = parse_input(input)?;

    let mut sum = 0;
    for t in tickets.iter() {
        for v in t.values.iter() {
            let x = fields.iter().find(|f| f.ok(*v));
            if x.is_none() {
                sum += v;
                break; // input guarantee: at most on value is invalid
            }
        }
    }

    Ok(sum)
}

type Column = usize;

fn assign_fields(
    fields: Vec<Field>,
    tickets: Vec<Ticket>,
) -> Result<HashMap<String, Column>, Error> {
    let tickets: Vec<&Ticket> = tickets
        .iter()
        .filter(|t| {
            for v in t.values.iter() {
                if fields.iter().find(|f| f.ok(*v)).is_none() {
                    return false;
                }
            }
            true
        })
        .collect();

    let mut x: HashMap<String, HashSet<Column>> = HashMap::new();
    for key in fields.iter().map(|f| f.name.clone()) {
        x.insert(key, HashSet::from_iter(0..fields.len()));
    }

    for t in tickets.iter() {
        for (i, v) in t.values.iter().enumerate() {
            for f in fields.iter() {
                if !f.ok(*v) {
                    x.get_mut(&f.name).unwrap().remove(&i);
                }
            }
        }
    }
    x.iter().for_each(|pair| assert!(!pair.1.is_empty()));

    let mut mapping: HashMap<String, Column> = HashMap::new();
    while !x.is_empty() {
        let field = x
            .iter()
            .find(|pair| pair.1.len() == 1)
            .map(|pair| pair.0.clone())
            .ok_or(Error::BadInput)?;
        let column = *x.get(&field).unwrap().iter().next().unwrap();
        x.remove(&field);
        mapping.insert(field, column);
        for value in x.values_mut() {
            value.remove(&column);
        }
    }

    Ok(mapping)
}

fn part_two(input: &str) -> Result<u64, Error> {
    let (fields, your_ticket, tickets) = parse_input(input)?;
    let mapping = assign_fields(fields, tickets)?;
    let product: u64 = mapping
        .iter()
        .filter(|x| x.0.starts_with("departure"))
        .map(|x| *your_ticket.values.get(*x.1).unwrap() as u64)
        .product();
    Ok(product)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_PART_ONE: &str = include_str!("test-input-part-one.txt");
    const INPUT_PART_TWO: &str = include_str!("test-input-part-two.txt");

    #[test]
    fn test_parse_input() {
        assert!(parse_input(INPUT_PART_ONE).is_ok());
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT_PART_ONE), Ok(4 + 55 + 12));
    }

    #[test]
    fn test_assign_fields() {
        let (fields, _, tickets) = parse_input(INPUT_PART_TWO).unwrap();
        let expected = vec![
            ("class".to_string(), 1),
            ("row".to_string(), 0),
            ("seat".to_string(), 2),
        ];
        assert_eq!(
            assign_fields(fields, tickets),
            Ok(HashMap::from_iter(expected))
        );
    }
}
