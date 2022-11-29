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

#[derive(Debug)]
struct Reindeer {
    name: String,
    speed: u32,    // km/s
    uptime: u32,   // s
    downtime: u32, // s
}

impl Reindeer {
    fn distance_after(&self, duration: u32) -> u32 {
        enum State {
            Running,
            Resting,
        }

        let mut distance = 0;
        let mut duration = duration;
        let mut state = State::Running;
        while duration > 0 {
            match state {
                State::Running => {
                    let step = u32::min(self.uptime, duration);
                    distance += step * self.speed;
                    duration -= step;
                    state = State::Resting;
                }
                State::Resting => {
                    duration -= u32::min(self.downtime, duration);
                    state = State::Running;
                }
            }
        }
        distance
    }
}

fn parse_input(input: &str) -> Result<Vec<Reindeer>, Error> {
    let mut reindeer = Vec::new();
    for line in input.lines() {
        let words: Vec<_> = line.split_whitespace().collect();
        reindeer.push(Reindeer {
            name: words.first().ok_or(Error::BadInput)?.to_string(),
            speed: words
                .get(3)
                .ok_or(Error::BadInput)?
                .parse::<u32>()
                .map_err(|_| Error::BadInput)?,
            uptime: words
                .get(6)
                .ok_or(Error::BadInput)?
                .parse::<u32>()
                .map_err(|_| Error::BadInput)?,
            downtime: words
                .get(13)
                .ok_or(Error::BadInput)?
                .parse::<u32>()
                .map_err(|_| Error::BadInput)?,
        });
    }
    Ok(reindeer)
}

fn cumulative_score(reindeer: &[Reindeer], duration: u32) -> Vec<(String, u32)> {
    let mut state: Vec<_> = reindeer.iter().map(|r| (r, 0u32, 0u32)).collect();

    for timestamp in 0..duration {
        for (reindeer, distance, _) in state.iter_mut() {
            let is_running = timestamp % (reindeer.uptime + reindeer.downtime) < reindeer.uptime;
            if is_running {
                *distance += reindeer.speed;
            }
        }
        let best_distance = state
            .iter()
            .map(|(_, distance, _)| distance)
            .max()
            .copied()
            .unwrap();
        state
            .iter_mut()
            .filter(|(_, distance, _)| *distance == best_distance)
            .for_each(|(_, _, score)| {
                *score += 1;
            });
    }

    state
        .iter()
        .map(|(reindeer, _, score)| (reindeer.name.clone(), *score))
        .collect()
}

fn part_one(input: &str) -> Result<u32, Error> {
    let reindeer = parse_input(input)?;
    reindeer
        .iter()
        .map(|r| r.distance_after(2503))
        .max()
        .ok_or(Error::BadInput)
}

fn part_two(input: &str) -> Result<u32, Error> {
    let reindeer = parse_input(input)?;
    let scores = cumulative_score(&reindeer, 2503);
    scores
        .iter()
        .map(|(_, score)| score)
        .max()
        .copied()
        .ok_or(Error::BadInput)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_distance_after() {
        let reindeer = parse_input(INPUT).unwrap();
        let comet = reindeer.iter().find(|r| r.name == "Comet").unwrap();
        assert_eq!(comet.distance_after(1000), 1120);
        let dancer = reindeer.iter().find(|r| r.name == "Dancer").unwrap();
        assert_eq!(dancer.distance_after(1000), 1056);
    }

    #[test]
    fn test_cumulative_score() {
        let reindeer = parse_input(INPUT).unwrap();
        let scores = cumulative_score(&reindeer, 1000);
        let comet = scores.iter().find(|(name, _)| name == "Comet").unwrap();
        assert_eq!(comet.1, 312);
        let dancer = scores.iter().find(|(name, _)| name == "Dancer").unwrap();
        assert_eq!(dancer.1, 689);
    }
}
