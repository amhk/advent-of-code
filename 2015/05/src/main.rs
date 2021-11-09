use fancy_regex::Regex as FancyRegex;
use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let input = include_str!("input.txt");

    let answer = part_one(input);
    println!("part 1: {}", answer);

    let answer = part_two(input);
    println!("part 2: {}", answer);
}

#[derive(Debug, PartialEq)]
enum State {
    Naughty,
    Nice,
}

fn classify_v1(text: &str) -> State {
    lazy_static! {
        static ref RE_FORBIDDEN_SUBSTRINGS: Regex = Regex::new(r"ab|cd|pq|xy").unwrap();
        static ref RE_VOWELS: Regex = Regex::new(r"a|e|i|o|u").unwrap();
        static ref RE_DOUBLE_LETTER: FancyRegex = FancyRegex::new(r"(.)\1").unwrap();
    };
    if RE_FORBIDDEN_SUBSTRINGS.is_match(text) {
        return State::Naughty;
    }
    if RE_VOWELS.find_iter(text).count() < 3 {
        return State::Naughty;
    }
    if !RE_DOUBLE_LETTER.is_match(text).unwrap() {
        return State::Naughty;
    }
    State::Nice
}

fn classify_v2(text: &str) -> State {
    lazy_static! {
        static ref RE_LETTER_PAIR_TWICE: FancyRegex = FancyRegex::new(r"(..).*\1").unwrap();
        static ref RE_LETTER_REPEAT_ONE_LETTER_BETWEEN: FancyRegex =
            FancyRegex::new(r"(.).\1").unwrap();
    };
    if !RE_LETTER_PAIR_TWICE.is_match(text).unwrap() {
        return State::Naughty;
    }
    if !RE_LETTER_REPEAT_ONE_LETTER_BETWEEN.is_match(text).unwrap() {
        return State::Naughty;
    }
    State::Nice
}

fn part_one(input: &str) -> usize {
    input
        .lines()
        .filter(|line| classify_v1(line) == State::Nice)
        .count()
}

fn part_two(input: &str) -> usize {
    input
        .lines()
        .filter(|line| classify_v2(line) == State::Nice)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_v1() {
        assert_eq!(classify_v1("ugknbfddgicrmopn"), State::Nice);
        assert_eq!(classify_v1("aaa"), State::Nice);
        assert_eq!(classify_v1("jchzalrnumimnmhp"), State::Naughty);
        assert_eq!(classify_v1("haegwjzuvuyypxyu"), State::Naughty);
        assert_eq!(classify_v1("dvszwmarrgswjxmb"), State::Naughty);
    }

    #[test]
    fn test_classify_v2() {
        assert_eq!(classify_v2("qjhvhtzxzqqjkmpb"), State::Nice);
        assert_eq!(classify_v2("xxyxx"), State::Nice);
        assert_eq!(classify_v2("uurcxstgmygtbstg"), State::Naughty);
        assert_eq!(classify_v2("ieodomkazucvgmuy"), State::Naughty);
    }
}
