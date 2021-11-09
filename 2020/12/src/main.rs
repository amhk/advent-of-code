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
    BadDegrees,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum CardinalDirection {
    N = 0,
    E = 90,
    S = 180,
    W = 270,
}

impl CardinalDirection {
    fn add(self, degrees: i32) -> Result<CardinalDirection, Error> {
        let mut angle = ((self as i32) + degrees) % 360;
        while angle < 0 {
            angle += 360;
        }
        match angle {
            0 => Ok(CardinalDirection::N),
            90 => Ok(CardinalDirection::E),
            180 => Ok(CardinalDirection::S),
            270 => Ok(CardinalDirection::W),
            _ => Err(Error::BadDegrees),
        }
    }

    fn sub(self, degrees: i32) -> Result<CardinalDirection, Error> {
        self.add(-degrees)
    }
}

fn part_one(input: &str) -> Result<i32, Error> {
    let steps = {
        let mut steps = Vec::new();
        let mut compass = CardinalDirection::E;
        for line in input.lines() {
            assert!(line.len() > 1);
            let (action, amp) = line.split_at(1);
            let amp = amp.parse::<i32>().map_err(|_| Error::BadInput)?;
            match action {
                "N" => steps.push((CardinalDirection::N, amp)),
                "E" => steps.push((CardinalDirection::E, amp)),
                "S" => steps.push((CardinalDirection::N, -amp)),
                "W" => steps.push((CardinalDirection::E, -amp)),
                "F" => match compass {
                    CardinalDirection::N => steps.push((CardinalDirection::N, amp)),
                    CardinalDirection::E => steps.push((CardinalDirection::E, amp)),
                    CardinalDirection::S => steps.push((CardinalDirection::N, -amp)),
                    CardinalDirection::W => steps.push((CardinalDirection::E, -amp)),
                },
                "R" => compass = compass.add(amp)?,
                "L" => compass = compass.sub(amp)?,
                _ => return Err(Error::BadInput),
            }
        }
        steps
    };

    let dx: i32 = steps
        .iter()
        .filter(|(dir, _)| dir == &CardinalDirection::E)
        .map(|(_, amp)| amp)
        .sum();

    let dy: i32 = steps
        .iter()
        .filter(|(dir, _)| dir == &CardinalDirection::N)
        .map(|(_, amp)| amp)
        .sum();

    Ok(dx.abs() + dy.abs())
}

fn part_two(input: &str) -> Result<i32, Error> {
    fn rotate90(vector: (i32, i32)) -> (i32, i32) {
        (vector.1, -vector.0)
    }

    let normalized = input.lines().map(|line| match line {
        "L90" => "R270",
        "L180" => "R180",
        "L270" => "R90",
        _ => line,
    });
    let mut ship: (i32, i32) = (0, 0);
    let mut waypoint: (i32, i32) = (10, 1);
    for line in normalized {
        assert!(line.len() > 1);
        let (action, amp) = line.split_at(1);
        let amp = amp.parse::<i32>().map_err(|_| Error::BadInput)?;
        match action {
            "N" => waypoint.1 += amp,
            "E" => waypoint.0 += amp,
            "S" => waypoint.1 -= amp,
            "W" => waypoint.0 -= amp,
            "R" => match amp {
                90 => waypoint = rotate90(waypoint),
                180 => waypoint = rotate90(rotate90(waypoint)),
                270 => waypoint = rotate90(rotate90(rotate90(waypoint))),
                _ => return Err(Error::BadInput),
            },
            "F" => {
                ship.0 += waypoint.0 * amp;
                ship.1 += waypoint.1 * amp;
            }
            _ => return Err(Error::BadInput),
        }
    }

    Ok(ship.0.abs() + ship.1.abs())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_cardinal_direction() {
        let compass = CardinalDirection::N;
        assert_eq!(compass.add(90), Ok(CardinalDirection::E));
        assert_eq!(compass.sub(90), Ok(CardinalDirection::W));

        assert_eq!(compass.add(180), Ok(CardinalDirection::S));
        assert_eq!(compass.sub(180), Ok(CardinalDirection::S));

        assert_eq!(compass.add(270), Ok(CardinalDirection::W));
        assert_eq!(compass.sub(270), Ok(CardinalDirection::E));

        assert_eq!(compass.add(0), Ok(CardinalDirection::N));
        assert_eq!(compass.add(90 * 4), Ok(CardinalDirection::N));

        assert_eq!(compass.sub(0), Ok(CardinalDirection::N));
        assert_eq!(compass.sub(90 * 4), Ok(CardinalDirection::N));

        assert_eq!(compass.add(1), Err(Error::BadDegrees));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), Ok(17 + 8));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), Ok(214 + 72));
    }
}
