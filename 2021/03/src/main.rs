use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");

    let answer = part_one(input).expect("no solution for part one");
    println!("part 1: {}", answer);

    let answer = part_two(input).expect("no solution for part two");
    println!("part 2: {}", answer);
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum Error {
    BadInput,
    NoSolution,
}

fn part_one(input: &str) -> Result<u32, Error> {
    let n_numbers = input.lines().count();
    let n_bits = input.lines().next().ok_or(Error::BadInput)?.len();
    let mut ones = vec![0; n_bits];

    for line in input.lines() {
        if line.len() != n_bits {
            return Err(Error::BadInput);
        }
        for (i, ch) in line.chars().enumerate() {
            match ch {
                '0' => {} // no-op
                '1' => ones[i] += 1,
                _ => return Err(Error::BadInput),
            }
        }
    }

    let mut gamma = String::with_capacity(n_bits);
    let mut epsilon = String::with_capacity(n_bits);
    for count in ones {
        // n_numbers may be even; assume the number of 1s must be strictly higher than the number
        // of 0s to set the bit to 1 in gamma
        if count > n_numbers / 2 {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }

    let gamma = u32::from_str_radix(&gamma, 2).map_err(|_| Error::BadInput)?;
    let epsilon = u32::from_str_radix(&epsilon, 2).map_err(|_| Error::BadInput)?;

    Ok(gamma * epsilon)
}

fn part_two(input: &str) -> Result<u32, Error> {
    fn filter_out_value<F>(input: &str, comparator: F) -> Result<String, Error>
    where
        F: std::ops::Fn(usize, usize) -> bool,
    {
        let n_bits = input.lines().next().ok_or(Error::BadInput)?.len();

        let mut numbers = HashSet::new();
        for line in input.lines() {
            numbers.insert(line);
        }

        for i in 0..n_bits {
            let (zeroes, ones) = numbers
                .iter()
                .fold((0, 0), |acc, s| match s.chars().nth(i) {
                    Some('0') => (acc.0 + 1, acc.1),
                    Some('1') => (acc.0, acc.1 + 1),
                    _ => panic!(),
                });

            if comparator(zeroes, ones) {
                numbers.retain(|&s| s.chars().nth(i) == Some('1'));
            } else {
                numbers.retain(|&s| s.chars().nth(i) == Some('0'));
            }

            if numbers.len() == 1 {
                return Ok(numbers.iter().next().unwrap().to_string());
            }
        }

        Err(Error::NoSolution)
    }

    let oxygen = filter_out_value(input, |zeroes, ones| ones >= zeroes)?;
    let co2 = filter_out_value(input, |zeroes, ones| ones < zeroes)?;

    let oxygen = u32::from_str_radix(&oxygen, 2).map_err(|_| Error::BadInput)?;
    let co2 = u32::from_str_radix(&co2, 2).map_err(|_| Error::BadInput)?;

    Ok(oxygen * co2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), Ok(198));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), Ok(230));
    }
}
