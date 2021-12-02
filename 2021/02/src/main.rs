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

enum Operation {
    Forward(i32),
    Up(i32),
    Down(i32),
}

fn parse_input(input: &str) -> Result<Vec<Operation>, Error> {
    let mut v = Vec::new();
    for line in input.lines() {
        if let Some((keyword, arg)) = line.split_once(' ') {
            let arg = arg.parse::<i32>().map_err(|_| Error::BadInput)?;
            match keyword {
                "forward" => v.push(Operation::Forward(arg)),
                "up" => v.push(Operation::Up(arg)),
                "down" => v.push(Operation::Down(arg)),
                _ => return Err(Error::BadInput),
            }
        } else {
            return Err(Error::BadInput);
        }
    }
    Ok(v)
}

fn part_one(input: &str) -> Result<i32, Error> {
    let ops = parse_input(input)?;
    let (f, u, d) = ops.iter().fold((0, 0, 0), |acc, op| match op {
        Operation::Forward(arg) => (acc.0 + arg, acc.1, acc.2),
        Operation::Up(arg) => (acc.0, acc.1 + arg, acc.2),
        Operation::Down(arg) => (acc.0, acc.1, acc.2 + arg),
    });
    debug_assert!(u <= d);
    Ok(f * (d - u))
}

fn part_two(input: &str) -> Result<i32, Error> {
    let mut aim = 0;
    let mut horizontal = 0;
    let mut depth = 0;
    for op in parse_input(input)?.iter() {
        match op {
            Operation::Forward(arg) => {
                horizontal += arg;
                depth += aim * arg;
            }
            Operation::Up(arg) => aim -= arg,
            Operation::Down(arg) => aim += arg,
        };
    }
    debug_assert!(depth >= 0);
    Ok(horizontal * depth)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), Ok(150));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), Ok(900));
    }
}
