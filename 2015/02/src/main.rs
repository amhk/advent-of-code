fn main() {
    let input = include_str!("input.txt");

    let answer = part_one(input).expect("no solution for part one");
    println!("part 1: {}", answer);

    let answer = part_two(input).expect("no solution for part two");
    println!("part 2: {}", answer);
}

fn part_one(input: &str) -> Result<usize, ()> {
    Ok(input
        .lines()
        .map(|line| {
            let sides: Vec<usize> = line
                .split('x')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            assert_eq!(sides.len(), 3);
            let a = sides[0] * sides[1];
            let b = sides[0] * sides[2];
            let c = sides[1] * sides[2];
            let min = usize::min(a, usize::min(b, c));
            2 * a + 2 * b + 2 * c + min
        })
        .sum())
}

fn part_two(input: &str) -> Result<usize, ()> {
    Ok(input
        .lines()
        .map(|line| {
            let mut sides: Vec<usize> = line
                .split('x')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            assert_eq!(sides.len(), 3);
            sides.sort_unstable();
            2 * sides[0] + 2 * sides[1] + sides[0] * sides[1] * sides[2]
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("2x3x4"), Ok(58));
        assert_eq!(part_one("1x1x10"), Ok(43));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("2x3x4"), Ok(34));
        assert_eq!(part_two("1x1x10"), Ok(14));
    }
}
