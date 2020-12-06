fn main() -> Result<(), ()> {
    let input = include_str!("input.txt");

    let answer = part_one(&input)?;
    println!("part 1: {}", answer);

    let answer = part_two(&input)?;
    println!("part 2: {}", answer);

    Ok(())
}

fn str_to_int_vec(input: &str) -> Vec<u32> {
    input
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

fn part_one(input: &str) -> Result<u32, ()> {
    let input = str_to_int_vec(&input);
    for i in 0..input.len() {
        for j in i..input.len() {
            let a = input[i];
            let b = input[j];
            if a + b == 2020 {
                return Ok(a * b);
            }
        }
    }
    return Err(());
}

fn part_two(input: &str) -> Result<u32, ()> {
    let input = str_to_int_vec(&input);
    for i in 0..input.len() {
        for j in i..input.len() {
            for k in j..input.len() {
                let a = input[i];
                let b = input[j];
                let c = input[k];
                if a + b + c == 2020 {
                    return Ok(a * b * c);
                }
            }
        }
    }
    return Err(());
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1721\n979\n366\n299\n675\n1456";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&INPUT), Ok(514579));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&INPUT), Ok(241861950));
    }
}
