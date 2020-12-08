use regex::Regex;

fn main() -> Result<(), ()> {
    let input = include_str!("input.txt");

    let answer = part_one(&input)?;
    println!("part 1: {}", answer);

    let answer = part_two(&input)?;
    println!("part 2: {}", answer);

    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
struct Password<'a> {
    min: usize,
    max: usize,
    ch: char,
    password: &'a str,
}

impl<'a> Password<'a> {
    fn is_valid_method_one(&self) -> bool {
        let count = self.password.chars().filter(|ch| ch == &self.ch).count();
        count >= self.min && count <= self.max
    }

    fn is_valid_method_two(&self) -> bool {
        let a = self.password.chars().nth(self.min - 1).unwrap();
        let b = self.password.chars().nth(self.max - 1).unwrap();
        (a == self.ch) ^ (b == self.ch)
    }
}

fn split_input(input: &str) -> Vec<Password> {
    let regex = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    regex
        .captures_iter(input)
        .map(|cap| Password {
            min: cap.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            max: cap.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            ch: cap.get(3).unwrap().as_str().chars().next().unwrap(),
            password: cap.get(4).unwrap().as_str(),
        })
        .inspect(|p| {
            assert!(p.min <= p.password.len());
            assert!(p.max <= p.password.len());
            assert!(p.min <= p.max);
        })
        .collect()
}

fn part_one(input: &str) -> Result<usize, ()> {
    Ok(split_input(input)
        .iter()
        .filter(|p| p.is_valid_method_one())
        .count())
}

fn part_two(input: &str) -> Result<usize, ()> {
    Ok(split_input(input)
        .iter()
        .filter(|p| p.is_valid_method_two())
        .count())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_split_input() {
        let input = split_input(INPUT);
        assert_eq!(input.len(), 3);
        assert_eq!(
            input[0],
            Password {
                ch: 'a',
                min: 1,
                max: 3,
                password: "abcde",
            }
        );
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&INPUT), Ok(2));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&INPUT), Ok(1));
    }
}
