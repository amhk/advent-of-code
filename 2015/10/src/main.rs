fn main() {
    let input = include_str!("input.txt").trim();

    let answer = part_one(&input);
    println!("part 1: {}", answer);

    let answer = part_two(&input);
    println!("part 2: {}", answer);
}

fn look_and_say(input: &str) -> String {
    let mut state: Option<(char, usize)> = None;
    let mut output = String::new();
    for ch in input.chars() {
        debug_assert!(ch.is_ascii_digit());
        if let Some(ref mut s) = state {
            if s.0 == ch {
                s.1 += 1;
            } else {
                output.push_str(&format!("{}{}", s.1, s.0));
                state = Some((ch, 1));
            }
        } else {
            state = Some((ch, 1));
        }
    }
    if let Some(s) = state {
        output.push_str(&format!("{}{}", s.1, s.0));
    }
    output
}

fn part_one(input: &str) -> usize {
    let mut input = input.to_owned();
    for _ in 0..40 {
        input = look_and_say(&input);
    }
    input.len()
}

fn part_two(input: &str) -> usize {
    let mut input = input.to_owned();
    for _ in 0..50 {
        input = look_and_say(&input);
    }
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_look_and_say() {
        assert_eq!(look_and_say("1"), "11");
        assert_eq!(look_and_say("11"), "21");
        assert_eq!(look_and_say("21"), "1211");
        assert_eq!(look_and_say("1211"), "111221");
        assert_eq!(look_and_say("111221"), "312211");
    }
}
