use anyhow::{anyhow, Context, Result};
use once_cell::sync::Lazy;
use regex::Regex;

fn main() -> Result<()> {
    let input = include_str!("input.txt");

    let answer = part_one(input).context("no solution for part one")?;
    println!("Part 1: {}", answer);
    assert_eq!(answer, 101_436);

    let answer = part_two(input).context("no solution for part two")?;
    println!("Part 2: {}", answer);
    assert_eq!(answer, 19_754_471_646);

    Ok(())
}

#[derive(Debug, PartialEq)]
enum Op {
    Add,
    Multiply,
}

impl TryFrom<&str> for Op {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "+" => Ok(Op::Add),
            "*" => Ok(Op::Multiply),
            _ => Err(anyhow!("cannot convert to Op")),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Term {
    CurrentValue,
    Constant(u64),
}

impl TryFrom<&str> for Term {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "old" => Ok(Self::CurrentValue),
            _ => Ok(Self::Constant(value.parse()?)),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Monkey {
    id: usize,
    items: Vec<u64>,
    expression: (Op, Term),
    condition: (u64, usize, usize), // divisor, monkey-if-true, monkey-if-false
}

impl TryFrom<&str> for Monkey {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        static REGEX: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"(?m)Monkey (\d):\n  Starting items: ([\d, ]+)\n  Operation: new = old ([+*]) (old|\d+)\n  Test: divisible by (\d+)\n    If true: throw to monkey (\d+)\n    If false: throw to monkey (\d+)").unwrap()
        });
        let caps = REGEX
            .captures(value)
            .context("regex does not match input")?;
        let id = caps.get(1).unwrap().as_str().parse().unwrap();
        let items = caps
            .get(2)
            .unwrap()
            .as_str()
            .split(", ")
            .map(|s| s.parse().unwrap())
            .collect();
        let expression = (
            caps.get(3).unwrap().as_str().try_into()?,
            caps.get(4).unwrap().as_str().try_into()?,
        );
        let condition = (
            caps.get(5).unwrap().as_str().parse()?,
            caps.get(6).unwrap().as_str().parse()?,
            caps.get(7).unwrap().as_str().parse()?,
        );
        Ok(Monkey {
            id,
            items,
            expression,
            condition,
        })
    }
}

fn parse(input: &str) -> Result<Vec<Monkey>> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    for chunk in input.split("\n\n") {
        monkeys.push(chunk.try_into()?);
        debug_assert!(monkeys.last().unwrap().id == monkeys.len() - 1);
    }
    Ok(monkeys)
}

fn monkey_business(input: &str, reduce_stress: u64, rounds: usize) -> Result<usize> {
    let mut monkeys = parse(input)?;
    let mut activity = vec![0; monkeys.len()];

    // All monkey's "Test divisible by X" values are prime numbers; let divisor be the product of
    // these values. At the end of of each round, reduce the value of each item to <item> mod
    // <divisor>, to prevent overflow.
    let divisor: u64 = monkeys.iter().map(|m| m.condition.0).product();

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let items = monkeys[i].items.split_off(0);
            activity[i] += items.len();
            for item in items {
                let expr = &monkeys[i].expression;
                let term = match expr.1 {
                    Term::CurrentValue => item,
                    Term::Constant(i) => i,
                };
                let item = match expr.0 {
                    Op::Add => (item + term) / reduce_stress,
                    Op::Multiply => (item * term) / reduce_stress,
                };
                let next_monkey = if item % monkeys[i].condition.0 == 0 {
                    monkeys[i].condition.1
                } else {
                    monkeys[i].condition.2
                };
                monkeys[next_monkey].items.push(item);
            }
        }

        for monkey in monkeys.iter_mut() {
            for item in monkey.items.iter_mut() {
                *item %= divisor;
            }
        }
    }

    activity.sort();
    let a = activity.pop().context("no monkeys")?;
    let b = activity.pop().context("only one monkey")?;
    Ok(a * b)
}

fn part_one(input: &str) -> Result<usize> {
    monkey_business(input, 3, 20)
}

fn part_two(input: &str) -> Result<usize> {
    monkey_business(input, 1, 10_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_parse() {
        let monkeys = parse(INPUT).unwrap();
        assert_eq!(monkeys.len(), 4);
        assert_eq!(
            monkeys[0],
            Monkey {
                id: 0,
                items: vec![79, 98],
                expression: (Op::Multiply, Term::Constant(19)),
                condition: (23, 2, 3),
            }
        );
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 10_605);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 2_713_310_158);
    }
}
