use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");

    let answer = part_one(&input).expect("no solution for part one");
    println!("part 1: {}", answer);

    let answer = part_two(&input).expect("no solution for part two");
    println!("part 2: {}", answer);
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum Error {
    BadInput,
    NoSolution,
    SumOfWeightsNotDivisibleByNumberOfBins,
}

fn parse_input(s: &str) -> Result<Vec<u32>, Error> {
    let mut v = Vec::new();
    for line in s.lines() {
        let value = line.parse::<u32>().map_err(|_| Error::BadInput)?;
        if v.contains(&value) {
            return Err(Error::BadInput); // code assumes no duplicates (when calling Vec::retain)
        }
        v.push(value);
    }
    Ok(v)
}

fn chomp(slice: &[u32]) -> Vec<(u32, Vec<u32>)> {
    debug_assert!(!slice.is_empty());
    let mut out = Vec::new();
    for index in 0..slice.len() {
        let mut v = Vec::new();
        v.extend_from_slice(&slice[0..index]);
        v.extend_from_slice(&slice[index + 1..slice.len()]);
        v.sort_by(|a, b| b.partial_cmp(a).unwrap()); // sort decending
        out.push((slice[index], v));
    }
    out
}

fn find_subsets(
    values: &[u32],
    target_size: usize,
    target_sum: u32,
) -> HashSet<(Vec<u32>, Vec<u32>)> {
    let mut out = HashSet::new();
    for perm in values.iter().cloned().combinations(target_size) {
        if perm.iter().sum::<u32>() == target_sum {
            let mut rest = values.to_vec();
            rest.retain(|value| !perm.contains(value));
            out.insert((perm, rest));
        }
    }
    out
}

fn find_smallest_subsets(values: &[u32], target_sum: u32) -> HashSet<(Vec<u32>, Vec<u32>)> {
    for size in 1..values.len() {
        let sets = find_subsets(values, size, target_sum);
        if !sets.is_empty() {
            return sets;
        }
    }
    unreachable!(); // assume input is nice
}

fn will_fit_in_bins(values: &[u32], target_sum: u32, bins_left: usize) -> bool {
    let values = {
        let mut values = values.to_vec();
        values.sort_by(|a, b| b.partial_cmp(a).unwrap());
        values
    };

    fn recurse(current: &[u32], values_left: &[u32], target_sum: u32, bins_left: usize) -> bool {
        debug_assert!(bins_left > 0);
        debug_assert!(current.iter().sum::<u32>() != target_sum);
        // values_left are sorted in decending order, but [T].is_sorted_by is nightly only
        // debug_assert!(values_left.is_sorted_by(...));

        if bins_left == 1 {
            return values_left.iter().sum::<u32>() == target_sum;
        }

        let sum = current.iter().sum::<u32>();
        for (value, values_left) in chomp(&values_left) {
            if sum + value < target_sum {
                let mut current = current.to_vec();
                current.push(value);
                if recurse(&current, &values_left, target_sum, bins_left) {
                    return true;
                }
            }
            if sum + value == target_sum {
                let mut current = current.to_vec();
                current.push(value);
                if recurse(&[], &values_left, target_sum, bins_left - 1) {
                    return true;
                }
            }
            if sum + value > target_sum {
                break;
            }
        }

        false
    }

    recurse(&[], &values, target_sum, bins_left)
}

fn find_target_sum(values: &[u32], bins: u32) -> Result<u32, Error> {
    let sum = values.iter().sum::<u32>();
    if sum % bins != 0 {
        return Err(Error::SumOfWeightsNotDivisibleByNumberOfBins);
    }
    Ok(sum / bins)
}

fn solve(values: &[u32], bins: u32) -> Result<u64, Error> {
    if bins < 2 {
        return Err(Error::BadInput);
    }
    let target_sum = find_target_sum(values, bins)?;
    let subsets = find_smallest_subsets(values, target_sum);
    let mut min: Option<u64> = None;
    for (first_bin, left_to_distribute) in subsets {
        if will_fit_in_bins(&left_to_distribute, target_sum, (bins - 1) as usize) {
            let product = first_bin.iter().map(|&x| x as u64).product::<u64>();
            if let Some(x) = min {
                if product < x {
                    min = Some(product);
                }
            } else {
                min = Some(product);
            }
        }
    }
    Ok(min.ok_or(Error::NoSolution)?)
}

fn part_one(input: &str) -> Result<u64, Error> {
    let values = parse_input(input)?;
    solve(&values, 3)
}

fn part_two(input: &str) -> Result<u64, Error> {
    let values = parse_input(input)?;
    solve(&values, 4)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_subsets() {
        let v = find_subsets(&vec![1, 2, 3, 5, 6, 7], 2, 7);
        assert_eq!(v.len(), 2);
    }

    #[test]
    fn test_chomp() {
        let v = chomp(&vec![1, 2, 3]);
        assert_eq!(v.len(), 3);
        assert!(v.contains(&(1, vec![3, 2])));
        assert!(v.contains(&(2, vec![3, 1])));
        assert!(v.contains(&(3, vec![2, 1])));
    }
}
