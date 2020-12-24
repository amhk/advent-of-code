use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");

    let answer = part_one(&input).expect("no solution for part one");
    println!("part 1: {}", answer);

    let answer = part_two(&input, 100).expect("no solution for part two");
    println!("part 2: {}", answer);
}

#[derive(Debug, PartialEq)]
enum Error {
    BadInput,
}

#[derive(Debug, PartialEq)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

#[derive(Debug, PartialEq)]
enum Color {
    White,
    Black,
}

type TileId = (i32, i32);

fn tokenize(input: &str) -> Result<Vec<Vec<Direction>>, Error> {
    let mut outer = Vec::new();
    for mut line in input.lines() {
        let mut inner = Vec::new();
        while !line.is_empty() {
            let mut chars = line.chars();
            let ch0 = chars.next();
            let ch1 = chars.next();

            if let Some('e') = ch0 {
                inner.push(Direction::East);
                line = &line[1..];
                continue;
            }
            if let Some('w') = ch0 {
                inner.push(Direction::West);
                line = &line[1..];
                continue;
            }
            if let Some('s') = ch0 {
                if let Some('e') = ch1 {
                    inner.push(Direction::SouthEast);
                    line = &line[2..];
                    continue;
                }
                if let Some('w') = ch1 {
                    inner.push(Direction::SouthWest);
                    line = &line[2..];
                    continue;
                }
            }
            if let Some('n') = ch0 {
                if let Some('e') = ch1 {
                    inner.push(Direction::NorthEast);
                    line = &line[2..];
                    continue;
                }
                if let Some('w') = ch1 {
                    inner.push(Direction::NorthWest);
                    line = &line[2..];
                    continue;
                }
            }
            return Err(Error::BadInput);
        }
        outer.push(inner);
    }
    Ok(outer)
}

fn setup(directions: &[Vec<Direction>]) -> HashMap<TileId, Color> {
    let mut tiles: HashMap<TileId, Color> = HashMap::new();

    for dirs in directions {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        for d in dirs {
            match d {
                Direction::East => {
                    x += 2;
                }
                Direction::SouthEast => {
                    x += 1;
                    y -= 1;
                }
                Direction::SouthWest => {
                    x -= 1;
                    y -= 1;
                }
                Direction::West => {
                    x -= 2;
                }
                Direction::NorthWest => {
                    x -= 1;
                    y += 1;
                }
                Direction::NorthEast => {
                    x += 1;
                    y += 1;
                }
            }
        }
        let c = tiles.entry((x, y)).or_insert(Color::White);
        match c {
            Color::White => {
                *c = Color::Black;
            }
            Color::Black => {
                *c = Color::White;
            }
        }
    }
    tiles
}

fn part_one(input: &str) -> Result<usize, Error> {
    let all_dirs = tokenize(input)?;
    let tiles = setup(&all_dirs);
    Ok(tiles.values().filter(|&c| *c == Color::Black).count())
}

fn part_two(input: &str, days: usize) -> Result<usize, Error> {
    fn neighbours(id: &TileId) -> [TileId; 6] {
        [
            (id.0 + 2, id.1),     // East
            (id.0 + 1, id.1 + 1), // SouthEast
            (id.0 - 1, id.1 + 1), // SouthWest
            (id.0 - 2, id.1),     // West
            (id.0 - 1, id.1 - 1), // NorthWest
            (id.0 + 1, id.1 - 1), // NorthEast
        ]
    }

    let all_dirs = tokenize(input)?;
    let mut tiles = setup(&all_dirs);

    for _ in 0..days {
        let mut ids: HashSet<TileId> = HashSet::new();
        for id in tiles.keys() {
            ids.insert(*id);
            neighbours(id).iter().for_each(|id| {
                ids.insert(*id);
            });
        }

        let mut next_gen: HashMap<TileId, Color> = HashMap::new();
        for id in ids {
            let colors: Vec<_> = neighbours(&id)
                .iter()
                .map(|id| tiles.get(id).unwrap_or(&Color::White))
                .collect();
            let count = colors.into_iter().filter(|&c| *c == Color::Black).count();
            match tiles.get(&id).unwrap_or(&Color::White) {
                Color::White => {
                    if count == 2 {
                        next_gen.insert(id, Color::Black);
                    }
                }
                Color::Black => {
                    if count == 1 || count == 2 {
                        next_gen.insert(id, Color::Black);
                    }
                }
            }
        }

        tiles = next_gen;
    }

    Ok(tiles.values().filter(|&c| *c == Color::Black).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_tokenize() {
        assert_eq!(
            tokenize("eseswwnwne"),
            Ok(vec![vec![
                Direction::East,
                Direction::SouthEast,
                Direction::SouthWest,
                Direction::West,
                Direction::NorthWest,
                Direction::NorthEast
            ]])
        );
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), Ok(10));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT, 1), Ok(15));
        assert_eq!(part_two(INPUT, 2), Ok(12));
        assert_eq!(part_two(INPUT, 3), Ok(25));
        assert_eq!(part_two(INPUT, 4), Ok(14));
        assert_eq!(part_two(INPUT, 5), Ok(23));
        assert_eq!(part_two(INPUT, 6), Ok(28));
        assert_eq!(part_two(INPUT, 7), Ok(41));
        assert_eq!(part_two(INPUT, 8), Ok(37));
        assert_eq!(part_two(INPUT, 9), Ok(49));
        assert_eq!(part_two(INPUT, 10), Ok(37));
        assert_eq!(part_two(INPUT, 20), Ok(132));
        assert_eq!(part_two(INPUT, 30), Ok(259));
        assert_eq!(part_two(INPUT, 40), Ok(406));
        assert_eq!(part_two(INPUT, 50), Ok(566));
        assert_eq!(part_two(INPUT, 60), Ok(788));
        assert_eq!(part_two(INPUT, 70), Ok(1106));
        assert_eq!(part_two(INPUT, 80), Ok(1373));
        assert_eq!(part_two(INPUT, 90), Ok(1844));
        assert_eq!(part_two(INPUT, 100), Ok(2208));
    }
}
