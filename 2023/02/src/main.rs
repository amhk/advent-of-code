use anyhow::{anyhow, Result};
use regex::Regex;

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 2101)?;
    aoc::run!(part_two(input), 58269)?;
    Ok(())
}

#[derive(Debug)]
struct Game {
    id: usize,
    reveals: Vec<Reveal>,
}

#[derive(Debug)]
struct Reveal {
    red: usize,
    green: usize,
    blue: usize,
}

fn parse(input: &str) -> Result<Vec<Game>> {
    let mut games = vec![];
    let re_game = Regex::new(r"Game (\d+)").unwrap();
    let re_red = Regex::new(r"(\d+) red").unwrap();
    let re_green = Regex::new(r"(\d+) green").unwrap();
    let re_blue = Regex::new(r"(\d+) blue").unwrap();
    for line in input.lines() {
        let caps = re_game
            .captures(line)
            .ok_or(anyhow!("re_game did not find any matches"))?;
        let mut game = Game {
            id: caps.get(1).unwrap().as_str().parse()?,
            reveals: vec![],
        };
        for chunk in line.split(';') {
            game.reveals.push(Reveal {
                red: re_red
                    .captures(chunk)
                    .map(|caps| caps.get(1).unwrap().as_str().parse().unwrap())
                    .unwrap_or_default(),
                green: re_green
                    .captures(chunk)
                    .map(|caps| caps.get(1).unwrap().as_str().parse().unwrap())
                    .unwrap_or_default(),
                blue: re_blue
                    .captures(chunk)
                    .map(|caps| caps.get(1).unwrap().as_str().parse().unwrap())
                    .unwrap_or_default(),
            });
        }

        games.push(game);
    }
    Ok(games)
}

fn part_one(input: &str) -> Result<usize> {
    let games = parse(input)?;
    let games = games
        .into_iter()
        .filter(|g| {
            g.reveals
                .iter()
                .all(|Reveal { red, green, blue }| *red <= 12 && *green <= 13 && *blue <= 14)
        })
        .collect::<Vec<_>>();
    Ok(games.into_iter().map(|g| g.id).sum())
}

fn part_two(input: &str) -> Result<usize> {
    let games = parse(input)?;
    let mut sum = 0;
    for game in games.into_iter() {
        let red = game.reveals.iter().map(|r| r.red).max().unwrap_or_default();
        let green = game
            .reveals
            .iter()
            .map(|r| r.green)
            .max()
            .unwrap_or_default();
        let blue = game
            .reveals
            .iter()
            .map(|r| r.blue)
            .max()
            .unwrap_or_default();
        sum += red * green * blue;
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 8);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 2286);
    }
}
