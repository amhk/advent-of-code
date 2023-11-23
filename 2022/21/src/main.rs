use std::collections::HashMap;

use anyhow::{bail, ensure, Context, Result};

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 78342931359552)?;
    aoc::run!(part_two(input), 3296135418820)?;
    Ok(())
}

#[derive(PartialEq, Eq, Hash)]
struct Id([u8; 4]);

impl std::fmt::Debug for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = std::str::from_utf8(&self.0).unwrap();
        write!(f, "Id(\"{}\")", s)
    }
}

#[derive(Debug)]
enum Expr {
    Add(Id, Id),
    Sub(Id, Id),
    Mul(Id, Id),
    Div(Id, Id),
    Constant(i128),
}

type ExprTree = HashMap<Id, Expr>;

fn eval(tree: &ExprTree, root: &Id) -> Result<i128> {
    let expr = tree.get(root).context("unknown id")?;
    let value = match expr {
        Expr::Add(a, b) => eval(tree, a)? + eval(tree, b)?,
        Expr::Sub(a, b) => eval(tree, a)? - eval(tree, b)?,
        Expr::Mul(a, b) => eval(tree, a)? * eval(tree, b)?,
        Expr::Div(a, b) => eval(tree, a)? / eval(tree, b)?,
        Expr::Constant(c) => *c,
    };
    Ok(value)
}

fn contains(tree: &ExprTree, root: &Id, target: &Id) -> bool {
    if root == target {
        return true;
    }
    if let Some(expr) = tree.get(root) {
        match expr {
            Expr::Add(a, b) | Expr::Sub(a, b) | Expr::Mul(a, b) | Expr::Div(a, b) => {
                contains(tree, a, target) || contains(tree, b, target)
            }
            Expr::Constant(_) => false,
        }
    } else {
        false
    }
}

fn eval_unknown(tree: &ExprTree, root: &Id, unknown: &Id) -> Result<i128> {
    let expr = tree.get(root).context("unknown id")?;
    let (subtree_unknown, subtree_known) = match expr {
        Expr::Add(a, b) | Expr::Sub(a, b) | Expr::Mul(a, b) | Expr::Div(a, b) => {
            let ac = contains(tree, a, unknown);
            let bc = contains(tree, b, unknown);
            ensure!(ac != bc, "no solution possible");
            if ac {
                (a, b)
            } else {
                (b, a)
            }
        }
        Expr::Constant(_) => bail!("root is not a tree"),
    };
    let value = eval(tree, subtree_known)?;
    eval_reverse(tree, subtree_unknown, unknown, value)
}

fn eval_reverse(tree: &ExprTree, root: &Id, unknown: &Id, value: i128) -> Result<i128> {
    if root == unknown {
        return Ok(value);
    }
    let expr = tree.get(root).context("unknown id")?;
    let (a, b) = match expr {
        Expr::Add(a, b) | Expr::Sub(a, b) | Expr::Mul(a, b) | Expr::Div(a, b) => (a, b),
        Expr::Constant(_) => bail!("root is not a tree"),
    };

    // value = lhs + rhs  ->  lhs = value - rhs    rhs = value - lhs
    // value = lhs - rhs  ->  lhs = value + rhs    rhs = lhs - value
    // value = lhs * rhs  ->  lhs = value / rhs    rhs = value / lhs
    // value = lhs / rhs  ->  lhs = value * rhs    rhs = value * lhs

    if contains(tree, a, unknown) {
        // lhs unknown, rhs known
        ensure!(!contains(tree, b, unknown), "no solution possible");
        let value = match expr {
            Expr::Add(_, b) => value - eval(tree, b)?,
            Expr::Sub(_, b) => value + eval(tree, b)?,
            Expr::Mul(_, b) => value / eval(tree, b)?,
            Expr::Div(_, b) => value * eval(tree, b)?,
            Expr::Constant(_) => panic!("should not happen"),
        };
        eval_reverse(tree, a, unknown, value)
    } else {
        // lhs known, rhs unknown
        ensure!(!contains(tree, a, unknown), "no solution possible");
        let value = match expr {
            Expr::Add(a, _) => value - eval(tree, a)?,
            Expr::Sub(a, _) => eval(tree, a)? - value,
            Expr::Mul(a, _) => value / eval(tree, a)?,
            Expr::Div(a, _) => value * eval(tree, a)?,
            Expr::Constant(_) => panic!("should not happen"),
        };
        eval_reverse(tree, b, unknown, value)
    }
}

fn parse(input: &str) -> Result<ExprTree> {
    let mut expressions = HashMap::new();
    for line in input.lines() {
        let id = Id(line.as_bytes()[0..4]
            .try_into()
            .context("missing identifier")?);
        let line = &line[6..];
        if let Ok(c) = line.parse::<i128>() {
            expressions.insert(id, Expr::Constant(c));
        } else {
            let a = Id(line.as_bytes()[0..4]
                .try_into()
                .context("missing left operand")?);
            let b = Id(line.as_bytes()[7..11]
                .try_into()
                .context("missing right operand")?);
            let expr = match line.chars().nth(5) {
                Some('+') => Expr::Add(a, b),
                Some('-') => Expr::Sub(a, b),
                Some('*') => Expr::Mul(a, b),
                Some('/') => Expr::Div(a, b),
                _ => bail!("missing operator: {}", line),
            };
            expressions.insert(id, expr);
        }
    }
    Ok(expressions)
}

fn part_one(input: &str) -> Result<i128> {
    let tree = parse(input)?;
    let root = Id("root".as_bytes()[0..4].try_into().unwrap());
    eval(&tree, &root)
}

fn part_two(input: &str) -> Result<i128> {
    let tree = parse(input)?;
    let root = Id("root".as_bytes()[0..4].try_into().unwrap());
    let humn = Id("humn".as_bytes()[0..4].try_into().unwrap());
    eval_unknown(&tree, &root, &humn)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 152);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 301);
    }
}
