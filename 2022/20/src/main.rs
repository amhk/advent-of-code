use std::num::ParseIntError;

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let input = include_str!("input.txt");

    let answer = part_one(input).context("no solution for part one")?;
    println!("Part 1: {}", answer);
    assert_eq!(answer, 988);

    let answer = part_two(input).context("no solution for part two")?;
    println!("Part 2: {}", answer);
    assert_eq!(answer, 7768531372516);

    Ok(())
}

type Index = usize;

#[derive(Debug, PartialEq)]
struct Item {
    value: i128,
    next: Index,
    prev: Index,
}

#[derive(Debug)]
struct CircularArray {
    items: Vec<Item>,
}

impl CircularArray {
    #[cfg(debug_assertions)]
    fn check_consistency(&self) {
        if self.items.is_empty() {
            return;
        }

        use std::collections::HashSet;

        let len = self.items.len();

        let mut visited = HashSet::new();
        let mut item = &self.items[0];
        for _ in 0..len {
            visited.insert(item.next);
            item = &self.items[item.next];
        }
        debug_assert!(visited.len() == len, "broken next chain (len)");

        let mut visited = HashSet::new();
        let mut item = &self.items[0];
        for _ in 0..len {
            visited.insert(item.prev);
            item = &self.items[item.prev];
        }
        debug_assert!(visited.len() == len, "broken prev chain (len)");
    }

    fn get(&self, index: usize) -> &Item {
        self.items.get(index).unwrap()
    }

    fn get_mut(&mut self, index: usize) -> &mut Item {
        self.items.get_mut(index).unwrap()
    }

    fn shift(&mut self, index: usize, steps: i128) {
        let len = self.items.len();
        let steps = steps.rem_euclid(len as i128 - 1);
        debug_assert!(steps >= 0);
        if steps == 0 || len < 2 {
            return;
        }

        let mut new_p = index;
        for _ in 0..steps {
            new_p = self.get(new_p).next;
        }
        let new_n = self.get(new_p).next;

        // remove item
        let old_p = self.get(index).prev;
        let old_n = self.get(index).next;
        self.get_mut(old_p).next = old_n;
        self.get_mut(old_n).prev = old_p;

        // insert item
        self.get_mut(new_p).next = index;
        self.get_mut(index).prev = new_p;
        self.get_mut(index).next = new_n;
        self.get_mut(new_n).prev = index;

        #[cfg(debug_assertions)]
        self.check_consistency();
    }

    fn iter(&self) -> CircularArrayIterator {
        CircularArrayIterator {
            array: self,
            current: Some(0),
        }
    }

    fn values_starting_from_zero(&self) -> Vec<i128> {
        let iter = self.iter().cycle();
        let iter = iter.skip_while(|item| item.value != 0);
        let len = self.items.len();
        iter.take(len).map(|item| item.value).collect::<Vec<_>>()
    }
}

#[derive(Clone, Copy)]
struct CircularArrayIterator<'a> {
    array: &'a CircularArray,
    current: Option<usize>,
}

impl<'a> Iterator for CircularArrayIterator<'a> {
    type Item = &'a Item;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(index) = self.current {
            let item = self.array.get(index);
            self.current = if item.next != 0 {
                Some(item.next)
            } else {
                None
            };
            Some(item)
        } else {
            None
        }
    }
}

impl FromIterator<i128> for CircularArray {
    fn from_iter<I: IntoIterator<Item = i128>>(iter: I) -> Self {
        let mut items = Vec::from_iter(iter.into_iter().map(|i| Item {
            value: i,
            next: 0,
            prev: 0,
        }));

        let index_last = items.len() - 1;
        for (index, item) in items.iter_mut().enumerate() {
            item.next = if index != index_last { index + 1 } else { 0 };
            item.prev = if index != 0 { index - 1 } else { index_last };
        }
        Self { items }
    }
}

fn mix(input: &str, secret_key: i128, rounds: usize) -> Result<CircularArray> {
    let x: Result<Vec<_>, ParseIntError> = input.lines().map(|s| s.parse::<i128>()).collect();
    let mut array =
        CircularArray::from_iter(x.context("bad input")?.into_iter().map(|i| i * secret_key));

    for _ in 0..rounds {
        for i in 0..array.items.len() {
            let steps = array.get(i).value;
            array.shift(i, steps);
        }
    }

    Ok(array)
}

fn magic_value(values: &[i128]) -> i128 {
    let a = values[1000 % values.len()];
    let b = values[2000 % values.len()];
    let c = values[3000 % values.len()];
    a + b + c
}

fn part_one(input: &str) -> Result<i128> {
    let array = mix(input, 1, 1)?;
    Ok(magic_value(&array.values_starting_from_zero()))
}

fn part_two(input: &str) -> Result<i128> {
    let array = mix(input, 811589153, 10)?;
    Ok(magic_value(&array.values_starting_from_zero()))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_circular_array() {
        let mut array = CircularArray::from_iter([0, 1, 2, 3].into_iter());
        assert_eq!(
            array.iter().map(|item| item.value).collect::<Vec<_>>(),
            [0, 1, 2, 3]
        );

        array.shift(1, 1);
        assert_eq!(array.values_starting_from_zero(), [0, 2, 1, 3]);

        array.shift(1, -1);
        assert_eq!(array.values_starting_from_zero(), [0, 1, 2, 3]);

        array.shift(0, 1);
        assert_eq!(array.values_starting_from_zero(), [0, 2, 3, 1]);

        array.shift(0, -1);
        assert_eq!(array.values_starting_from_zero(), [0, 1, 2, 3]);

        array.shift(2, 1);
        assert_eq!(array.values_starting_from_zero(), [0, 1, 3, 2]);
    }

    #[test]
    fn test_shift_test_input() {
        let mut array = CircularArray::from_iter([1, 2, -3, 3, -2, 0, 4].into_iter());
        assert_eq!(array.values_starting_from_zero(), [0, 4, 1, 2, -3, 3, -2]);

        array.shift(0, 1);
        assert_eq!(array.values_starting_from_zero(), [0, 4, 2, 1, -3, 3, -2]);

        array.shift(1, 2);
        assert_eq!(array.values_starting_from_zero(), [0, 4, 1, -3, 2, 3, -2]);

        array.shift(2, -3);
        assert_eq!(array.values_starting_from_zero(), [0, 4, 1, 2, 3, -2, -3]);

        array.shift(3, 3);
        assert_eq!(array.values_starting_from_zero(), [0, 3, 4, 1, 2, -2, -3]);

        array.shift(4, -2);
        assert_eq!(array.values_starting_from_zero(), [0, 3, 4, -2, 1, 2, -3]);

        array.shift(5, 0);
        assert_eq!(array.values_starting_from_zero(), [0, 3, 4, -2, 1, 2, -3]);

        array.shift(6, 4);
        assert_eq!(array.values_starting_from_zero(), [0, 3, -2, 1, 2, -3, 4]);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 3);
    }

    #[test]
    fn test_part_two() {
        let array = mix(INPUT, 811589153, 1).unwrap();
        assert_eq!(
            array.values_starting_from_zero(),
            [
                0,
                -2434767459,
                3246356612,
                -1623178306,
                2434767459,
                1623178306,
                811589153
            ]
        );

        let array = mix(INPUT, 811589153, 2).unwrap();
        assert_eq!(
            array.values_starting_from_zero(),
            [
                0,
                2434767459,
                1623178306,
                3246356612,
                -2434767459,
                -1623178306,
                811589153
            ]
        );

        assert_eq!(part_two(INPUT).unwrap(), 1623178306);
    }
}
