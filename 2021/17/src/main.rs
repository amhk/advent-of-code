use regex::{Captures, Regex};

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
    NoHitFound,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

struct Projectile {
    position: Point,
    velocity: Point,
    apex: Option<Point>,
}

impl Projectile {
    fn new(initial_velocity: Point) -> Projectile {
        Projectile {
            position: Point { x: 0, y: 0 },
            velocity: initial_velocity,
            apex: None,
        }
    }

    fn advance(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;

        match self.velocity.x {
            x if x > 0 => {
                self.velocity.x -= 1;
            }
            x if x < 0 => {
                self.velocity.x += 1;
            }
            _ => {}
        }

        self.velocity.y -= 1;

        if self.velocity.y == 0 {
            self.apex = Some(Point::new(self.position.x, self.position.y));
        }
    }

    fn is_inside(&self, bb: (Point, Point)) -> bool {
        debug_assert!(bb.0.x <= bb.1.x);
        debug_assert!(bb.0.y <= bb.1.y);
        self.position.x >= bb.0.x
            && self.position.x <= bb.1.x
            && self.position.y >= bb.0.y
            && self.position.y <= bb.1.y
    }
}

#[derive(Debug, PartialEq)]
enum HitOrMiss {
    Hit { apex: Point },
    Miss,
}

fn shoot(target: (Point, Point), initial_velocity: Point) -> HitOrMiss {
    let mut p = Projectile::new(initial_velocity);
    loop {
        if p.is_inside(target) {
            return HitOrMiss::Hit {
                apex: p.apex.unwrap_or(initial_velocity),
            };
        }
        if p.position.x > target.1.x || p.position.y < target.0.y {
            return HitOrMiss::Miss;
        }
        p.advance();
    }
}

fn parse_bounding_box(input: &str) -> Result<(Point, Point), Error> {
    fn parse_i32(caps: &Captures, index: usize) -> Result<i32, Error> {
        caps.get(index)
            .ok_or(Error::BadInput)?
            .as_str()
            .parse()
            .map_err(|_| Error::BadInput)
    }

    let re = Regex::new(r"^target area: x=(-?\d+)\.\.(-?\d+), y=(-?\d+)\.\.(-?\d+)$").unwrap();
    if let Some(caps) = re.captures(input.trim()) {
        let x1 = parse_i32(&caps, 1)?;
        let x2 = parse_i32(&caps, 2)?;
        let y1 = parse_i32(&caps, 3)?;
        let y2 = parse_i32(&caps, 4)?;
        Ok((
            Point::new(x1.min(x2), y1.min(y2)),
            Point::new(x1.max(x2), y1.max(y2)),
        ))
    } else {
        Err(Error::BadInput)
    }
}

fn shooting_range(input: &str) -> Result<Vec<(Point, Point)>, Error> {
    const ASSUMED_MAX_Y: i32 = 750;
    let target = parse_bounding_box(input)?;
    let mut hits = vec![];
    for x in 0..=target.1.x {
        for y in target.0.y..=ASSUMED_MAX_Y {
            if let HitOrMiss::Hit { apex } = shoot(target, Point::new(x, y)) {
                hits.push((Point::new(x, y), apex));
            }
        }
    }
    Ok(hits)
}

fn part_one(input: &str) -> Result<i32, Error> {
    let hits = shooting_range(input)?;
    hits.iter()
        .map(|(_, apex)| apex.y)
        .max()
        .ok_or(Error::NoHitFound)
}

fn part_two(input: &str) -> Result<usize, Error> {
    Ok(shooting_range(input)?.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_parse_bounding_box() {
        let bb = parse_bounding_box(INPUT).unwrap();
        assert_eq!(bb.0.x, 20);
        assert_eq!(bb.0.y, -10);
        assert_eq!(bb.1.x, 30);
        assert_eq!(bb.1.y, -5);
    }

    #[test]
    fn test_projectile() {
        let mut p = Projectile::new(Point { x: 7, y: 2 });
        assert_eq!(p.position, Point::new(0, 0));
        p.advance();
        assert_eq!(p.position, Point::new(7, 2));
        p.advance();
        assert_eq!(p.position, Point::new(13, 3));
        p.advance();
        assert_eq!(p.position, Point::new(18, 3));
        p.advance();
        assert_eq!(p.position, Point::new(22, 2));
        p.advance();
        assert_eq!(p.position, Point::new(25, 0));
        p.advance();
        assert_eq!(p.position, Point::new(27, -3));
        p.advance();
        assert_eq!(p.position, Point::new(28, -7));
    }

    #[test]
    fn test_shoot() {
        let target = parse_bounding_box(INPUT).unwrap();
        assert_eq!(
            shoot(target, Point::new(7, 2)),
            HitOrMiss::Hit {
                apex: Point::new(13, 3)
            }
        );
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), Ok(45));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), Ok(112));
    }
}
