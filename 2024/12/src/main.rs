use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use anyhow::Result;
use aoc::XY;

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 1461752)?;
    aoc::run!(part_two(input), 904114)?;
    Ok(())
}

type ID = usize;

fn parse(input: &str) -> Result<HashMap<ID, HashSet<XY>>> {
    let mut all_cliques: HashMap<char, Vec<HashSet<XY>>> = HashMap::new();
    let _ = aoc::parse_grid(input, |xy, ch| {
        if let Some(cliques) = all_cliques.get_mut(&ch) {
            let west_index = cliques.iter().position(|set| set.contains(&xy.west()));
            let north_index = cliques.iter().position(|set| set.contains(&xy.north()));

            match (west_index, north_index) {
                (Some(i), Some(j)) => {
                    // extend existing clique
                    cliques[i].insert(xy);
                    // merge existing cliques if different
                    match i.cmp(&j) {
                        Ordering::Less => {
                            let set = cliques.remove(j);
                            cliques[i].extend(set);
                        }
                        Ordering::Equal => {
                            // same clique, no need to merge anything
                        }
                        Ordering::Greater => {
                            let set = cliques.remove(i);
                            cliques[j].extend(set);
                        }
                    }
                }
                (Some(i), None) | (None, Some(i)) => {
                    // extend existing clique
                    cliques[i].insert(xy);
                }
                (None, None) => {
                    // new clique
                    cliques.push(HashSet::from_iter([xy]));
                }
            }
        } else {
            // first time we see ch, create new vec and new clique
            all_cliques.insert(ch, vec![HashSet::from_iter([xy].into_iter())]);
        }
        Ok(())
    });
    let regions: HashMap<ID, HashSet<XY>> =
        HashMap::from_iter(all_cliques.into_values().flatten().enumerate());
    Ok(regions)
}

fn count_corners(region: &HashSet<XY>) -> usize {
    if region.is_empty() {
        return 0;
    }
    let mut count = 0;
    for xy in region {
        if !region.contains(&xy.north_east()) {
            if region.contains(&xy.north()) == region.contains(&xy.east()) {
                count += 1;
            }
        } else if !region.contains(&xy.north()) && !region.contains(&xy.east()) {
            count += 1;
        }
        if !region.contains(&xy.south_east()) {
            if region.contains(&xy.east()) == region.contains(&xy.south()) {
                count += 1;
            }
        } else if !region.contains(&xy.east()) && !region.contains(&xy.south()) {
            count += 1;
        }
        if !region.contains(&xy.south_west()) {
            if region.contains(&xy.south()) == region.contains(&xy.west()) {
                count += 1;
            }
        } else if !region.contains(&xy.south()) && !region.contains(&xy.west()) {
            count += 1;
        }
        if !region.contains(&xy.north_west()) {
            if region.contains(&xy.west()) == region.contains(&xy.north()) {
                count += 1;
            }
        } else if !region.contains(&xy.west()) && !region.contains(&xy.north()) {
            count += 1;
        }
    }
    debug_assert!(count > 0);
    debug_assert!(count % 2 == 0);
    count
}

fn part_one(input: &str) -> Result<usize> {
    let regions = parse(input)?;
    let mut price = 0;
    for set in regions.into_values() {
        let mut perimeter = 0;
        for xy in set.iter() {
            perimeter += xy
                .four_neighbours()
                .iter()
                .filter(|n| !set.contains(n))
                .count();
        }
        price += set.len() * perimeter;
    }
    Ok(price)
}

fn part_two(input: &str) -> Result<usize> {
    let regions = parse(input)?;
    let mut price = 0;
    for set in regions.into_values() {
        price += set.len() * count_corners(&set);
    }
    Ok(price)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_parse() {
        // AAA
        // ABA
        // AAA
        let regions = parse("AAA\nABA\nAAA").unwrap();
        assert_eq!(regions.keys().len(), 2);

        // AAA
        // ABA
        // ABA
        let regions = parse("AAA\nABA\nABA").unwrap();
        assert_eq!(regions.keys().len(), 2);

        // AAA
        // BBB
        // AAA
        let regions = parse("AAA\nBBB\nAAA").unwrap();
        assert_eq!(regions.keys().len(), 3);

        // AAB
        // BAB
        // BBB
        let regions = parse("AAB\nBAB\nBBB").unwrap();
        assert_eq!(regions.keys().len(), 2);

        // AAB
        // BAB
        // BBB
        // CCC
        // BBA
        let regions = parse("AAB\nBAB\nBBB\nCCC\nBBA").unwrap();
        assert_eq!(regions.keys().len(), 5);

        // AAAA
        // BBCD
        // BBCC
        // EEEC
        let regions = parse("AAAA\nBBCD\nBBCC\nEEEC").unwrap();
        assert_eq!(regions.keys().len(), 5);

        let regions = parse(INPUT).unwrap();
        assert_eq!(regions.keys().len(), 11);
    }

    #[test]
    fn test_count_corners() {
        // FIXME: uncomment
        //let regions = parse("A").unwrap();
        //assert_eq!(count_corners(&regions[&0]), 4);

        //let regions = parse("AA").unwrap();
        //assert_eq!(count_corners(&regions[&0]), 4);

        //let regions = parse("AA\nBA").unwrap();
        //let mut actual: Vec<usize> = regions.values().map(count_corners).collect();
        //actual.sort();
        //let mut expected: Vec<usize> = vec![4, 6];
        //expected.sort();
        //assert_eq!(actual, expected);

        //let regions = parse("ABC\nADE\nAAA").unwrap();
        //let mut actual: Vec<usize> = regions.values().map(count_corners).collect();
        //actual.sort();
        //let mut expected: Vec<usize> = vec![4, 4, 4, 4, 6];
        //expected.sort();
        //assert_eq!(actual, expected);

        let regions = parse("AAAAAA\nAAABBA\nAAABBA\nABBAAA\nABBAAA\nAAAAAA").unwrap();
        let mut actual: Vec<usize> = regions.values().map(count_corners).collect();
        actual.sort();
        let mut expected: Vec<usize> = vec![12, 4, 4];
        expected.sort();
        assert_eq!(actual, expected);

        let regions = parse(INPUT).unwrap();
        let mut actual: Vec<usize> = regions.values().map(count_corners).collect();
        actual.sort();
        let mut expected: Vec<usize> = vec![10, 4, 22, 12, 10, 12, 4, 8, 16, 6, 6];
        expected.sort();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 1930);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("AAAA\nBBCD\nBBCC\nEEEC").unwrap(), 80);
        assert_eq!(part_two("EEEEE\nEXXXX\nEEEEE\nEXXXX\nEEEEE").unwrap(), 236);
        assert_eq!(
            part_two("AAAAAA\nAAABBA\nAAABBA\nABBAAA\nABBAAA\nAAAAAA").unwrap(),
            368
        );
        assert_eq!(part_two(INPUT).unwrap(), 1206);
    }
}
