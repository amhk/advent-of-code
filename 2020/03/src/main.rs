fn main() {
    let input = include_str!("input.txt");

    let answer = part_one(input);
    println!("part 1: {}", answer);

    let answer = part_two(input);
    println!("part 2: {}", answer);
}

fn count_trees_in_ride_toboggan(input: &str, speed_right: usize, speed_down: usize) -> usize {
    assert_eq!(input.chars().next().unwrap(), '.');
    let line_length = input.lines().next().unwrap().len();
    let mut pos = 0;
    let mut count = 0;
    for line in input.lines().step_by(speed_down) {
        assert_eq!(line.len(), line_length);
        if line.chars().nth(pos) == Some('#') {
            count += 1;
        }
        pos = (pos + speed_right) % line_length;
    }
    count
}

fn part_one(input: &str) -> usize {
    count_trees_in_ride_toboggan(input, 3, 1)
}

fn part_two(input: &str) -> usize {
    count_trees_in_ride_toboggan(input, 1, 1)
        * count_trees_in_ride_toboggan(input, 3, 1)
        * count_trees_in_ride_toboggan(input, 5, 1)
        * count_trees_in_ride_toboggan(input, 7, 1)
        * count_trees_in_ride_toboggan(input, 1, 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 7);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(count_trees_in_ride_toboggan(INPUT, 1, 1), 2);
        assert_eq!(count_trees_in_ride_toboggan(INPUT, 3, 1), 7);
        assert_eq!(count_trees_in_ride_toboggan(INPUT, 5, 1), 3);
        assert_eq!(count_trees_in_ride_toboggan(INPUT, 7, 1), 4);
        assert_eq!(count_trees_in_ride_toboggan(INPUT, 1, 2), 2);
        assert_eq!(part_two(INPUT), 336);
    }
}
