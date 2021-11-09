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

type Id = usize;

fn parse_input(input: &str) -> Result<Vec<Id>, Error> {
    Ok(input
        .trim()
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as Id)
        .collect())
}

// play the game in O(n) time, where n is the number of rounds to play:
// all operations on the data are O(1)
fn play_game(initial: &[Id], rounds: usize) -> Vec<Id> {
    // input guarantee: contiguous range of unique values from 1..n
    assert!(initial.len() > 3);

    // `next vector`: which cup follows cup with a given Id, stored at index Id?
    // include a leading 0 to make Id == index
    let mut next: Vec<Id> = vec![0; 1 + initial.len()];
    let mut prev_id = 0;
    for id in initial.iter() {
        next[prev_id] = *id;
        prev_id = *id;
    }
    next[prev_id] = initial[0];
    next[0] = Id::MAX; // invalid

    // `which` vector: which cup has value Id, stored at index Id?
    // include a leading 0 to make value == index
    let mut which: Vec<Id> = vec![0; 1 + initial.len()];
    for (position, id) in initial.iter().enumerate().map(|(pos, id)| (pos + 1, id)) {
        which[*id] = position;
    }
    which[0] = Id::MAX; // invalid

    // head: current cup
    let mut head = initial[0];

    let max = initial.len();

    for _ in 0..rounds {
        // remove three cups
        let a = next[head];
        let b = next[a];
        let c = next[b];
        next[head] = next[c];

        // find where to insert the three cups
        let find_dest_id = || {
            let mut dest_id = if head == 1 { max } else { head - 1 };
            let exclude = [a, b, c];
            while exclude.contains(&dest_id) {
                dest_id = if dest_id == 1 { max } else { dest_id - 1 };
            }
            dest_id
        };
        let dest_id = find_dest_id();

        // insert the three cups
        next[c] = next[dest_id];
        next[dest_id] = a;

        // advance head
        head = next[head];
    }

    let mut v = Vec::with_capacity(initial.len());
    let stop_at = head;
    loop {
        v.push(head);
        head = next[head];
        if head == stop_at {
            return v;
        }
    }
}

fn cups_to_magic_string(cups: &[Id]) -> String {
    assert!(cups.iter().any(|&c| c == 1));
    assert!(cups.len() > 1);

    let mut cups = cups.to_vec();
    while cups[0] != 1 {
        let a = cups.remove(0);
        cups.push(a);
    }

    let mut s = String::new();
    cups.iter().skip(1).for_each(|c| {
        s.push_str(&format!("{}", c));
    });
    s
}

fn part_one(input: &str) -> Result<String, Error> {
    let cups = parse_input(input)?;
    let cups = play_game(&cups, 100);
    Ok(cups_to_magic_string(&cups))
}

fn part_two(input: &str) -> Result<u64, Error> {
    let mut cups = parse_input(input)?;
    let max = *cups.iter().max().ok_or(Error::BadInput)?;
    for i in (max + 1)..=1_000_000 {
        cups.push(i);
    }
    let cups = play_game(&cups, 10_000_000);
    let pos_1 = cups.iter().position(|&c| c == 1).unwrap();
    let pos_a = (pos_1 + 1) % cups.len();
    let pos_b = (pos_1 + 2) % cups.len();
    let a = *cups.get(pos_a).unwrap() as u64;
    let b = *cups.get(pos_b).unwrap() as u64;

    Ok(a * b)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_play_game() {
        let initial = parse_input(INPUT).unwrap();
        assert_eq!(play_game(&initial, 1), vec![2, 8, 9, 1, 5, 4, 6, 7, 3]);
        assert_eq!(play_game(&initial, 2), vec![5, 4, 6, 7, 8, 9, 1, 3, 2]);
        assert_eq!(play_game(&initial, 10), vec![8, 3, 7, 4, 1, 9, 2, 6, 5]);
    }

    #[test]
    fn test_cups_to_magic_string() {
        let cups = vec![8, 3, 7, 4, 1, 9, 2, 6, 5];
        assert_eq!(cups_to_magic_string(&cups), "92658374");
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), "67384529");
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 934001_u64 * 159792_u64);
    }
}
