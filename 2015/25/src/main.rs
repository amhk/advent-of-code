fn main() {
    let answer = part_one();
    println!("part 1: {}", answer);
}

struct CoordinatesIterator {
    x: u32,
    y: u32,
    next_y: u32,
}

impl CoordinatesIterator {
    fn new() -> CoordinatesIterator {
        CoordinatesIterator {
            x: 1,
            y: 1,
            next_y: 2,
        }
    }
}

impl Iterator for CoordinatesIterator {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        let coords = (self.x, self.y);
        if self.y > 1 {
            self.x += 1;
            self.y -= 1;
        } else {
            self.x = 1;
            self.y = self.next_y;
            self.next_y += 1;
        }
        Some(coords)
    }
}

fn next_code(code: u64) -> u64 {
    (code * 252533) % 33554393
}

fn part_one() -> u64 {
    let mut code = 20151125;
    let mut iter = CoordinatesIterator::new();
    while iter.next() != Some((3019, 3010)) {
        code = next_code(code);
    }
    code
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinates_iterator() {
        let mut iter = CoordinatesIterator::new();
        assert_eq!(iter.next(), Some((1, 1)));
        assert_eq!(iter.next(), Some((1, 2)));
        assert_eq!(iter.next(), Some((2, 1)));
        assert_eq!(iter.next(), Some((1, 3)));
        assert_eq!(iter.next(), Some((2, 2)));
        assert_eq!(iter.next(), Some((3, 1)));
        assert_eq!(iter.next(), Some((1, 4)));
    }
}
