use std::collections::BTreeMap;

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

#[derive(Debug, PartialEq)]
enum EnergyLevel {
    Charging(u32),
    Flashing,
}

type Grid = BTreeMap<(i32, i32), EnergyLevel>;

fn parse_input(input: &str) -> Result<Grid, Error> {
    let mut grid = Grid::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            grid.insert(
                (x.try_into().unwrap(), y.try_into().unwrap()),
                EnergyLevel::Charging(ch.to_digit(10).ok_or(Error::BadInput)?),
            );
        }
    }
    Ok(grid)
}

fn advance(grid: &mut Grid) {
    grid.iter_mut().for_each(|(_, value)| {
        if let EnergyLevel::Charging(i) = value {
            *value = EnergyLevel::Charging(*i + 1);
        }
    });

    loop {
        let about_to_flash: Vec<_> = grid
            .iter()
            .filter(|(_, value)| {
                if let EnergyLevel::Charging(i) = value {
                    *i >= 10
                } else {
                    false
                }
            })
            .map(|(key, _)| *key)
            .collect();
        if about_to_flash.is_empty() {
            break;
        }
        for key in about_to_flash {
            let value = grid.get_mut(&key).unwrap();
            *value = EnergyLevel::Flashing;

            for neighbour_key in [
                (key.0 - 1, key.1 - 1),
                (key.0, key.1 - 1),
                (key.0 + 1, key.1 - 1),
                (key.0 - 1, key.1),
                (key.0 + 1, key.1),
                (key.0 - 1, key.1 + 1),
                (key.0, key.1 + 1),
                (key.0 + 1, key.1 + 1),
            ] {
                let neighbour = grid.get_mut(&neighbour_key);
                if let Some(EnergyLevel::Charging(i)) = neighbour {
                    *neighbour.unwrap() = EnergyLevel::Charging(*i + 1)
                }
            }
        }
    }
}

fn count_flashing(grid: &Grid) -> usize {
    grid.iter()
        .filter(|(_, value)| **value == EnergyLevel::Flashing)
        .count()
}

fn clear_flashing(grid: &mut Grid) {
    grid.iter_mut()
        .filter(|(_, value)| **value == EnergyLevel::Flashing)
        .for_each(|(_, value)| *value = EnergyLevel::Charging(0));
}

fn part_one(input: &str) -> Result<usize, Error> {
    let mut grid = parse_input(input)?;
    let mut sum = 0;
    for _ in 0..100 {
        advance(&mut grid);
        sum += count_flashing(&grid);
        clear_flashing(&mut grid);
    }
    Ok(sum)
}

fn part_two(input: &str) -> Result<usize, Error> {
    let mut grid = parse_input(input)?;
    let size = grid.keys().count();
    let mut step = 0;
    loop {
        step += 1;
        advance(&mut grid);
        if count_flashing(&grid) == size {
            return Ok(step);
        }
        clear_flashing(&mut grid);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = include_str!("test-input1.txt");
    const INPUT2: &str = include_str!("test-input2.txt");
    const INPUT3: &str = include_str!("test-input3.txt");

    #[test]
    fn test_grid() {
        let mut grid = parse_input(INPUT1).unwrap();

        advance(&mut grid);
        assert_eq!(count_flashing(&grid), 0);
        clear_flashing(&mut grid);
        assert_eq!(grid, parse_input(INPUT2).unwrap());

        advance(&mut grid);
        assert_eq!(count_flashing(&grid), 35);
        clear_flashing(&mut grid);
        assert_eq!(grid, parse_input(INPUT3).unwrap());
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT1), Ok(1656));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT1), Ok(195));
    }
}
