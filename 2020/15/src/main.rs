use rustc_hash::FxHashMap;

fn main() {
    let input = "13,0,10,12,1,5,8";

    let answer = part_one(input, 2020);
    println!("part 1: {}", answer);

    let answer = part_one(input, 30_000_000);
    println!("part 2: {}", answer);
}

fn part_one(input: &str, stop_at: usize) -> usize {
    assert!(stop_at > 0);
    let input: Vec<usize> = input
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    if stop_at <= input.len() {
        return input[stop_at - 1];
    }

    let mut map: FxHashMap<usize, (usize, Option<usize>)> = FxHashMap::default();
    for (turn, value) in input.iter().enumerate() {
        map.insert(*value, (turn + 1, None));
    }
    let mut last_spoken = *input.last().unwrap();

    for turn in (input.len() + 1)..=stop_at {
        last_spoken = match map.get(&last_spoken) {
            Some((_, None)) => {
                let a = map.get(&0).map(|pair| pair.0);
                map.insert(0, (turn, a));
                0
            }
            Some((a, Some(b))) => {
                let speak_this = *a - *b;
                let a = map.get(&speak_this).map(|pair| pair.0);
                map.insert(speak_this, (turn, a));
                speak_this
            }
            None => {
                unreachable!();
            }
        }
    }

    last_spoken
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("0,3,6", 1), 0);
        assert_eq!(part_one("0,3,6", 2), 3);
        assert_eq!(part_one("0,3,6", 3), 6);
        assert_eq!(part_one("0,3,6", 4), 0);
        assert_eq!(part_one("0,3,6", 5), 3);
        assert_eq!(part_one("0,3,6", 6), 3);
        assert_eq!(part_one("0,3,6", 7), 1);
        assert_eq!(part_one("0,3,6", 8), 0);
        assert_eq!(part_one("0,3,6", 9), 4);
        assert_eq!(part_one("0,3,6", 10), 0);
        assert_eq!(part_one("0,3,6", 2020), 436);
        assert_eq!(part_one("1,3,2", 2020), 1);
        assert_eq!(part_one("2,1,3", 2020), 10);
        assert_eq!(part_one("1,2,3", 2020), 27);
        assert_eq!(part_one("2,3,1", 2020), 78);
        assert_eq!(part_one("3,2,1", 2020), 438);
        assert_eq!(part_one("3,1,2", 2020), 1836);
    }
}
