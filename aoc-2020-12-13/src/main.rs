fn main() {
    let input = include_str!("input.txt");

    let answer = part_one(&input).expect("no solution for part one");
    println!("part 1: {}", answer);

    let answer = part_two(&input).expect("no solution for part two");
    println!("part 2: {}", answer);
}

#[derive(Debug, PartialEq)]
enum Error {
    BadInput(&'static str),
}

fn part_one(input: &str) -> Result<usize, Error> {
    let timestamp = input
        .lines()
        .next()
        .ok_or(Error::BadInput("missing first line"))?
        .parse::<usize>()
        .map_err(|_| Error::BadInput("first line not a number"))?;
    let mut busses = Vec::new();
    for x in input
        .lines()
        .nth(1)
        .ok_or(Error::BadInput("missing second line"))?
        .split(',')
    {
        match x {
            "x" => {}
            _ => {
                let id = x.parse::<usize>().map_err(|_| Error::BadInput("bad id"))?;
                let rem = (id - (timestamp % id)) % id;
                busses.push((id, rem));
            }
        }
    }
    let bus = busses
        .iter()
        .min_by_key(|x| x.1)
        .ok_or(Error::BadInput("no busses"))?;
    Ok(bus.0 * bus.1)
}

fn part_two(input: &str) -> Result<u64, Error> {
    let mut gears = Vec::new();
    for (offset, id) in input
        .lines()
        .nth(1)
        .ok_or(Error::BadInput("missing second line"))?
        .split(',')
        .enumerate()
    {
        match id {
            "x" => {}
            _ => {
                let period = id.parse::<u64>().map_err(|_| Error::BadInput("bad id"))? as u64;
                gears.push((period, offset as u64));
            }
        }
    }

    fn lcm(n: u64, m: u64) -> u64 {
        // Our input is guaranteed to be only prime numbers, so lcm(n, m) == n * m.
        n * m
    }

    // References: [1] https://en.wikipedia.org/wiki/Least_common_multiple#Gears_problem
    //
    // Think of the buses as gears on the same axis. The gears have different number of teeth (the
    // bus IDs). Each gear has one marked tooth. At timestamp == 0 the gears are offset from each
    // other so that none of the marked teeth are aligned. Find the lowest timestamp when all gears
    // are aligned.
    //
    // According to [1], two gears with m and n teeth re-align after lcm(m, n) rotations.
    let (_, timestamp) =
        gears
            .iter()
            .fold((1, 0), |(period, timestamp), (gear_period, gear_offset)| {
                (
                    // Calculate new period needed to align visited gears.
                    lcm(period, *gear_period),
                    // Find timestamp when visited gears become aligned with current gear.
                    // Start counting from current timestamp.
                    (0..)
                        .map(|i| period * i + timestamp)
                        .find(|i| (i + gear_offset) % gear_period == 0)
                        .unwrap(),
                )
            });

    Ok(timestamp)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), Ok(5 * 59));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("0\n5,x,7\n"), Ok(5));
        assert_eq!(part_two("0\n5,7\n"), Ok(20));
        assert_eq!(part_two("0\n5,x,7,13\n"), Ok(75));
        assert_eq!(part_two("0\n17,x,13,19\n"), Ok(3417));
        assert_eq!(part_two("0\n67,7,59,61\n"), Ok(754018));
        assert_eq!(part_two("0\n67,x,7,59,61\n"), Ok(779210));
        assert_eq!(part_two("0\n67,7,x,59,61\n"), Ok(1261476));
        assert_eq!(part_two("0\n1789,37,47,1889\n"), Ok(1202161486));
        assert_eq!(part_two(INPUT), Ok(1068781));
    }
}
