fn main() {
    let input = include_str!("input.txt");

    let answer = part_one(&input);
    println!("part 1: {}", answer);

    let answer = part_two(&input);
    println!("part 2: {}", answer);
}

fn str_to_seat_id(line: &str) -> u32 {
    let line = line
        .replace('F', "0")
        .replace('B', "1")
        .replace('L', "0")
        .replace('R', "1");
    u32::from_str_radix(&line, 2).unwrap()
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
    fn test_str_to_seat_id() {
        assert_eq!(str_to_seat_id("BFFFBBFRRR"), 567);
        assert_eq!(str_to_seat_id("FFFBBBFRRR"), 119);
        assert_eq!(str_to_seat_id("BBFFBBFRLL"), 820);
    }
}
