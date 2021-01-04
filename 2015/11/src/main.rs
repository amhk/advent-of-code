use ascii::AsciiChar;
use itertools::izip;
use std::fmt::Display;
use std::str::FromStr;

fn main() {
    let input = include_str!("input.txt").trim();

    let answer = part_one(&input).expect("no solution for part one");
    println!("part 1: {}", answer);

    let answer = part_two(&input).expect("no solution for part two");
    println!("part 2: {}", answer);
}

#[derive(Debug, PartialEq)]
enum Error {
    AsciiChar,
    InvalidInput,
    EmptyInput,
}

struct AlmostBase26 {
    // pos 0: LSB
    digits: Vec<AsciiChar>,
}

impl AlmostBase26 {
    fn increment(&mut self) {
        self.inc(0);
    }

    fn inc(&mut self, index: usize) {
        if index == self.digits.len() {
            self.digits.push(AsciiChar::a);
        } else {
            let value = self.digits[index].as_byte();
            if value < AsciiChar::z {
                self.digits[index] = AsciiChar::from_ascii(value + 1).unwrap();
            } else {
                self.digits[index] = AsciiChar::a;
                self.inc(index + 1);
            }
        }
    }
}

impl FromStr for AlmostBase26 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(Error::EmptyInput);
        }
        let mut digits = Vec::new();
        for ch in s.chars().rev() {
            if !ch.is_ascii_lowercase() {
                return Err(Error::InvalidInput);
            }
            let ascii = AsciiChar::from_ascii(ch).map_err(|_| Error::AsciiChar)?;
            digits.push(ascii);
        }
        Ok(AlmostBase26 { digits })
    }
}

impl Display for AlmostBase26 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for ch in self.digits.iter().rev() {
            s.push_str(&format!("{}", ch));
        }
        f.write_str(&s)
    }
}

fn is_valid_password(s: &str) -> bool {
    // rule 1: at least one three char straight (increasing chars of diff 1)
    if !izip!(s.chars(), s.chars().skip(1), s.chars().skip(2))
        .map(|(a, b, c)| (a as u8, b as u8, c as u8))
        .any(|(a, b, c)| a == b - 1 && b == c - 1)
    {
        return false;
    }

    // rule 2: no i, o or l
    if s.chars().any(|ch| ch == 'i' || ch == 'o' || ch == 'l') {
        return false;
    }

    // rule 3: at least two different, non-overlapping pairs of chars
    if {
        let mut i = 0;
        let mut count = 0;
        while i < s.len() - 1 {
            if s.chars().nth(i) == s.chars().nth(i + 1) {
                count += 1;
                i += 2;
            } else {
                i += 1;
            }
        }
        count
    } < 2
    {
        return false;
    }

    true
}

fn find_next_valid_password(s: &str) -> Result<String, Error> {
    let mut password = AlmostBase26::from_str(s)?;
    password.increment();
    while !is_valid_password(&format!("{}", password)) {
        password.increment();
    }
    Ok(format!("{}", password))
}

fn part_one(input: &str) -> Result<String, Error> {
    find_next_valid_password(input)
}

fn part_two(input: &str) -> Result<String, Error> {
    find_next_valid_password(&find_next_valid_password(input)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_almostbase64() {
        let mut a = AlmostBase26::from_str("y").unwrap();
        assert_eq!(format!("{}", a), "y");
        a.increment();
        assert_eq!(format!("{}", a), "z");
        a.increment();
        assert_eq!(format!("{}", a), "aa");

        let mut a = AlmostBase26::from_str("xx").unwrap();
        assert_eq!(format!("{}", a), "xx");
        a.increment();
        assert_eq!(format!("{}", a), "xy");
        a.increment();
        assert_eq!(format!("{}", a), "xz");
        a.increment();
        assert_eq!(format!("{}", a), "ya");
        a.increment();
        assert_eq!(format!("{}", a), "yb");
    }

    #[test]
    fn test_is_valid_password() {
        assert!(is_valid_password("abcdffaa"));
        assert!(is_valid_password("ghjaabcc"));

        assert!(!is_valid_password("hijklmmn"));
        assert!(!is_valid_password("abbceffg"));
        assert!(!is_valid_password("abbcegjk"));
        assert!(!is_valid_password("abcdeggg"));
    }

    #[test]
    #[ignore] // expensive test, enable via 'cargo test -- --ignored'
    fn test_find_next_valid_password() {
        assert_eq!(
            find_next_valid_password("abcdefgh"),
            Ok("abcdffaa".to_string())
        );
        assert_eq!(
            find_next_valid_password("ghijklmn"),
            Ok("ghjaabcc".to_string())
        );
    }
}
