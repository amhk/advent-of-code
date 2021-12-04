use regex::{Captures, Regex};
use std::collections::HashSet;

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

#[allow(dead_code)]
#[derive(Debug)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

fn parse_input(input: &str) -> Result<Vec<Ingredient>, Error> {
    fn read_i32(caps: &Captures, i: usize) -> Result<i32, Error> {
        caps.get(i)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .map_err(|_| Error::BadInput)
    }

    let re = Regex::new(r"^(\w+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (\d+)$").unwrap();
    let mut ingredients = Vec::new();
    for line in input.lines() {
        let caps = re.captures(line).ok_or(Error::BadInput)?;
        ingredients.push(Ingredient {
            name: caps.get(1).unwrap().as_str().to_string(),
            capacity: read_i32(&caps, 2)?,
            durability: read_i32(&caps, 3)?,
            flavor: read_i32(&caps, 4)?,
            texture: read_i32(&caps, 5)?,
            calories: read_i32(&caps, 6)?,
        });
    }
    Ok(ingredients)
}

fn generate_weights(num: usize) -> HashSet<Vec<i32>> {
    fn gen(num: usize, fixed: &[i32], set: &mut HashSet<Vec<i32>>) {
        debug_assert_ne!(num, 0);
        let sum: i32 = fixed.iter().sum();
        if num == 1 {
            for i in 0..=100 {
                match sum + i {
                    100 => {
                        let mut v = fixed.to_vec();
                        v.push(i);
                        set.insert(v);
                    }
                    101..=i32::MAX => {
                        break;
                    }
                    _ => {}
                }
            }
        } else {
            for i in 0..=100 {
                if sum + i > 100 {
                    break;
                }
                let mut v = fixed.to_vec();
                v.push(i);
                gen(num - 1, &v, set);
            }
        }
    }

    let mut set = HashSet::new();
    gen(num, &[], &mut set);
    set
}

fn find_highest_score(ingredients: &[Ingredient], target_calories: Option<i32>) -> i32 {
    let mut highest_score = i32::MIN;
    for weights in generate_weights(ingredients.len()) {
        let (c, d, f, t, cals) = ingredients.iter().zip(weights.iter()).fold(
            (0, 0, 0, 0, 0),
            |(c, d, f, t, cals), (i, w)| {
                (
                    c + i.capacity * w,
                    d + i.durability * w,
                    f + i.flavor * w,
                    t + i.texture * w,
                    cals + i.calories * w,
                )
            },
        );
        if target_calories.is_some() && target_calories != Some(cals) {
            continue;
        }
        let score = if c < 0 || d < 0 || f < 0 || t < 0 {
            0
        } else {
            c * d * f * t
        };
        if score > highest_score {
            highest_score = score;
        }
    }
    highest_score
}

fn part_one(input: &str) -> Result<i32, Error> {
    let ingredients = parse_input(input)?;
    Ok(find_highest_score(&ingredients, None))
}

fn part_two(input: &str) -> Result<i32, Error> {
    let ingredients = parse_input(input)?;
    Ok(find_highest_score(&ingredients, Some(500)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_generate_weights() {
        let mut expected = HashSet::new();
        for i in 0..=100 {
            for j in 0..=100 {
                for k in 0..=100 {
                    if i + j + k == 100 {
                        expected.insert(vec![i, j, k]);
                    }
                }
            }
        }

        assert_eq!(generate_weights(3), expected);
    }

    #[test]
    fn test_part_one() {
        let ingredients = parse_input(INPUT).unwrap();
        assert_eq!(find_highest_score(&ingredients, None), 62842880);
    }

    #[test]
    fn test_part_two() {
        let ingredients = parse_input(INPUT).unwrap();
        assert_eq!(find_highest_score(&ingredients, Some(500)), 57600000);
    }
}
