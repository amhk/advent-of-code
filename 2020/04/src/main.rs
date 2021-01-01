use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let input = include_str!("input.txt");

    let answer = part_one(&input);
    println!("part 1: {}", answer);

    let answer = part_two(&input);
    println!("part 2: {}", answer);
}

#[derive(Debug, Eq, PartialEq)]
struct Passport<'a> {
    byr: Option<&'a str>,
    iyr: Option<&'a str>,
    eyr: Option<&'a str>,
    hgt: Option<&'a str>,
    hcl: Option<&'a str>,
    ecl: Option<&'a str>,
    pid: Option<&'a str>,
    cid: Option<&'a str>,
}

impl<'a> Passport<'a> {
    fn from_str(s: &'a str) -> Passport<'a> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"([a-z]{3}):(\S+)").unwrap();
        }

        let mut p = Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        };
        for cap in RE.captures_iter(s) {
            let key = cap.get(1).unwrap().as_str();
            let value = cap.get(2).unwrap().as_str();
            match key {
                "byr" => p.byr = Some(value),
                "iyr" => p.iyr = Some(value),
                "eyr" => p.eyr = Some(value),
                "hgt" => p.hgt = Some(value),
                "hcl" => p.hcl = Some(value),
                "ecl" => p.ecl = Some(value),
                "pid" => p.pid = Some(value),
                "cid" => p.cid = Some(value),
                _ => panic!("unexpected field '{}'", key),
            }
        }
        p
    }
}

fn byr_valid(s: &str) -> bool {
    s.parse::<u32>()
        .map_or(false, |year| (1920..=2002).contains(&year))
}

fn iyr_valid(s: &str) -> bool {
    s.parse::<u32>()
        .map_or(false, |year| (2010..=2020).contains(&year))
}

fn eyr_valid(s: &str) -> bool {
    s.parse::<u32>()
        .map_or(false, |year| (2020..=2030).contains(&year))
}

fn hgt_valid(s: &str) -> bool {
    if let Some(s) = s.strip_suffix("cm") {
        return s
            .parse::<u32>()
            .map_or(false, |height| (150..=193).contains(&height));
    }
    if let Some(s) = s.strip_suffix("in") {
        return s
            .parse::<u32>()
            .map_or(false, |height| (59..76).contains(&height));
    }
    false
}

fn hcl_valid(s: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    }
    RE.is_match(s)
}

fn ecl_valid(s: &str) -> bool {
    matches!(s, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
}

fn pid_valid(s: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\d{9}$").unwrap();
    }
    RE.is_match(s)
}

fn parse_input(input: &str) -> Vec<Passport> {
    input.split("\n\n").map(|s| Passport::from_str(s)).collect()
}

fn part_one(input: &str) -> usize {
    parse_input(input)
        .iter()
        .filter(|p| {
            p.byr.is_some()
                && p.iyr.is_some()
                && p.eyr.is_some()
                && p.hgt.is_some()
                && p.hcl.is_some()
                && p.ecl.is_some()
                && p.pid.is_some()
        })
        .count()
}

fn part_two(input: &str) -> usize {
    parse_input(input)
        .iter()
        .filter(|p| {
            p.byr.map_or(false, byr_valid)
                && p.iyr.map_or(false, iyr_valid)
                && p.eyr.map_or(false, eyr_valid)
                && p.hgt.map_or(false, hgt_valid)
                && p.hcl.map_or(false, hcl_valid)
                && p.ecl.map_or(false, ecl_valid)
                && p.pid.map_or(false, pid_valid)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_parse_input() {
        let passports = parse_input(INPUT);
        assert_eq!(passports.len(), 4);
        assert_eq!(
            passports[0],
            Passport {
                ecl: Some("gry"),
                pid: Some("860033327"),
                eyr: Some("2020"),
                hcl: Some("#fffffd"),
                byr: Some("1937"),
                iyr: Some("2017"),
                cid: Some("147"),
                hgt: Some("183cm"),
            }
        );
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 2);
    }

    #[test]
    fn test_part_two() {
        assert!(byr_valid("2002"));
        assert!(!byr_valid("2003"));

        assert!(iyr_valid("2010"));
        assert!(!iyr_valid("2000"));

        assert!(eyr_valid("2020"));
        assert!(!eyr_valid("2018"));

        assert!(hgt_valid("60in"));
        assert!(hgt_valid("190cm"));
        assert!(!hgt_valid("190in"));
        assert!(!hgt_valid("190"));

        assert!(hcl_valid("#123abc"));
        assert!(!hcl_valid("#123abz"));
        assert!(!hcl_valid("123abc"));

        assert!(ecl_valid("brn"));
        assert!(!ecl_valid("wat"));

        assert!(pid_valid("000000001"));
        assert!(!pid_valid("0123456789"));
    }
}
