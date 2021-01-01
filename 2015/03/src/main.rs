use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");

    let answer = part_one(&input).expect("no solution for part one");
    println!("part 1: {}", answer);

    let answer = part_two(&input).expect("no solution for part two");
    println!("part 2: {}", answer);
}

fn deliver_presents(directions: impl Iterator<Item = char>) -> HashSet<(i32, i32)> {
    let mut set = HashSet::new();
    let mut x = 0;
    let mut y = 0;
    set.insert((0, 0));
    directions.for_each(|ch| {
        match ch {
            '<' => x -= 1,
            '>' => x += 1,
            '^' => y -= 1,
            'v' => y += 1,
            _ => panic!("unexpected input"),
        }
        set.insert((x, y));
    });
    set
}

fn part_one(input: &str) -> Result<usize, ()> {
    Ok(deliver_presents(input.chars()).len())
}

fn part_two(input: &str) -> Result<usize, ()> {
    let santa = deliver_presents(input.chars().step_by(2));
    let robo_santa = deliver_presents(input.chars().skip(1).step_by(2));
    Ok((&santa | &robo_santa).len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(">"), Ok(2));
        assert_eq!(part_one("^>v<"), Ok(4));
        assert_eq!(part_one("^v^v^v^v^v"), Ok(2));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("^v"), Ok(3));
        assert_eq!(part_two("^>v<"), Ok(3));
        assert_eq!(part_two("^v^v^v^v^v"), Ok(11));
    }
}
