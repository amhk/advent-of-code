use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");

    let answer = part_one(input).expect("no solution for part one");
    println!("part 1: {}", answer);

    let answer = part_two(input).expect("no solution for part two");
    println!("part 2: {}", answer);
}

#[derive(Debug, PartialEq)]
enum Error {
    BadInput,
}

#[derive(Debug)]
struct Rule {
    from: String,
    to: String,
}

fn parse_input(input: &str) -> Result<(Vec<Rule>, String), Error> {
    let mut rules = Vec::new();
    let parts: Vec<_> = input.split("\n\n").collect();
    for line in parts.first().ok_or(Error::BadInput)?.lines() {
        let tmp: Vec<_> = line.split_whitespace().collect();
        rules.push(Rule {
            from: tmp.first().ok_or(Error::BadInput)?.to_string(),
            to: tmp.get(2).ok_or(Error::BadInput)?.to_string(),
        });
    }
    let molecule = parts.get(1).ok_or(Error::BadInput)?.trim().to_string();
    Ok((rules, molecule))
}

fn expand(rules: &[Rule], molecule: &str) -> HashSet<String> {
    fn partition<'a>(s: &'a str, substr: &str) -> Vec<(&'a str, &'a str, &'a str)> {
        debug_assert!(!substr.is_empty());
        let mut out = Vec::new();
        let mut ss = s;
        while !ss.is_empty() {
            if ss.starts_with(substr) {
                let a = s.len() - ss.len();
                let b = s.len() - ss.len() + substr.len();
                out.push((&s[..a], &s[a..b], &s[b..]));
            }
            ss = &ss[1..];
        }
        out
    }

    let mut set = HashSet::new();
    for rule in rules {
        for (before, _, after) in partition(molecule, &rule.from) {
            set.insert(format!("{}{}{}", before, rule.to, after));
        }
    }
    set
}

fn construct(rules: &[Rule], molecule: &str) -> Option<usize> {
    // reverse rules: we will search from the target molecule down to just "e"
    let rules: Vec<_> = rules
        .iter()
        .map(|r| Rule {
            from: r.to.clone(),
            to: r.from.clone(),
        })
        .inspect(|r| {
            debug_assert!(r.to.len() <= r.from.len());
        })
        .collect();

    let mut molecule = molecule.to_string();
    for i in 1.. {
        let expanded = expand(&rules, &molecule);
        if expanded.contains("e") {
            return Some(i);
        }

        // impose bounds on the search space: because we search from <long string> to "e", and
        // because we have verified that all expansions expand to an equally long or shorter
        // string, and because we *assume* the puzzle input does not contain any dead ends that
        // would force us to have to back up, only keep the smallest expanded string
        molecule = expanded.iter().min_by_key(|s| s.len())?.to_string();
    }
    None
}

fn part_one(input: &str) -> Result<usize, Error> {
    let (rules, molecule) = parse_input(input)?;
    Ok(expand(&rules, &molecule).len())
}

fn part_two(input: &str) -> Result<usize, Error> {
    let (rules, molecule) = parse_input(input)?;
    construct(&rules, &molecule).ok_or(Error::BadInput)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_expand() {
        let (rules, molecule) = parse_input(INPUT).unwrap();
        assert_eq!(molecule, "HOH");
        assert_eq!(expand(&rules, "HOH").len(), 4);
        assert_eq!(expand(&rules, "HOHOHO").len(), 7);
    }

    #[test]
    fn test_construct() {
        let (mut rules, _) = parse_input(INPUT).unwrap();
        rules.push(Rule {
            from: "e".to_string(),
            to: "H".to_string(),
        });
        rules.push(Rule {
            from: "e".to_string(),
            to: "O".to_string(),
        });
        assert_eq!(construct(&rules, "HOH"), Some(3));
        //assert_eq!(construct(&rules, "HOHOHO"), Some(6));
    }
}
