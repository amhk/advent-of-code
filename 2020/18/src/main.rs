fn main() {
    let input = include_str!("input.txt");

    let answer = part_one(input).expect("no solution for part one");
    println!("part 1: {}", answer);

    let answer = part_two(input).expect("no solution for part two");
    println!("part 2: {}", answer);
}

#[derive(Debug, PartialEq)]
enum Error {
    BadInput(String),
    Internal(&'static str),
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Token {
    Int(u64),
    Plus,
    Mul,
    LParen,
    RParen,
}

fn tokenize(input: &str) -> Result<Vec<Token>, Error> {
    let input = input.replace('+', " + ");
    let input = input.replace('*', " * ");
    let input = input.replace('(', " ( ");
    let input = input.replace(')', " ) ");
    let mut tokens = Vec::new();
    for word in input.split_whitespace() {
        tokens.push(match word {
            "+" => Token::Plus,
            "*" => Token::Mul,
            "(" => Token::LParen,
            ")" => Token::RParen,
            _ => Token::Int(
                word.parse::<u64>()
                    .map_err(|_| Error::BadInput(word.to_string()))?,
            ),
        });
    }
    Ok(tokens)
}

#[derive(Debug, PartialEq)]
enum Precedence {
    Same,
    PlusHigherThanMul,
}

fn infix_to_postfix(tokens: &[Token], precedence: Precedence) -> Result<Vec<Token>, Error> {
    let mut queue = Vec::new();
    let mut stack = Vec::new();

    for tok in tokens.iter() {
        match tok {
            Token::Int(value) => queue.push(Token::Int(*value)),
            Token::Plus => {
                match stack.last() {
                    Some(Token::Plus) => {
                        queue.push(stack.pop().unwrap());
                    }
                    Some(Token::Mul) => {
                        if precedence == Precedence::Same {
                            queue.push(stack.pop().unwrap());
                        }
                    }
                    Some(Token::Int(_)) => unreachable!(),
                    _ => {}
                }
                stack.push(*tok);
            }
            Token::Mul => {
                match stack.last() {
                    Some(Token::Plus) | Some(Token::Mul) => {
                        queue.push(stack.pop().unwrap());
                    }
                    _ => {}
                }
                stack.push(*tok);
            }
            Token::LParen => {
                stack.push(*tok);
            }
            Token::RParen => loop {
                match stack.pop() {
                    None => return Err(Error::BadInput("imbalanced parens".to_string())),
                    Some(Token::Int(_)) => unreachable!(),
                    Some(Token::Plus) => queue.push(Token::Plus),
                    Some(Token::Mul) => queue.push(Token::Mul),
                    Some(Token::LParen) => break,
                    Some(Token::RParen) => unreachable!(),
                }
            },
        }
    }

    while let Some(element) = stack.pop() {
        queue.push(element);
    }

    Ok(queue)
}

fn eval_postfix(tokens: &[Token]) -> Result<u64, Error> {
    let mut stack = Vec::new();
    for tok in tokens.iter() {
        match tok {
            Token::Int(value) => stack.push(*value),
            Token::Plus => {
                let a = stack.pop().ok_or(Error::Internal("empty stack"))?;
                let b = stack.pop().ok_or(Error::Internal("empty stack"))?;
                stack.push(a + b);
            }
            Token::Mul => {
                let a = stack.pop().ok_or(Error::Internal("empty stack"))?;
                let b = stack.pop().ok_or(Error::Internal("empty stack"))?;
                stack.push(a * b);
            }
            Token::LParen => unreachable!(),
            Token::RParen => unreachable!(),
        }
    }

    match stack.len() {
        0 => Err(Error::Internal("empty stack")),
        1 => Ok(stack.pop().unwrap()),
        _ => Err(Error::Internal("stack contains more than one element")),
    }
}

fn part_one(input: &str) -> Result<u64, Error> {
    Ok(input
        .lines()
        .map(|line| {
            let tokens = tokenize(line).unwrap();
            let tokens = infix_to_postfix(&tokens, Precedence::Same).unwrap();
            eval_postfix(&tokens).unwrap()
        })
        .sum())
}

fn part_two(input: &str) -> Result<u64, Error> {
    Ok(input
        .lines()
        .map(|line| {
            let tokens = tokenize(line).unwrap();
            let tokens = infix_to_postfix(&tokens, Precedence::PlusHigherThanMul).unwrap();
            eval_postfix(&tokens).unwrap()
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let tokens = tokenize("2 * 3 + (4 * 5)").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Int(2),
                Token::Mul,
                Token::Int(3),
                Token::Plus,
                Token::LParen,
                Token::Int(4),
                Token::Mul,
                Token::Int(5),
                Token::RParen,
            ]
        );
    }

    #[test]
    fn test_infix_to_postfix() {
        let tokens = tokenize("2 * 3 + (4 * 5)").unwrap();
        assert_eq!(
            infix_to_postfix(&tokens, Precedence::Same),
            Ok(vec![
                Token::Int(2),
                Token::Int(3),
                Token::Mul,
                Token::Int(4),
                Token::Int(5),
                Token::Mul,
                Token::Plus,
            ])
        );
        assert_eq!(
            infix_to_postfix(&tokens, Precedence::PlusHigherThanMul),
            Ok(vec![
                Token::Int(2),
                Token::Int(3),
                Token::Int(4),
                Token::Int(5),
                Token::Mul,
                Token::Plus,
                Token::Mul,
            ])
        );
    }

    #[test]
    fn test_eval_postfix() {
        fn eval1(s: &str) -> Result<u64, Error> {
            let tokens = tokenize(s)?;
            let tokens = infix_to_postfix(&tokens, Precedence::Same)?;
            eval_postfix(&tokens)
        }
        assert_eq!(eval1("1 + 2 * 3 + 4 * 5 + 6"), Ok(71));
        assert_eq!(eval1("1 + (2 * 3) + (4 * (5 + 6))"), Ok(51));
        assert_eq!(eval1("2 * 3 + (4 * 5)"), Ok(26));
        assert_eq!(eval1("5 + (8 * 3 + 9 + 3 * 4 * 3)"), Ok(437));
        assert_eq!(
            eval1("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            Ok(12240)
        );
        assert_eq!(
            eval1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            Ok(13632)
        );

        fn eval2(s: &str) -> Result<u64, Error> {
            let tokens = tokenize(s)?;
            let tokens = infix_to_postfix(&tokens, Precedence::PlusHigherThanMul)?;
            eval_postfix(&tokens)
        }
        assert_eq!(eval2("1 + 2 * 3 + 4 * 5 + 6"), Ok(231));
        assert_eq!(eval2("1 + (2 * 3) + (4 * (5 + 6))"), Ok(51));
        assert_eq!(eval2("2 * 3 + (4 * 5)"), Ok(46));
        assert_eq!(eval2("5 + (8 * 3 + 9 + 3 * 4 * 3)"), Ok(1445));
        assert_eq!(
            eval2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            Ok(669060)
        );
        assert_eq!(
            eval2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            Ok(23340)
        );
    }
}
