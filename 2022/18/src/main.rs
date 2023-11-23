use anyhow::{Context, Result};
use rustc_hash::{FxHashMap, FxHashSet};

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 3326)?;
    aoc::run!(part_two(input), 1996)?;
    Ok(())
}

#[allow(clippy::upper_case_acronyms)]
type XYZ = (i32, i32, i32);

#[derive(PartialEq)]
enum Cube {
    UnknownSpace,
    OuterSpace,
    SolidMatter(usize),
}

struct Space {
    cubes: FxHashMap<XYZ, Cube>,
    min_x: i32,
    min_y: i32,
    min_z: i32,
}

impl Space {
    fn surface_area(&mut self) -> usize {
        let keys_solid_matter: FxHashSet<_> = self
            .cubes
            .iter()
            .filter_map(|(k, v)| match v {
                Cube::SolidMatter(_) => Some(k),
                _ => None,
            })
            .cloned()
            .collect();
        for ((x, y, z), sides) in self.cubes.iter_mut().filter_map(|(k, v)| match v {
            Cube::SolidMatter(sides) => Some((k, sides)),
            _ => None,
        }) {
            for (nx, ny, nz) in [
                (x + 1, *y, *z),
                (x - 1, *y, *z),
                (*x, y + 1, *z),
                (*x, y - 1, *z),
                (*x, *y, z + 1),
                (*x, *y, z - 1),
            ] {
                if keys_solid_matter.contains(&(nx, ny, nz)) {
                    *sides -= 1;
                }
            }
        }
        self.cubes
            .values()
            .filter_map(|cube| match cube {
                Cube::SolidMatter(sides) => Some(sides),
                _ => None,
            })
            .sum()
    }

    fn solidify_trapped_space(&mut self) {
        fn visit(space: &mut Space, x: i32, y: i32, z: i32) {
            *space.cubes.get_mut(&(x, y, z)).unwrap() = Cube::OuterSpace;
            for (nx, ny, nz) in [
                (x + 1, y, z),
                (x - 1, y, z),
                (x, y + 1, z),
                (x, y - 1, z),
                (x, y, z + 1),
                (x, y, z - 1),
            ] {
                if space.cubes.get(&(nx, ny, nz)) == Some(&Cube::UnknownSpace) {
                    visit(space, nx, ny, nz);
                }
            }
        }

        // because of the added space around the bounding box, (min_x, min_y, min_z) is guaranteed
        // to be part of the outer space
        debug_assert!(
            self.cubes.get(&(self.min_x, self.min_y, self.min_z)) == Some(&Cube::UnknownSpace)
        );
        visit(self, self.min_x, self.min_y, self.min_z);

        self.cubes
            .iter_mut()
            .filter_map(|(_, cube)| match cube {
                Cube::UnknownSpace => Some(cube),
                _ => None,
            })
            .for_each(|cube| *cube = Cube::SolidMatter(6));
    }
}

impl TryFrom<&str> for Space {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        fn next_i32(iter: &mut dyn Iterator<Item = &str>) -> Result<i32> {
            iter.next()
                .context("unexpected empty iterator")?
                .parse()
                .context("failed to convert to i32")
        }

        let mut cubes = FxHashMap::default();
        for line in value.lines() {
            let mut iter = line.split(',');
            let x = next_i32(&mut iter)?;
            let y = next_i32(&mut iter)?;
            let z = next_i32(&mut iter)?;
            cubes.insert((x, y, z), Cube::SolidMatter(6));
        }

        let min_x = cubes.keys().map(|(x, _, _)| x).min().unwrap() - 1;
        let max_x = cubes.keys().map(|(x, _, _)| x).max().unwrap() + 1;
        let min_y = cubes.keys().map(|(_, y, _)| y).min().unwrap() - 1;
        let max_y = cubes.keys().map(|(_, y, _)| y).max().unwrap() + 1;
        let min_z = cubes.keys().map(|(_, _, z)| z).min().unwrap() - 1;
        let max_z = cubes.keys().map(|(_, _, z)| z).max().unwrap() + 1;

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                for z in min_z..=max_z {
                    cubes.entry((x, y, z)).or_insert(Cube::UnknownSpace);
                }
            }
        }

        Ok(Space {
            cubes,
            min_x,
            min_y,
            min_z,
        })
    }
}

fn part_one(input: &str) -> Result<usize> {
    let mut space: Space = input.try_into()?;
    Ok(space.surface_area())
}

fn part_two(input: &str) -> Result<usize> {
    let mut space: Space = input.try_into()?;
    space.solidify_trapped_space();
    Ok(space.surface_area())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("1,1,1\n2,1,1").unwrap(), 10);
        assert_eq!(part_one(INPUT).unwrap(), 64);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("1,1,1\n2,1,1").unwrap(), 10);
        assert_eq!(part_two(INPUT).unwrap(), 58);
    }
}
