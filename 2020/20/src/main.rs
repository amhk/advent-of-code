use std::fmt::Debug;
use std::iter::FromIterator;
use std::str::FromStr;

fn main() {
    let input = include_str!("input.txt");

    let answer = part_one(&input).expect("no solution for part one");
    println!("part 1: {}", answer);

    let answer = part_two(&input).expect("no solution for part two");
    println!("part 2: {}", answer);
}

#[derive(Debug, PartialEq)]
enum Error {
    BadInput,
    NoSolution,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Orientation {
    // rotated N degrees (R)
    R0 = 0,
    R90 = 4,
    R180 = 8,
    R270 = 12,

    // flipped along Y axis, then rotated N degrees (ᴙ)
    F0 = 16,
    F90 = 20,
    F180 = 24,
    F270 = 28,
}

const NO_CACHED_VALUE: u32 = u32::MAX;

#[derive(Clone)]
struct Tile {
    orientation: Orientation,
    data: Box<[char]>,
    cache: Vec<u32>,
}

impl Tile {
    fn side(&self) -> usize {
        (self.data.len() as f32).sqrt() as usize
    }

    fn set_orientation(&mut self, new_orientation: Orientation) {
        self.orientation = new_orientation;
    }

    fn northern_edge(&self) -> String {
        let s = self.as_string();
        s[..self.side()].to_string()
    }

    fn eastern_edge(&self) -> String {
        let s = self.as_string();
        String::from_iter(s.chars().skip(self.side() - 1).step_by(self.side()))
    }

    fn southern_edge(&self) -> String {
        let s = self.as_string();
        s[s.len() - self.side()..].to_string()
    }

    fn western_edge(&self) -> String {
        let s = self.as_string();
        String::from_iter(s.chars().step_by(self.side()))
    }

    fn northern_edge_cached(&self) -> u32 {
        self.cache[self.orientation as usize]
    }

    fn eastern_edge_cached(&self) -> u32 {
        self.cache[self.orientation as usize + 1]
    }

    fn southern_edge_cached(&self) -> u32 {
        self.cache[self.orientation as usize + 2]
    }

    fn western_edge_cached(&self) -> u32 {
        self.cache[self.orientation as usize + 3]
    }

    fn populate_cache<F>(&mut self, index: usize, func: F)
    where
        F: Fn(&Tile) -> String,
    {
        let s = func(&self).replace('.', "0").replace('#', "1");
        let c = s.parse::<u32>().unwrap();
        self.cache[index] = c;
    }

    fn as_string(&self) -> String {
        match self.orientation {
            Orientation::R0 => String::from_iter(self.data.iter()),
            Orientation::R90 => String::from_iter(rot90_iter(&mut self.data.iter().cloned())),
            Orientation::R180 => {
                String::from_iter(rot90_iter(&mut rot90_iter(&mut self.data.iter().cloned())))
            }
            Orientation::R270 => String::from_iter(rot90_iter(&mut rot90_iter(&mut rot90_iter(
                &mut self.data.iter().cloned(),
            )))),
            Orientation::F0 => String::from_iter(flip_iter(&mut self.data.iter().cloned())),
            Orientation::F90 => {
                String::from_iter(rot90_iter(&mut flip_iter(&mut self.data.iter().cloned())))
            }
            Orientation::F180 => String::from_iter(rot90_iter(&mut rot90_iter(&mut flip_iter(
                &mut self.data.iter().cloned(),
            )))),
            Orientation::F270 => String::from_iter(rot90_iter(&mut rot90_iter(&mut rot90_iter(
                &mut flip_iter(&mut self.data.iter().cloned()),
            )))),
        }
    }
}

impl FromStr for Tile {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v = Vec::new();
        let mut side = None;
        for line in s.lines() {
            match side {
                None => side = Some(line.len()),
                Some(l) => {
                    if l != line.len() {
                        return Err(Error::BadInput);
                    }
                }
            }
            for ch in line.chars() {
                v.push(ch);
            }
        }
        let side = side.ok_or(Error::BadInput)?;
        if s.lines().count() != side {
            return Err(Error::BadInput);
        }

        let mut t = Tile {
            orientation: Orientation::R0,
            data: v.into_boxed_slice(),
            cache: vec![NO_CACHED_VALUE; 4 * 8],
        };

        for o in &[
            Orientation::R0,
            Orientation::R90,
            Orientation::R180,
            Orientation::R270,
            Orientation::F0,
            Orientation::F90,
            Orientation::F180,
            Orientation::F270,
        ] {
            t.set_orientation(*o);
            t.populate_cache(*o as usize, Tile::northern_edge);
            t.populate_cache(*o as usize + 1, Tile::eastern_edge);
            t.populate_cache(*o as usize + 2, Tile::southern_edge);
            t.populate_cache(*o as usize + 3, Tile::western_edge);
        }

        t.set_orientation(Orientation::R0);

        Ok(t)
    }
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        let side = self.side();
        self.as_string().chars().enumerate().for_each(|(i, ch)| {
            if i % side == 0 {
                s.push('\n');
            }
            s.push(ch);
            s.push(' ');
        });
        f.write_str(&format!("{:?}{}", self.orientation, s))
    }
}

#[derive(Debug, Clone)]
struct PuzzlePiece {
    id: u64,
    tile: Tile,
}

impl FromStr for PuzzlePiece {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let first = lines.next().ok_or(Error::BadInput)?;
        if !first.starts_with("Tile ") || !first.ends_with(':') {
            return Err(Error::BadInput);
        }
        let id = first[5..first.len() - 1]
            .parse::<u64>()
            .map_err(|_| Error::BadInput)?;

        let tile = Tile::from_str(&s[first.len() + 1..])?;

        Ok(PuzzlePiece { tile, id })
    }
}

fn parse_puzzle_pieces(input: &str) -> Result<Vec<PuzzlePiece>, Error> {
    let mut pieces = Vec::new();
    for block in input.split("\n\n") {
        if !block.is_empty() {
            pieces.push(PuzzlePiece::from_str(block)?);
        }
    }
    Ok(pieces)
}

fn rot90_iter(iter: &mut impl Iterator<Item = char>) -> impl Iterator<Item = char> + '_ {
    let chars = iter.collect::<Vec<_>>();
    let side = (chars.len() as f32).sqrt() as usize;
    let mut start_index = chars.len() - side;
    let mut index = start_index;
    std::iter::from_fn(move || {
        if index >= chars.len() {
            return None;
        }
        let ch = chars[index];
        if index >= side {
            index -= side;
        } else {
            start_index += 1;
            index = start_index;
        }
        Some(ch)
    })
}

#[allow(clippy::needless_collect)]
fn flip_iter(iter: &mut impl Iterator<Item = char>) -> impl Iterator<Item = char> + '_ {
    let chars = iter.collect::<Vec<_>>();
    let side = (chars.len() as f32).sqrt() as usize;
    let mut index = side - 1;
    std::iter::from_fn(move || {
        if index >= chars.len() {
            return None;
        }
        let ch = chars[index];
        if index % side == 0 {
            index += 2 * side - 1;
        } else {
            index -= 1;
        }
        Some(ch)
    })
}

#[allow(clippy::needless_collect)]
fn shrink_iter(iter: &mut impl Iterator<Item = char>) -> impl Iterator<Item = char> + '_ {
    let chars = iter.collect::<Vec<_>>();
    let side = (chars.len() as f32).sqrt() as usize;
    let mut index = side + 1;
    std::iter::from_fn(move || {
        if index >= chars.len() - side {
            return None;
        }
        let ch = chars[index];
        index += 1;
        while index % side == 0 || index % side == side - 1 {
            index += 1;
        }
        Some(ch)
    })
}

fn solve_puzzle(pieces: &[PuzzlePiece]) -> Option<Vec<PuzzlePiece>> {
    solve_puzzle0(&Vec::new(), pieces)
}

fn solve_puzzle0(locked_in: &[PuzzlePiece], left: &[PuzzlePiece]) -> Option<Vec<PuzzlePiece>> {
    let side = ((locked_in.len() + left.len()) as f32).sqrt() as usize;
    if left.is_empty() {
        return Some(locked_in.to_vec());
    }

    let index = locked_in.len();
    let edge_to_the_west = if index % side == 0 {
        None
    } else {
        Some(locked_in[index - 1].tile.eastern_edge_cached())
    };
    let edge_to_the_north = if index < side {
        None
    } else {
        Some(locked_in[index - side].tile.southern_edge_cached())
    };

    let mut locked_in = locked_in.to_vec();
    for mut piece in left.iter().cloned() {
        for o in &[
            Orientation::R0,
            Orientation::R90,
            Orientation::R180,
            Orientation::R270,
            Orientation::F0,
            Orientation::F90,
            Orientation::F180,
            Orientation::F270,
        ] {
            piece.tile.set_orientation(*o);
            if let Some(x) = &edge_to_the_west {
                if *x != piece.tile.western_edge_cached() {
                    continue;
                }
            }
            if let Some(x) = &edge_to_the_north {
                if *x != piece.tile.northern_edge_cached() {
                    continue;
                }
            }
            let id = piece.id;
            locked_in.push(piece);
            if let Some(solution) = solve_puzzle0(
                &locked_in,
                &left
                    .iter()
                    .filter(|p| p.id != id)
                    .cloned()
                    .collect::<Vec<_>>(),
            ) {
                return Some(solution);
            }
            piece = locked_in.pop().unwrap();
        }
    }

    None
}

fn stitch_strings(tiles: &[String], tile_side: usize) -> Vec<char> {
    let tiles_per_x = (tiles.len() as f32).sqrt() as usize;
    let canvas_side = tile_side * tiles_per_x;
    let mut canvas: Vec<char> = vec!['_'; tile_side * tile_side * tiles.len()];
    for (ti, t) in tiles.iter().enumerate() {
        let offset = canvas_side * tile_side * (ti / tiles_per_x) + (ti % tiles_per_x) * tile_side;
        for (i, ch) in t.chars().enumerate() {
            let row = i / tile_side;
            let col = i % tile_side;
            let index = offset + row * canvas_side + col;
            canvas[index] = ch;
        }
    }
    canvas
}

#[allow(clippy::iter_nth_zero)]
fn count_sea_monsters(data: &str) -> usize {
    let text_width = data.lines().next().unwrap().len();
    let data = data.replace('\n', "");

    fn mask_iter(data: &str, text_width: usize) -> impl Iterator<Item = Vec<&str>> {
        let mut offset = 0;
        const MASK_WIDTH: usize = 20;
        const MASK_HEIGHT: usize = 3;

        std::iter::from_fn(move || {
            if offset == usize::MAX {
                return None;
            }

            let out = vec![
                &data[offset..(offset + MASK_WIDTH)],
                &data[(offset + text_width)..(offset + text_width + MASK_WIDTH)],
                &data[(offset + 2 * text_width)..(offset + text_width * 2 + MASK_WIDTH)],
            ];

            if (offset + MASK_WIDTH) % text_width == 0 {
                offset += MASK_WIDTH;
            } else {
                offset += 1;
            }

            if offset + MASK_WIDTH + (MASK_HEIGHT - 1) * text_width >= data.len() {
                offset = usize::MAX;
            }

            Some(out)
        })
    }

    let mut count = 0;
    for window in mask_iter(&data, text_width) {
        let sea_monster = window[0].chars().nth(18) == Some('#')
            && window[1].chars().nth(0) == Some('#')
            && window[1].chars().nth(5) == Some('#')
            && window[1].chars().nth(6) == Some('#')
            && window[1].chars().nth(11) == Some('#')
            && window[1].chars().nth(12) == Some('#')
            && window[1].chars().nth(17) == Some('#')
            && window[1].chars().nth(18) == Some('#')
            && window[1].chars().nth(19) == Some('#')
            && window[2].chars().nth(1) == Some('#')
            && window[2].chars().nth(4) == Some('#')
            && window[2].chars().nth(7) == Some('#')
            && window[2].chars().nth(10) == Some('#')
            && window[2].chars().nth(13) == Some('#')
            && window[2].chars().nth(16) == Some('#');

        if sea_monster {
            count += 1;
        }
    }
    count
}

fn part_one(input: &str) -> Result<u64, Error> {
    let puzzle_pieces = parse_puzzle_pieces(input)?;
    let solution = solve_puzzle(&puzzle_pieces).ok_or(Error::NoSolution)?;
    let side = (puzzle_pieces.len() as f32).sqrt() as usize;
    let a = solution[0].id;
    let b = solution[side - 1].id;
    let c = solution[puzzle_pieces.len() - side].id;
    let d = solution[puzzle_pieces.len() - 1].id;
    Ok(a * b * c * d)
}

fn part_two(input: &str) -> Result<u64, Error> {
    let puzzle_pieces = parse_puzzle_pieces(input)?;
    let solution = solve_puzzle(&puzzle_pieces).ok_or(Error::NoSolution)?;
    let tiles = solution
        .iter()
        .map(|p| String::from_iter(shrink_iter(&mut p.tile.as_string().chars())))
        .collect::<Vec<_>>();

    fn print_canvas(iter: impl Iterator<Item = char>) -> String {
        let mut s = String::new();
        for (i, ch) in iter.enumerate() {
            if i > 0 && i % (8 * 12) == 0 {
                s.push('\n');
            }
            s.push(ch);
        }
        s.push('\n');
        s
    }
    let canvas = stitch_strings(&tiles, 8);
    let r0 = print_canvas(canvas.iter().cloned());
    let r90 = print_canvas(rot90_iter(&mut canvas.iter().cloned()));
    let r180 = print_canvas(rot90_iter(&mut rot90_iter(&mut canvas.iter().cloned())));
    let r270 = print_canvas(rot90_iter(&mut rot90_iter(&mut rot90_iter(
        &mut canvas.iter().cloned(),
    ))));
    let f0 = print_canvas(flip_iter(&mut canvas.iter().cloned()));
    let f90 = print_canvas(rot90_iter(&mut flip_iter(&mut canvas.iter().cloned())));
    let f180 = print_canvas(rot90_iter(&mut rot90_iter(&mut flip_iter(
        &mut canvas.iter().cloned(),
    ))));
    let f270 = print_canvas(rot90_iter(&mut rot90_iter(&mut rot90_iter(
        &mut flip_iter(&mut canvas.iter().cloned()),
    ))));

    for canvas in &[r0, r90, r180, r270, f0, f90, f180, f270] {
        let sea_monsters = count_sea_monsters(&canvas);
        if sea_monsters > 0 {
            let hashes = canvas.chars().filter(|&ch| ch == '#').count();
            // each sea monster takes up 15 hashes
            return Ok((hashes - sea_monsters * 15) as u64);
        }
    }

    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_rot90_iter() {
        let r90 = String::from_iter(rot90_iter(&mut "012345678".chars()));
        assert_eq!(&r90, "630741852");

        let r180 = String::from_iter(rot90_iter(&mut rot90_iter(&mut "012345678".chars())));
        assert_eq!(&r180, "876543210");

        let r270 = String::from_iter(rot90_iter(&mut rot90_iter(&mut rot90_iter(
            &mut "012345678".chars(),
        ))));
        assert_eq!(&r270, "258147036");

        let r360 = String::from_iter(rot90_iter(&mut rot90_iter(&mut rot90_iter(
            &mut rot90_iter(&mut "012345678".chars()),
        ))));
        assert_eq!(&r360, "012345678");
    }

    #[test]
    fn test_flip_iter() {
        let s = String::from_iter(flip_iter(&mut "012345678".chars()));
        assert_eq!(&s, "210543876");

        let s = String::from_iter(flip_iter(&mut flip_iter(&mut "012345678".chars())));
        assert_eq!(&s, "012345678");
    }

    #[test]
    fn test_flip_and_rot90_iters() {
        let s = String::from_iter(rot90_iter(&mut flip_iter(&mut "012345678".chars())));
        assert_eq!(&s, "852741630");
    }

    #[test]
    fn test_tile() {
        let s = "####.
#...#
####.
#.#..
#..#.";
        let mut t = Tile::from_str(&s).unwrap();

        t.set_orientation(Orientation::R0);
        assert_eq!(t.northern_edge(), "####.");
        assert_eq!(t.eastern_edge(), ".#...");
        assert_eq!(t.southern_edge(), "#..#.");
        assert_eq!(t.western_edge(), "#####");

        t.set_orientation(Orientation::F270);
        assert_eq!(t.northern_edge(), "#####");
        assert_eq!(t.eastern_edge(), "#..#.");
        assert_eq!(t.southern_edge(), ".#...");
        assert_eq!(t.western_edge(), "####.");
    }

    #[test]
    fn test_shrink_iter() {
        let s = "+---+|012||345||678|+---+";
        assert_eq!(&String::from_iter(shrink_iter(&mut s.chars())), "012345678");
    }

    #[test]
    fn test_solve_puzzle() {
        let puzzle_pieces = parse_puzzle_pieces(INPUT).unwrap();
        let solution = solve_puzzle(&puzzle_pieces).unwrap();
        let solution = solution.iter().map(|p| p.id).collect::<Vec<_>>();

        // The puzzle can be solved in any orientation, including flipped, so the example solution
        // online may not be what was found. We happen to find the solution that is F270 compared
        // to the example online.
        let expected = vec![1951, 2729, 2971, 2311, 1427, 1489, 3079, 2473, 1171];

        assert_eq!(solution, expected);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), Ok(1951 * 3079 * 2971 * 1171));
    }

    #[test]
    fn test_stitch_strings() {
        let strings = vec![
            "aaaAAAααα".to_string(),
            "bbbBBBβββ".to_string(),
            "cccCCCγγγ".to_string(),
            "dddDDDδδδ".to_string(),
        ];
        let chars = stitch_strings(&strings, 3);
        let canvas = String::from_iter(chars.iter());
        assert_eq!(
            String::from_iter(canvas.chars()),
            "aaabbbAAABBBαααβββcccdddCCCDDDγγγδδδ"
        );
    }

    #[test]
    fn test_highlight_sea_monsters() {
        let s = "aaaa..............bbbb
<..................#.>
<#....##....##....###>
<.#..#..#..#..#..#...>
cccc..............dddd
";
        assert_eq!(count_sea_monsters(&s), 1);
    }
}
