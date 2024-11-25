fn main() {
    let input = include_str!("input.txt");

    let answer = count(input, false).expect("no solution for part one");
    println!("part 1: {}", answer);

    let answer = count(input, true).expect("no solution for part two");
    println!("part 2: {}", answer);
}

#[derive(Debug, PartialEq)]
enum Error {
    BadInput,
}

#[derive(Debug, PartialEq, Clone)]
enum Node {
    Leaf(char),
    Tree(Vec<Vec<usize>>),
}

impl Node {
    fn matches(&self, use_hardcoded_hack: bool, nodes: &[Node], input: &str) -> bool {
        let mut input = input.chars().collect::<Vec<_>>();
        if use_hardcoded_hack {
            // Input guarantees:
            //   Nothing loops back to 0
            //   Nothing loops back to 8 (except 8)
            //   Nothing loops back to 11 (except 11)
            //   There are no loops (except 8 and 11)
            //
            // Loop expansions (a == 42, b == 31):
            //  0: 8 11
            //  8: a   | a 8    | a a 8      | a a a 8        | ...
            // 11: a b | a 11 b | a a 11 b b | a a a 11 b b b | ...
            //
            // Strategy:
            //   (1) parse 42 n > 0 times
            //   (2) parse 31 m > 0 times
            //   (3) OK if input is completely consumed and m < n

            // (1)
            let node = &nodes[42];
            let mut count42 = 0;
            loop {
                if input.is_empty() {
                    break;
                }
                match node.matches0(nodes, &input) {
                    Some(remainder) => {
                        input = remainder.to_vec();
                        count42 += 1;
                    }
                    None => {
                        break;
                    }
                }
            }
            if count42 == 0 {
                return false;
            }

            // (2)
            let node = &nodes[31];
            let mut count31 = 0;
            loop {
                if input.is_empty() {
                    break;
                }
                match node.matches0(nodes, &input) {
                    Some(remainder) => {
                        input = remainder.to_vec();
                        count31 += 1;
                    }
                    None => {
                        break;
                    }
                }
            }
            if count31 == 0 {
                return false;
            }

            // (3)
            input.is_empty() && count31 < count42
        } else {
            self.matches0(nodes, &input) == Some(&[])
        }
    }

    fn matches0<'a>(&self, nodes: &[Node], input: &'a [char]) -> Option<&'a [char]> {
        assert!(!input.is_empty());
        match self {
            Node::Leaf(ch) => {
                if ch == &input[0] {
                    return Some(&input[1..]);
                }
                None
            }
            Node::Tree(v) => {
                for subtree in v {
                    let mut i = input;
                    let mut fail = false;
                    for index in subtree {
                        if i.is_empty() {
                            fail = true;
                            break;
                        }
                        let node = &nodes[*index];
                        match node.matches0(nodes, i) {
                            None => {
                                fail = true;
                                break;
                            }
                            Some(ii) => i = ii,
                        }
                    }
                    if !fail {
                        return Some(i);
                    }
                }
                None
            }
        }
    }
}

fn parse_input(input: &str) -> Result<(Vec<Node>, &str), Error> {
    fn parse_tree(input: &str) -> Result<Node, Error> {
        let mut v = vec![Vec::new()];
        for word in input.split_whitespace() {
            match word {
                "|" => v.push(Vec::new()),
                _ => {
                    let index = word.parse::<usize>().map_err(|_| Error::BadInput)?;
                    v.last_mut().unwrap().push(index);
                }
            }
        }
        Ok(Node::Tree(v))
    }

    let mut parts = input.split("\n\n");
    let first = parts.next().ok_or(Error::BadInput)?;
    let second = parts.next().ok_or(Error::BadInput)?;

    let mut nodes = vec![Node::Leaf('x'); first.lines().count()];
    for line in first.lines() {
        let mut parts = line.split(':');
        let index = parts
            .next()
            .ok_or(Error::BadInput)?
            .parse::<usize>()
            .map_err(|_| Error::BadInput)?;

        let rest = parts.next().ok_or(Error::BadInput)?;
        let node = match rest {
            " \"a\"" => Node::Leaf('a'),
            " \"b\"" => Node::Leaf('b'),
            _ => parse_tree(rest)?,
        };

        *nodes.get_mut(index).unwrap() = node;
    }

    Ok((nodes, second))
}

fn count(input: &str, use_hardcoded_hack: bool) -> Result<usize, Error> {
    let (nodes, input) = parse_input(input)?;
    let root = nodes.first().unwrap();
    Ok(input
        .lines()
        .filter(|line| root.matches(use_hardcoded_hack, &nodes, line))
        .count())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_parse_input() {
        let (_, input) = parse_input(INPUT).unwrap();
        assert_eq!(input.lines().count(), 5);
    }

    #[test]
    fn test_node_matches_only_leafs() {
        let nodes = vec![Node::Leaf('a')];
        let root = nodes.first().unwrap();
        assert!(root.matches(false, &nodes, "a"));
        assert!(!root.matches(false, &nodes, "b"));
    }

    #[test]
    fn test_node_matches_single_branch_depth_1() {
        let nodes = vec![Node::Tree(vec![vec![1]]), Node::Leaf('a')];
        let root = nodes.first().unwrap();
        assert!(root.matches(false, &nodes, "a"));
        assert!(!root.matches(false, &nodes, "b"));
    }

    #[test]
    fn test_node_matches_single_branch_depth_2() {
        let nodes = vec![
            Node::Tree(vec![vec![1]]),
            Node::Tree(vec![vec![2]]),
            Node::Leaf('a'),
        ];
        let root = nodes.first().unwrap();
        assert!(root.matches(false, &nodes, "a"));
        assert!(!root.matches(false, &nodes, "b"));
    }

    #[test]
    fn test_node_matches_single_fork() {
        let nodes = vec![
            Node::Tree(vec![vec![1], vec![2]]),
            Node::Leaf('a'),
            Node::Leaf('b'),
        ];
        let root = nodes.first().unwrap();
        assert!(root.matches(false, &nodes, "a"));
        assert!(root.matches(false, &nodes, "b"));
    }

    #[test]
    fn test_node_matches_test_input() {
        let (nodes, _) = parse_input(INPUT).unwrap();
        let root = nodes.first().unwrap();

        assert!(root.matches(false, &nodes, "ababbb"));
        assert!(root.matches(false, &nodes, "abbbab"));
        assert!(!root.matches(false, &nodes, "bababa"));
        assert!(!root.matches(false, &nodes, "aaabbb"));
        assert!(!root.matches(false, &nodes, "aaaabbb"));
    }
}
