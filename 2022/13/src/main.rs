use anyhow::{ensure, Context, Result};
use std::cmp::Ordering;

fn main() -> Result<()> {
    let input = include_str!("input.txt");

    let answer = part_one(input).context("no solution for part one")?;
    println!("Part 1: {}", answer);
    assert_eq!(answer, 6086);

    let answer = part_two(input).context("no solution for part two")?;
    println!("Part 2: {}", answer);
    assert_eq!(answer, 27930);

    Ok(())
}

struct InputIterator<'a> {
    string: &'a str,
    index: usize,
}

impl<'a> InputIterator<'a> {
    fn new(string: &'a str) -> Self {
        InputIterator { string, index: 0 }
    }
}

/// Split by any of the delimiters [],
/// Keep delimiter if any of []
/// Discard delimiter if ,
impl<'a> Iterator for InputIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.string.len() {
            return None;
        }
        let start = self.index;
        let mut end = self.index;
        for ch in self.string.chars().skip(start) {
            match ch {
                '[' => {
                    self.index += 1;
                    end += 1;
                    break;
                }
                ',' => {
                    self.index += 1;
                    break;
                }
                ']' => {
                    if start != end {
                        break;
                    }
                    self.index += 1;
                    end += 1;
                }
                _ => {
                    self.index += 1;
                    end += 1;
                }
            };
        }
        Some(&self.string[start..end])
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Element {
    Constant(i32),
    List(Vec<Element>),
}

impl TryFrom<&str> for Element {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut current: Vec<Element> = vec![];
        let mut stack: Vec<Element> = vec![];
        for substring in InputIterator::new(value) {
            match substring {
                "[" => {
                    stack.push(Element::List(current));
                    current = vec![];
                }
                "]" => {
                    let tmp = stack.pop().context("unbalanced [] braces: unexpected ]")?;
                    if let Element::List(mut v) = tmp {
                        v.push(Element::List(current));
                        current = v;
                    } else {
                        panic!("cannot happen");
                    }
                }
                _ => {
                    let value: i32 = substring
                        .parse()
                        .with_context(|| format!("failed to convert '{}' to i32", substring))?;
                    current.push(Element::Constant(value));
                }
            }
        }
        ensure!(stack.is_empty(), "unbalanced [] braces: missing ]");
        ensure!(current.len() == 1, "failed to parse '{}'", value);
        Ok(current.pop().unwrap())
    }
}

#[derive(Debug, PartialEq)]
enum ElementOrdering {
    Correct,
    Incorrect,
    Undecided,
}

impl From<ElementOrdering> for Ordering {
    fn from(eo: ElementOrdering) -> Self {
        match eo {
            ElementOrdering::Correct => Ordering::Less,
            ElementOrdering::Incorrect => Ordering::Greater,
            ElementOrdering::Undecided => Ordering::Equal,
        }
    }
}

impl From<Ordering> for ElementOrdering {
    fn from(o: Ordering) -> Self {
        match o {
            Ordering::Less => ElementOrdering::Correct,
            Ordering::Equal => ElementOrdering::Undecided,
            Ordering::Greater => ElementOrdering::Incorrect,
        }
    }
}

fn order_of(left: &Element, right: &Element) -> ElementOrdering {
    if let (&Element::Constant(l), &Element::Constant(r)) = (&left, &right) {
        l.cmp(r).into()
    } else if let (&Element::List(l), &Element::List(r)) = (&left, &right) {
        for i in 0..l.len() {
            if i >= r.len() {
                return ElementOrdering::Incorrect;
            }
            match order_of(&l[i], &r[i]) {
                ElementOrdering::Correct => {
                    return ElementOrdering::Correct;
                }
                ElementOrdering::Incorrect => {
                    return ElementOrdering::Incorrect;
                }
                ElementOrdering::Undecided => {
                    // keep going through the elements
                }
            }
        }
        if l.len() == r.len() {
            ElementOrdering::Undecided
        } else {
            ElementOrdering::Correct
        }
    } else if let (&Element::Constant(l), &Element::List(_)) = (&left, &right) {
        order_of(&Element::List(vec![Element::Constant(*l)]), right)
    } else if let (&Element::List(_), &Element::Constant(r)) = (&left, &right) {
        order_of(left, &Element::List(vec![Element::Constant(*r)]))
    } else {
        panic!("cannot happen");
    }
}

fn part_one(input: &str) -> Result<usize> {
    let mut sum = 0;
    for (i, chunk) in input.split("\n\n").enumerate() {
        let i = i + 1;
        let chunk = chunk.trim();
        let (left, right) = chunk.split_once('\n').context("bad input")?;
        let left: Element = left.try_into()?;
        let right: Element = right.try_into()?;
        if order_of(&left, &right) == ElementOrdering::Correct {
            sum += i;
        }
    }
    Ok(sum)
}

fn part_two(input: &str) -> Result<usize> {
    let mut packets: Vec<Element> = vec![];
    let extra1: Element = "[[2]]".try_into()?;
    let extra2: Element = "[[6]]".try_into()?;
    for line in input.split('\n').filter(|line| !line.is_empty()) {
        packets.push(line.try_into()?);
    }
    packets.push(extra1.clone());
    packets.push(extra2.clone());
    packets.sort_by(|a, b| order_of(a, b).into());
    let a = packets.iter().position(|e| e == &extra1).unwrap() + 1;
    let b = packets.iter().position(|e| e == &extra2).unwrap() + 1;
    Ok(a * b)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_input_iterator() {
        let iter = InputIterator::new("[1,2,[[30]],40,50]");
        let actual: Vec<&str> = iter.collect();
        let expected: Vec<&str> = vec!["[", "1", "2", "[", "[", "30", "]", "]", "40", "50", "]"];
        dbg!(&actual);
        dbg!(&expected);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_element() {
        let element: Element = "[]".try_into().unwrap();
        assert_eq!(element, Element::List(vec![]));

        let element: Element = "[1,2]".try_into().unwrap();
        assert_eq!(
            element,
            Element::List(vec![Element::Constant(1), Element::Constant(2)])
        );

        let element: Element = "[[1,2],[3],4,[5]]".try_into().unwrap();
        assert_eq!(
            element,
            Element::List(vec![
                Element::List(vec![Element::Constant(1), Element::Constant(2)]),
                Element::List(vec![Element::Constant(3)]),
                Element::Constant(4),
                Element::List(vec![Element::Constant(5)]),
            ])
        );
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 13);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 140);
    }
}
