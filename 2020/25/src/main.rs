fn main() {
    let input = include_str!("input.txt");

    let answer = part_one(&input).expect("no solution for part one");
    println!("part 1: {}", answer);
}

#[derive(Debug, PartialEq)]
enum Error {
    BadInput,
}

const MAGIC_NUMBER: u64 = 20201227;

fn loop_size_from_public_key(public_key: u64, subject_number: u64) -> u64 {
    let mut value = 1;
    let mut loop_size = 0;
    loop {
        loop_size += 1;
        value *= subject_number;
        value %= MAGIC_NUMBER;
        if value == public_key {
            return loop_size;
        }
    }
}

fn encryption_key(public_key: u64, loop_size: u64) -> u64 {
    let mut value = 1;
    for _ in 0..loop_size {
        value *= public_key;
        value %= MAGIC_NUMBER;
    }
    value
}

fn part_one(input: &str) -> Result<u64, Error> {
    let mut lines = input.lines();
    let door_pub_key = lines
        .next()
        .ok_or(Error::BadInput)?
        .parse::<u64>()
        .map_err(|_| Error::BadInput)?;
    let card_pub_key = lines
        .next()
        .ok_or(Error::BadInput)?
        .parse::<u64>()
        .map_err(|_| Error::BadInput)?;
    let door_loop_size = loop_size_from_public_key(door_pub_key, 7);
    let card_loop_size = loop_size_from_public_key(card_pub_key, 7);
    let enc_key_alt1 = encryption_key(door_pub_key, card_loop_size);
    let enc_key_alt2 = encryption_key(card_pub_key, door_loop_size);
    assert_eq!(enc_key_alt1, enc_key_alt2);
    Ok(enc_key_alt1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_loop_size_from_public_key() {
        assert_eq!(loop_size_from_public_key(5764801, 7), 8);
        assert_eq!(loop_size_from_public_key(17807724, 7), 11);
    }

    #[test]
    fn test_encryption_key() {
        assert_eq!(encryption_key(5764801, 11), 14897079);
        assert_eq!(encryption_key(17807724, 8), 14897079);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), Ok(14897079));
    }
}
