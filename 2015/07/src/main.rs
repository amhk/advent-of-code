use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::{cell::RefCell, collections::HashMap};

fn main() {
    let input = include_str!("input.txt");

    let answer = part_one(&input).expect("no solution for part one");
    println!("part 1: {}", answer);

    let answer = part_two(&input).expect("no solution for part two");
    println!("part 2: {}", answer);
}

#[derive(Debug, PartialEq)]
enum Error {
    BadInputLine,
    BadSignal,
    BadShiftAmount,
    DisconnectedNode,
}

type Signal = u16;

type Id = String;

type ShiftAmount = u16;

#[derive(Debug, PartialEq)]
enum Node {
    Fixed(Signal),               // <int>
    Passthrough(Id),             // <id>
    FixedAnd(Signal, Id),        // <signal> AND <id>
    And(Id, Id),                 // <id> AND <id>
    Or(Id, Id),                  // <id> OR <id>
    Not(Id),                     // NOT <id>
    LeftShift(Id, ShiftAmount),  // <id> LSHIFT <int>
    RightShift(Id, ShiftAmount), // <id> RSHIFT <int>
}

struct Circuit {
    nodes: HashMap<Id, Node>,
    values: RefCell<HashMap<Id, Signal>>,
}

impl Circuit {
    // id: use &str instead of &Id for better ergonomics:
    // `&"foo".to_string()` looks too weird
    fn value(&self, id: &str) -> Option<Signal> {
        if !self.nodes.contains_key(id) {
            return None;
        }
        if let Some(signal) = self.values.borrow().get(id) {
            return Some(*signal);
        }
        let signal = match &self.nodes[id] {
            Node::Fixed(signal) => *signal,
            Node::Passthrough(id) => self.value(&id)?,
            Node::FixedAnd(signal, id) => signal & self.value(&id)?,
            Node::And(id_lhs, id_rhs) => self.value(&id_lhs)? & self.value(&id_rhs)?,
            Node::Or(id_lhs, id_rhs) => self.value(&id_lhs)? | self.value(&id_rhs)?,
            Node::Not(id) => !self.value(id)?,
            Node::LeftShift(id, amount) => self.value(id)? << amount,
            Node::RightShift(id, amount) => self.value(id)? >> amount,
        };
        self.values.borrow_mut().insert(id.to_string(), signal);
        Some(signal)
    }
}

fn parse_input(input: &str) -> Result<Circuit, Error> {
    fn cap_id(caps: &Captures, i: usize) -> Result<Id, Error> {
        Ok(caps.get(i).unwrap().as_str().to_string())
    }
    fn cap_signal(caps: &Captures, i: usize) -> Result<Signal, Error> {
        caps.get(i)
            .unwrap()
            .as_str()
            .parse::<u16>()
            .map_err(|_| Error::BadSignal)
    }
    fn cap_amount(caps: &Captures, i: usize) -> Result<ShiftAmount, Error> {
        cap_signal(caps, i).map_err(|_| Error::BadShiftAmount)
    }
    lazy_static! {
        static ref RE_FIXED: Regex = Regex::new(r"^(\d+) -> (\w+)$").unwrap();
        static ref RE_PASSTHROUGH: Regex = Regex::new(r"^(\w+) -> (\w+)$").unwrap();
        static ref RE_FIXED_AND: Regex = Regex::new(r"^(\d+) AND (\w+) -> (\w+)$").unwrap();
        static ref RE_AND: Regex = Regex::new(r"^(\w+) AND (\w+) -> (\w+)$").unwrap();
        static ref RE_OR: Regex = Regex::new(r"^(\w+) OR (\w+) -> (\w+)$").unwrap();
        static ref RE_NOT: Regex = Regex::new(r"^NOT (\w+) -> (\w+)$").unwrap();
        static ref RE_LEFT_SHIFT: Regex = Regex::new(r"^(\w+) LSHIFT (\d+) -> (\w+)$").unwrap();
        static ref RE_RIGHT_SHIFT: Regex = Regex::new(r"^(\w+) RSHIFT (\d+) -> (\w+)$").unwrap();
    }
    let mut nodes = HashMap::new();
    for line in input.lines() {
        if let Some(caps) = RE_FIXED.captures(line) {
            nodes.insert(cap_id(&caps, 2)?, Node::Fixed(cap_signal(&caps, 1)?));
            continue;
        }
        if let Some(caps) = RE_PASSTHROUGH.captures(line) {
            nodes.insert(cap_id(&caps, 2)?, Node::Passthrough(cap_id(&caps, 1)?));
            continue;
        }
        if let Some(caps) = RE_FIXED_AND.captures(line) {
            nodes.insert(
                cap_id(&caps, 3)?,
                Node::FixedAnd(cap_signal(&caps, 1)?, cap_id(&caps, 2)?),
            );
            continue;
        }
        if let Some(caps) = RE_AND.captures(line) {
            nodes.insert(
                cap_id(&caps, 3)?,
                Node::And(cap_id(&caps, 1)?, cap_id(&caps, 2)?),
            );
            continue;
        }
        if let Some(caps) = RE_OR.captures(line) {
            nodes.insert(
                cap_id(&caps, 3)?,
                Node::Or(cap_id(&caps, 1)?, cap_id(&caps, 2)?),
            );
            continue;
        }
        if let Some(caps) = RE_NOT.captures(line) {
            nodes.insert(cap_id(&caps, 2)?, Node::Not(cap_id(&caps, 1)?));
            continue;
        }
        if let Some(caps) = RE_LEFT_SHIFT.captures(line) {
            nodes.insert(
                cap_id(&caps, 3)?,
                Node::LeftShift(cap_id(&caps, 1)?, cap_amount(&caps, 2)?),
            );
            continue;
        }
        if let Some(caps) = RE_RIGHT_SHIFT.captures(line) {
            nodes.insert(
                cap_id(&caps, 3)?,
                Node::RightShift(cap_id(&caps, 1)?, cap_amount(&caps, 2)?),
            );
            continue;
        }
        return Err(Error::BadInputLine);
    }
    Ok(Circuit {
        nodes,
        values: RefCell::new(HashMap::new()),
    })
}

fn part_one(input: &str) -> Result<Signal, Error> {
    let circuit = parse_input(input)?;
    circuit.value("a").ok_or(Error::DisconnectedNode)
}

fn part_two(input: &str) -> Result<Signal, Error> {
    let mut circuit = parse_input(input)?;
    let signal = circuit.value("a").ok_or(Error::DisconnectedNode)?;
    circuit.nodes.insert("b".to_string(), Node::Fixed(signal));
    circuit.values.borrow_mut().clear();
    circuit.value("a").ok_or(Error::DisconnectedNode)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_example_circuit() {
        let circuit = parse_input(INPUT).unwrap();
        assert_eq!(circuit.value("d"), Some(72));
        assert_eq!(circuit.value("e"), Some(507));
        assert_eq!(circuit.value("f"), Some(492));
        assert_eq!(circuit.value("g"), Some(114));
        assert_eq!(circuit.value("h"), Some(65412));
        assert_eq!(circuit.value("i"), Some(65079));
        assert_eq!(circuit.value("x"), Some(123));
        assert_eq!(circuit.value("y"), Some(456));
    }
}
