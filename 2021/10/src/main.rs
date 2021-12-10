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

#[derive(Debug, PartialEq)]
enum ParseError {
    Incomplete(String),  // missing chars, in order that would make a valid line
    Corrupt(char, char), // expected char, actual char
    UnexpectedChar(char),
}

fn parse_line(line: &str) -> Result<(), ParseError> {
    fn inverse(ch: char) -> char {
        match ch {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => panic!(),
        }
    }

    let mut stack = vec![];
    for ch in line.chars() {
        match ch {
            '(' | '[' | '{' | '<' => stack.push(ch),
            ')' | ']' | '}' | '>' => {
                let top = stack.pop().ok_or(ParseError::UnexpectedChar(ch))?;
                let pot = inverse(top);
                if pot != ch {
                    return Err(ParseError::Corrupt(pot, ch));
                }
            }
            _ => return Err(ParseError::UnexpectedChar(ch)),
        }
    }
    if !stack.is_empty() {
        let s = stack.iter().rev().map(|ch| inverse(*ch)).collect();
        return Err(ParseError::Incomplete(s));
    }
    Ok(())
}

fn part_one(input: &str) -> Result<usize, Error> {
    let mut sum = 0;
    for line in input.lines() {
        if let Err(ParseError::Corrupt(_, actual)) = parse_line(line) {
            sum += match actual {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => return Err(Error::BadInput),
            };
        }
    }
    Ok(sum)
}

fn part_two(input: &str) -> Result<usize, Error> {
    let mut scores = vec![];
    for line in input.lines() {
        if let Err(ParseError::Incomplete(missing)) = parse_line(line) {
            let score = missing.chars().fold(0, |acc, ch| {
                let char_score = match ch {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => panic!(),
                };
                acc * 5 + char_score
            });
            scores.push(score);
        }
    }
    if scores.len() % 2 != 1 {
        // input guaranteed to have an odd number of incomplete lines
        return Err(Error::BadInput);
    }
    scores.sort_unstable();
    Ok(scores[scores.len() / 2])
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_parse_line_ok() {
        assert_eq!(parse_line("()"), Ok(()));
        assert_eq!(parse_line("([])"), Ok(()));
        assert_eq!(parse_line("{()()()}"), Ok(()));
        assert_eq!(parse_line("<([{}])>"), Ok(()));
        assert_eq!(parse_line("[<>({}){}[([])<>]]"), Ok(()));
        assert_eq!(parse_line("(((((((((())))))))))"), Ok(()));
    }

    #[test]
    fn test_parse_line_corrupt() {
        assert_eq!(
            parse_line("{([(<{}[<>[]}>{[]{[(<()>"),
            Err(ParseError::Corrupt(']', '}'))
        );
        assert_eq!(
            parse_line("[[<[([]))<([[{}[[()]]]"),
            Err(ParseError::Corrupt(']', ')'))
        );
        assert_eq!(
            parse_line("[{[{({}]{}}([{[{{{}}([]"),
            Err(ParseError::Corrupt(')', ']'))
        );
        assert_eq!(
            parse_line("[<(<(<(<{}))><([]([]()"),
            Err(ParseError::Corrupt('>', ')'))
        );
        assert_eq!(
            parse_line("<{([([[(<>()){}]>(<<{{"),
            Err(ParseError::Corrupt(']', '>'))
        );
    }

    #[test]
    fn test_parse_line_incomplete() {
        assert_eq!(
            parse_line("[({(<(())[]>[[{[]{<()<>>"),
            Err(ParseError::Incomplete("}}]])})]".to_string()))
        );
        assert_eq!(
            parse_line("[(()[<>])]({[<{<<[]>>("),
            Err(ParseError::Incomplete(")}>]})".to_string()))
        );
        assert_eq!(
            parse_line("(((({<>}<{<{<>}{[]{[]{}"),
            Err(ParseError::Incomplete("}}>}>))))".to_string()))
        );
        assert_eq!(
            parse_line("{<[[]]>}<{[{[{[]{()[[[]"),
            Err(ParseError::Incomplete("]]}}]}]}>".to_string()))
        );
        assert_eq!(
            parse_line("<{([{{}}[<[[[<>{}]]]>[]]"),
            Err(ParseError::Incomplete("])}>".to_string()))
        );
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), Ok(26397));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), Ok(288957));
    }
}
