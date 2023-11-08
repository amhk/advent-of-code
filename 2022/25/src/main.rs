use anyhow::{Context, Result};
use std::fmt::Display;

fn main() -> Result<()> {
    let input = include_str!("input.txt");

    let answer = part_one(input).context("no solution for part one")?;
    println!("Part 1: {}", answer);
    assert_eq!(format!("{}", answer), "122-0==-=211==-2-200");

    Ok(())
}

#[derive(Debug, PartialEq)]
enum SnafuDigit {
    PositiveTwo,
    PositiveOne,
    Zero,
    NegativeOne,
    NegativeTwo,
}

#[derive(Debug, PartialEq)]
struct Snafu {
    digits: Vec<SnafuDigit>,
}

impl Snafu {
    fn to_i64(&self) -> i64 {
        let mut sum = 0;
        for (i, digit) in self.digits.iter().rev().enumerate() {
            let x = match digit {
                SnafuDigit::PositiveTwo => 2,
                SnafuDigit::PositiveOne => 1,
                SnafuDigit::Zero => 0,
                SnafuDigit::NegativeOne => -1,
                SnafuDigit::NegativeTwo => -2,
            };
            sum += x * 5_i64.pow(i as u32);
        }
        sum
    }
}

impl Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self
            .digits
            .iter()
            .map(|digit| match digit {
                SnafuDigit::PositiveTwo => '2',
                SnafuDigit::PositiveOne => '1',
                SnafuDigit::Zero => '0',
                SnafuDigit::NegativeOne => '-',
                SnafuDigit::NegativeTwo => '=',
            })
            .collect();
        write!(f, "{}", s)
    }
}

impl TryFrom<&str> for Snafu {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut digits = vec![];
        for ch in value.chars().rev() {
            digits.push(match ch {
                '2' => SnafuDigit::PositiveTwo,
                '1' => SnafuDigit::PositiveOne,
                '0' => SnafuDigit::Zero,
                '-' => SnafuDigit::NegativeOne,
                '=' => SnafuDigit::NegativeTwo,
                _ => {
                    return Err(format!("unexpected char '{}'", ch));
                }
            });
        }
        digits.reverse();
        Ok(Snafu { digits })
    }
}

impl From<i64> for Snafu {
    fn from(value: i64) -> Self {
        if value == 0 {
            return Self {
                digits: vec![SnafuDigit::Zero],
            };
        }

        let mut dec_value = value;
        let mut snafu_digits = vec![];
        while dec_value != 0 {
            let (snafu_digit, carry) = match dec_value % 5 {
                0 => (SnafuDigit::Zero, 0),
                1 => (SnafuDigit::PositiveOne, 0),
                2 => (SnafuDigit::PositiveTwo, 0),
                3 => (SnafuDigit::NegativeTwo, 1),
                4 => (SnafuDigit::NegativeOne, 1),
                _ => panic!("{}", dec_value),
            };
            snafu_digits.push(snafu_digit);
            dec_value = dec_value / 5 + carry;
        }
        snafu_digits.reverse();
        Self {
            digits: snafu_digits,
        }
    }
}

fn part_one(input: &str) -> Result<Snafu> {
    let sum: i64 = input
        .lines()
        .map(|s| Snafu::try_from(s).unwrap().to_i64())
        .sum();
    Ok(Snafu::from(sum))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_snafu_to_dec() {
        assert_eq!(Snafu::try_from("0").unwrap().to_i64(), 0);
        assert_eq!(Snafu::try_from("1").unwrap().to_i64(), 1);
        assert_eq!(Snafu::try_from("2").unwrap().to_i64(), 2);
        assert_eq!(Snafu::try_from("1=").unwrap().to_i64(), 3);
        assert_eq!(Snafu::try_from("1-").unwrap().to_i64(), 4);
        assert_eq!(Snafu::try_from("10").unwrap().to_i64(), 5);
        assert_eq!(Snafu::try_from("11").unwrap().to_i64(), 6);
        assert_eq!(Snafu::try_from("12").unwrap().to_i64(), 7);
        assert_eq!(Snafu::try_from("2=").unwrap().to_i64(), 8);
        assert_eq!(Snafu::try_from("2-").unwrap().to_i64(), 9);
        assert_eq!(Snafu::try_from("20").unwrap().to_i64(), 10);
        assert_eq!(Snafu::try_from("1=0").unwrap().to_i64(), 15);
        assert_eq!(Snafu::try_from("1-0").unwrap().to_i64(), 20);
        assert_eq!(Snafu::try_from("1=11-2").unwrap().to_i64(), 2022);
        assert_eq!(Snafu::try_from("1-0---0").unwrap().to_i64(), 12345);
        assert_eq!(
            Snafu::try_from("1121-1110-1=0").unwrap().to_i64(),
            314159265
        );
    }

    #[test]
    fn test_dec_to_snafu() {
        assert_eq!(format!("{}", Snafu::from(0)), "0");
        assert_eq!(format!("{}", Snafu::from(1)), "1");
        assert_eq!(format!("{}", Snafu::from(2)), "2");
        assert_eq!(format!("{}", Snafu::from(3)), "1=");
        assert_eq!(format!("{}", Snafu::from(4)), "1-");
        assert_eq!(format!("{}", Snafu::from(5)), "10");
        assert_eq!(format!("{}", Snafu::from(6)), "11");
        assert_eq!(format!("{}", Snafu::from(7)), "12");
        assert_eq!(format!("{}", Snafu::from(8)), "2=");
        assert_eq!(format!("{}", Snafu::from(9)), "2-");
        assert_eq!(format!("{}", Snafu::from(10)), "20");
        assert_eq!(format!("{}", Snafu::from(15)), "1=0");
        assert_eq!(format!("{}", Snafu::from(20)), "1-0");
        assert_eq!(format!("{}", Snafu::from(2022)), "1=11-2");
        assert_eq!(format!("{}", Snafu::from(12345)), "1-0---0");
        assert_eq!(format!("{}", Snafu::from(314159265)), "1121-1110-1=0");
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), Snafu::try_from("2=-1=0").unwrap());
    }
}
