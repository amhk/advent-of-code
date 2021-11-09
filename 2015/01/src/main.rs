fn main() {
    let input = include_str!("input.txt");

    let answer = part_one(input).expect("no solution for part one");
    println!("part 1: {}", answer);

    let answer = part_two(input).expect("no solution for part two");
    println!("part 2: {}", answer);
}

fn part_one(input: &str) -> Result<i32, ()> {
    Ok(input
        .chars()
        .map(|ch| match ch {
            '(' => 1,
            ')' => -1,
            _ => panic!("unexpected input"),
        })
        .sum())
}

fn part_two(input: &str) -> Result<usize, ()> {
    input
        .chars()
        .enumerate()
        .scan(0, |floor, (i, ch)| {
            if *floor < 0 {
                return None;
            }
            match ch {
                '(' => *floor += 1,
                ')' => *floor -= 1,
                _ => panic!("unexpected input"),
            }
            Some(i + 1)
        })
        .last()
        .ok_or(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("(())"), Ok(0));
        assert_eq!(part_one("()()"), Ok(0));
        assert_eq!(part_one("((("), Ok(3));
        assert_eq!(part_one("(()(()("), Ok(3));
        assert_eq!(part_one("))((((("), Ok(3));
        assert_eq!(part_one("())"), Ok(-1));
        assert_eq!(part_one("))("), Ok(-1));
        assert_eq!(part_one(")))"), Ok(-3));
        assert_eq!(part_one(")())())"), Ok(-3));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(")"), Ok(1));
        assert_eq!(part_two("()())"), Ok(5));
    }
}
