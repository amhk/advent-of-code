fn main() {
    let input = include_str!("input.txt");

    let answer = part_one(input).expect("no solution for part one");
    println!("part 1: {}", answer);

    let answer = part_two(input).expect("no solution for part part_two");
    println!("part 2: {}", answer);
}

#[derive(Debug)]
enum Error {
    BadInput,
}

fn parse_input(input: &str) -> Vec<u64> {
    let mut values: Vec<u64> = input
        .split_whitespace()
        .map(|s| s.parse::<u64>())
        .filter_map(Result::ok)
        .collect();
    values.sort_unstable();
    values
}

fn part_one(input: &str) -> Result<u64, Error> {
    let values = parse_input(input);

    let mut prev = 0;
    let mut diff_1 = 0;
    let mut diff_3 = 1; // include the jump from last adapter to device

    for jolt in values {
        assert!(prev < jolt);
        match jolt - prev {
            1 => diff_1 += 1,
            2 => {}
            3 => diff_3 += 1,
            _ => return Err(Error::BadInput),
        }
        prev = jolt;
    }

    Ok(diff_1 * diff_3)
}

#[derive(Debug)]
struct Node {
    jolt: u64,
    count: u64,
}

fn try_skip_to_node(node: Option<&Node>, from_jolt: u64) -> Option<&Node> {
    node.and_then(|node| {
        assert!(from_jolt < node.jolt);
        match node.jolt - from_jolt {
            1..=3 => Some(node),
            _ => None,
        }
    })
}

fn part_two(input: &str) -> Result<u64, Error> {
    let mut nodes = vec![Node { jolt: 0, count: 0 }];
    nodes.append(
        &mut parse_input(input)
            .into_iter()
            .map(|jolt| Node { jolt, count: 1 })
            .collect::<Vec<_>>(),
    );

    for i in (0..nodes.len() - 1).rev() {
        let node = nodes.get(i).unwrap();
        let a = try_skip_to_node(nodes.get(i + 1), node.jolt)
            .map(|n| n.count)
            .expect("the immediately following adapter must be compatible");
        let b = try_skip_to_node(nodes.get(i + 2), node.jolt).map_or(0, |n| n.count);
        let c = try_skip_to_node(nodes.get(i + 3), node.jolt).map_or(0, |n| n.count);

        let node = nodes.get_mut(i).unwrap();
        node.count = a + b + c;
    }

    nodes.first().map_or(Err(Error::BadInput), |n| Ok(n.count))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_A: &str = include_str!("test-input-a.txt");
    const INPUT_B: &str = include_str!("test-input-b.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT_A).unwrap(), 7 * 5);
        assert_eq!(part_one(INPUT_B).unwrap(), 22 * 10);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT_A).unwrap(), 8);
        assert_eq!(part_two(INPUT_B).unwrap(), 19208);
    }
}
