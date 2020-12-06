fn main() {
    let input = include_str!("input.txt");

    let answer = part_one(&input);
    println!("part 1: {}", answer);

    let answer = part_two(&input);
    println!("part 2: {}", answer);
}

#[derive(Debug, Eq, PartialEq)]
enum Op {
    Lower,
    Upper,
}

fn binary_space_partition(input: &[Op]) -> u32 {
    let mut min = 0;
    let mut max = 2_u32.pow(input.len() as u32) - 1;
    for op in input {
        let mid = (max - min) / 2 + min;
        match op {
            Op::Lower => max = mid,
            Op::Upper => min = mid + 1,
        }
    }
    min
}

fn parse_input(input: &str, lower: char, upper: char) -> Result<Vec<Op>, ()> {
    assert_ne!(lower, upper);
    let mut v = Vec::new();
    for ch in input.chars() {
        let op = match ch {
            x if x == lower => Op::Lower,
            x if x == upper => Op::Upper,
            _ => return Err(()),
        };
        v.push(op);
    }
    Ok(v)
}

fn str_to_seat_id(line: &str) -> u32 {
    assert_eq!(line.len(), 10);

    let ops = parse_input(&line[0..7], 'F', 'B').unwrap();
    let row = binary_space_partition(&ops);

    let ops = parse_input(&line[7..10], 'L', 'R').unwrap();
    let col = binary_space_partition(&ops);

    row * 8 + col
}

fn part_one(input: &str) -> u32 {
    input.lines().map(str_to_seat_id).max().unwrap()
}

fn part_two(input: &str) -> u32 {
    let mut ids = input.lines().map(str_to_seat_id).collect::<Vec<_>>();
    ids.sort_unstable();

    let mut iter = ids.iter();
    iter.next();
    for pair in iter.zip(ids.iter()) {
        let higher = pair.0;
        let lower = pair.1;
        if higher - lower != 1 {
            return lower + 1;
        }
    }

    panic!("failed to find expected gap between seat ids");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_space_partition() {
        assert_eq!(
            binary_space_partition(&vec![Op::Upper, Op::Lower, Op::Upper]),
            5
        );
        assert_eq!(
            binary_space_partition(&vec![
                Op::Lower,
                Op::Upper,
                Op::Lower,
                Op::Upper,
                Op::Upper,
                Op::Lower,
                Op::Lower
            ]),
            44
        );
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input("ab", 'a', 'b').unwrap(),
            vec![Op::Lower, Op::Upper]
        );
    }
}
