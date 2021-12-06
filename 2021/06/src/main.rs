fn main() {
    let input = include_str!("input.txt");

    let answer = simulate(input, 80).expect("no solution for part one");
    println!("part 1: {}", answer);

    let answer = simulate(input, 256).expect("no solution for part two");
    println!("part 2: {}", answer);
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum Error {
    BadInput,
}

fn parse_input(input: &str) -> Result<[u64; 7], Error> {
    let mut population = [0; 7];
    for timer in input.trim().split(',') {
        let timer: usize = timer.parse().map_err(|_| Error::BadInput)?;
        if timer >= population.len() {
            return Err(Error::BadInput);
        }
        population[timer] += 1;
    }
    Ok(population)
}

fn simulate(input: &str, steps: usize) -> Result<u64, Error> {
    let mut population = parse_input(input)?;
    let mut seven = 0;
    let mut eight = 0;
    for step in 0..steps {
        let index = step % population.len();
        let tmp = population[index];
        population[index] += seven;
        seven = eight;
        eight = tmp;
    }
    Ok(population.iter().sum::<u64>() + seven + eight)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(INPUT), Ok([0, 1, 1, 2, 1, 0, 0]));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(simulate(INPUT, 18), Ok(26));
        assert_eq!(simulate(INPUT, 80), Ok(5934));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(simulate(INPUT, 256), Ok(26984457539u64));
    }
}
