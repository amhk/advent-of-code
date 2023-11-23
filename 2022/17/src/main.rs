use anyhow::{bail, Result};
use std::collections::HashMap;

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 3133)?;
    aoc::run!(part_two(input), 1547953216393)?;
    Ok(())
}

struct Well {
    // #### .... ...# ####
    rows: Vec<u16>,
}

impl Well {
    fn new() -> Self {
        Self {
            rows: vec![0b1111_1111_1111_1111],
        }
    }

    fn extend_to(&mut self, new_height: usize) {
        while self.rows.len() <= new_height {
            self.rows.push(0b1111_0000_0001_1111);
        }
    }

    fn add(&mut self, block: &Block, x: usize, y: usize) {
        debug_assert!(y + 4 <= self.rows.len());
        debug_assert!(x <= 6);

        for i in 0..4 {
            let a = block.get(i, x);
            let b = self.rows.get(y + i).unwrap();
            *self.rows.get_mut(y + i).unwrap() = a | b;
        }
    }

    fn can_move_left(&self, block: &Block, x: usize, y: usize) -> bool {
        x != 0 && self.will_block_fit(block, x - 1, y)
    }

    fn can_move_right(&self, block: &Block, x: usize, y: usize) -> bool {
        x < 6 && self.will_block_fit(block, x + 1, y)
    }

    fn can_move_down(&self, block: &Block, x: usize, y: usize) -> bool {
        self.will_block_fit(block, x, y - 1)
    }

    fn will_block_fit(&self, block: &Block, x: usize, y: usize) -> bool {
        for i in 0..4 {
            if (self.rows.get(y + i).unwrap() & block.get(i, x)) != 0 {
                return false;
            }
        }
        true
    }

    fn get_fingerprint(&self, y: usize) -> (u128, u128, u128, u128) {
        let mut a = 0u128;
        for i in 0..4 {
            if i <= y {
                a |= (*self.rows.get(y - i).unwrap() as u128) << (i * 16);
            }
        }
        let mut b = 0u128;
        for i in 0..4 {
            if i <= y {
                b |= (*self.rows.get(y - i - 4).unwrap() as u128) << (i * 16);
            }
        }
        let mut c = 0u128;
        for i in 0..4 {
            if i <= y {
                c |= (*self.rows.get(y - i - 8).unwrap() as u128) << (i * 16);
            }
        }
        let mut d = 0u128;
        for i in 0..4 {
            if i <= y {
                d |= (*self.rows.get(y - i - 12).unwrap() as u128) << (i * 16);
            }
        }
        (a, b, c, d)
    }
}

impl std::fmt::Debug for Well {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for (i, row) in self.rows.iter().enumerate().rev() {
            if i == 0 {
                s.push_str("   0 +-------+");
            } else {
                s.push_str(&format!("{:4} |", i));
                for i in (5..12).rev() {
                    if (row >> i) & 0x1 == 0x1 {
                        s.push('#');
                    } else {
                        s.push('.');
                    }
                }
                s.push('|');
                s.push('\n');
            }
        }
        write!(f, "{}", s)
    }
}

#[derive(Clone)]
struct Block {
    // 3: .... .... .... ....
    // 2: .... .#.. .... ....
    // 1: .... ###. .... ....
    // 0: .... .#.. .... ....
    // index:  0123 456
    pattern: [u16; 4],
    height: usize,
}

impl Block {
    fn get(&self, index: usize, x: usize) -> u16 {
        debug_assert!(x <= 6);
        self.pattern[index] >> x
    }
}

struct BlockGenerator {
    blocks: Vec<Block>,
}

impl BlockGenerator {
    fn new() -> Self {
        let mut blocks = Vec::new();

        // ####
        let mut pattern = [
            0b_0000_0000_0000_0000,
            0b_0000_0000_0000_0000,
            0b_0000_0000_0000_0000,
            0b_0000_1111_0000_0000,
        ];
        pattern.reverse();
        blocks.push(Block { pattern, height: 1 });

        // .#.
        // ###
        // .#.
        let mut pattern = [
            0b_0000_0000_0000_0000,
            0b_0000_0100_0000_0000,
            0b_0000_1110_0000_0000,
            0b_0000_0100_0000_0000,
        ];
        pattern.reverse();
        blocks.push(Block { pattern, height: 3 });

        // ..#
        // ..#
        // ###
        let mut pattern = [
            0b_0000_0000_0000_0000,
            0b_0000_0010_0000_0000,
            0b_0000_0010_0000_0000,
            0b_0000_1110_0000_0000,
        ];
        pattern.reverse();
        blocks.push(Block { pattern, height: 3 });

        // #
        // #
        // #
        // #
        let mut pattern = [
            0b_0000_1000_0000_0000,
            0b_0000_1000_0000_0000,
            0b_0000_1000_0000_0000,
            0b_0000_1000_0000_0000,
        ];
        pattern.reverse();
        blocks.push(Block { pattern, height: 4 });

        // ##
        // ##
        let mut pattern = [
            0b_0000_0000_0000_0000,
            0b_0000_0000_0000_0000,
            0b_0000_1100_0000_0000,
            0b_0000_1100_0000_0000,
        ];
        pattern.reverse();
        blocks.push(Block { pattern, height: 2 });

        BlockGenerator { blocks }
    }

    fn iter(&self) -> impl Iterator<Item = (usize, Block)> + '_ {
        self.blocks.iter().cloned().enumerate().cycle()
    }
}

#[derive(Clone)]
enum Direction {
    Left,
    Right,
}

struct DirectionGenerator {
    directions: Vec<Direction>,
}

impl TryFrom<&str> for DirectionGenerator {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut directions = Vec::new();
        for ch in value.trim().chars() {
            directions.push(match ch {
                '<' => Direction::Left,
                '>' => Direction::Right,
                _ => bail!("bad input: '{}'", ch),
            });
        }
        Ok(DirectionGenerator { directions })
    }
}

impl DirectionGenerator {
    fn iter(&self) -> impl Iterator<Item = (usize, Direction)> + '_ {
        self.directions.iter().cloned().enumerate().cycle()
    }
}

#[derive(Hash, Eq, PartialEq)]
struct Fingerprint {
    block_index: usize,
    dir_index: usize,
    rows: (u128, u128, u128, u128),
}

struct Cycle {
    blocks_per_cycle: usize,
    cycle_starts_at_block: usize,
    blocks_to_height: Vec<usize>,
}

fn simulate_inner(directions: &DirectionGenerator) -> Cycle {
    let mut well = Well::new();
    let mut tower_height = 0;

    let blocks = BlockGenerator::new();
    let mut blocks_iter = blocks.iter();

    let mut dir_iter = directions.iter();

    let mut cache: HashMap<Fingerprint, Vec<usize>> = HashMap::new();

    // after i blocks, the height of the tower is blocks_to_height[i] units tall
    let mut blocks_to_height = vec![0];

    for block_no in 0..20_000 {
        well.extend_to(tower_height + 9);
        let (block_index, block) = blocks_iter.next().unwrap();
        let mut y = tower_height + 4;
        let mut x = 2;
        let mut dir_index;

        loop {
            let (tmp, dir) = dir_iter.next().unwrap();
            dir_index = tmp;
            match dir {
                Direction::Left => {
                    if well.can_move_left(&block, x, y) {
                        x -= 1;
                    }
                }
                Direction::Right => {
                    if well.can_move_right(&block, x, y) {
                        x += 1;
                    }
                }
            };

            if well.can_move_down(&block, x, y) {
                y -= 1;
            } else {
                break;
            }
        }

        well.add(&block, x, y);
        tower_height = tower_height.max(y + block.height - 1);

        if block_no > 2000 && tower_height - y == 0 {
            cache
                .entry(Fingerprint {
                    block_index,
                    dir_index,
                    rows: well.get_fingerprint(y),
                })
                .or_default()
                .push(block_no);
        }

        blocks_to_height.push(tower_height);
    }

    let (_, x) = cache
        .iter()
        .find(|(_, v)| v.len() > 10)
        .expect("expecting repeating cycles");
    debug_assert!(x.get(1).unwrap() - x.first().unwrap() == x.get(2).unwrap() - x.get(1).unwrap());

    Cycle {
        blocks_per_cycle: x.get(1).unwrap() - x.first().unwrap(),
        cycle_starts_at_block: *x.first().unwrap(),
        blocks_to_height,
    }
}

fn simulate(input: &str, number_of_blocks: usize) -> Result<usize> {
    let directions = DirectionGenerator::try_from(input)?;
    let c = simulate_inner(&directions);
    if c.blocks_to_height.len() > number_of_blocks {
        return Ok(*c.blocks_to_height.get(number_of_blocks).unwrap());
    }

    // 0123456789012345678901234567890
    // |......|....|....|....|....|....
    // |  h1  |         h2        |h3|

    let h1 = c.blocks_to_height[c.cycle_starts_at_block];

    let blocks_left = number_of_blocks - c.cycle_starts_at_block;
    let height_per_cycle = c.blocks_to_height[c.cycle_starts_at_block + c.blocks_per_cycle] - h1;

    let h2 = height_per_cycle * (blocks_left / c.blocks_per_cycle);

    let blocks_left = blocks_left % c.blocks_per_cycle;
    let h3 = c.blocks_to_height[c.cycle_starts_at_block + blocks_left] - h1;

    Ok(h1 + h2 + h3)
}

fn part_one(input: &str) -> Result<usize> {
    simulate(input, 2022)
}

fn part_two(input: &str) -> Result<usize> {
    simulate(input, 1_000_000_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_block() {
        let mut pattern = [
            0b_0000_0000_0000_0000,
            0b_0000_0100_0000_0000,
            0b_0000_1110_0000_0000,
            0b_0000_0100_0000_0000,
        ];
        pattern.reverse();
        let block = Block { pattern, height: 3 };

        assert_eq!(block.get(3, 0), 0b0000_0000_0000_0000);
        assert_eq!(block.get(2, 0), 0b0000_0100_0000_0000);
        assert_eq!(block.get(1, 0), 0b0000_1110_0000_0000);
        assert_eq!(block.get(0, 0), 0b0000_0100_0000_0000);

        assert_eq!(block.get(3, 1), 0b0000_0000_0000_0000);
        assert_eq!(block.get(2, 1), 0b0000_0010_0000_0000);
        assert_eq!(block.get(1, 1), 0b0000_0111_0000_0000);
        assert_eq!(block.get(0, 1), 0b0000_0010_0000_0000);

        assert_eq!(block.get(3, 6), 0b0000_0000_0000_0000);
        assert_eq!(block.get(2, 6), 0b0000_0000_0001_0000);
        assert_eq!(block.get(1, 6), 0b0000_0000_0011_1000);
        assert_eq!(block.get(0, 6), 0b0000_0000_0001_0000);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 3068);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 1_514_285_714_288);
    }
}
