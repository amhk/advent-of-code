fn main() {
    let input = include_str!("input.txt");

    let answer = part_one(&input).expect("no solution for part one");
    println!("part 1: {}", answer);

    let answer = part_two(&input).expect("no solution for part two");
    println!("part 2: {}", answer);
}

#[derive(Debug, PartialEq)]
enum Error {
    TooShort,
    MissingLeadingDoubleQuote,
    MissingTerminatingDoubleQuote,
    TrailingData,
    NonAscii,
    BadEscape,
}

#[derive(Debug, PartialEq)]
enum State {
    Null,    // outside "..."
    Ascii,   // default inside "..."
    Esc,     // right after \
    EscHex1, // right after \x
    EscHex2, // right after \x<hex>
}

fn count_in_memory_chars(s: &str) -> Result<usize, Error> {
    if s.len() < 2 {
        return Err(Error::TooShort);
    }
    if !s.starts_with('"') {
        return Err(Error::MissingLeadingDoubleQuote);
    }
    let mut count = 0;
    let mut state = State::Ascii;
    for ch in s.chars().skip(1) {
        match state {
            State::Null => return Err(Error::TrailingData),
            State::Ascii => {
                if ch == '\\' {
                    state = State::Esc;
                } else if ch == '"' {
                    state = State::Null;
                } else if ch.is_ascii() {
                    count += 1;
                } else {
                    return Err(Error::NonAscii);
                }
            }
            State::Esc => {
                if ch == '"' || ch == '\\' {
                    count += 1;
                    state = State::Ascii;
                } else if ch == 'x' {
                    state = State::EscHex1;
                } else {
                    return Err(Error::BadEscape);
                }
            }
            State::EscHex1 => {
                if ch.is_ascii_hexdigit() {
                    state = State::EscHex2;
                } else {
                    return Err(Error::BadEscape);
                }
            }
            State::EscHex2 => {
                if ch.is_ascii_hexdigit() {
                    count += 1;
                    state = State::Ascii;
                } else {
                    return Err(Error::BadEscape);
                }
            }
        }
    }
    if state == State::Null {
        Ok(count)
    } else {
        Err(Error::MissingTerminatingDoubleQuote)
    }
}

fn encode(s: &str) -> String {
    let s = s.replace('\\', "\\\\");
    let s = s.replace('"', "\\\"");
    format!(r#""{}""#, s)
}

fn part_one(input: &str) -> Result<usize, Error> {
    let mut count = 0;
    for line in input.lines() {
        count += line.len();
        count -= count_in_memory_chars(line)?;
    }
    Ok(count)
}

fn part_two(input: &str) -> Result<usize, Error> {
    let mut count = 0;
    for line in input.lines() {
        count += encode(line).len();
        count -= line.len();
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_in_memory_chars() {
        assert_eq!(count_in_memory_chars(r#""""#), Ok(0));
        assert_eq!(count_in_memory_chars(r#""abd""#), Ok(3));
        assert_eq!(count_in_memory_chars(r#""aaa\"aaa""#), Ok(7));
        assert_eq!(count_in_memory_chars(r#""\x27""#), Ok(1));
    }

    #[test]
    fn test_encode() {
        assert_eq!(encode(r#""""#), r#""\"\"""#);
        assert_eq!(encode(r#""abc""#), r#""\"abc\"""#);
        assert_eq!(encode(r#""aaa\"aaa""#), r#""\"aaa\\\"aaa\"""#);
        assert_eq!(encode(r#""\x27""#), r#""\"\\x27\"""#);
    }
}
