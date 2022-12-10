use anyhow::{bail, Context, Result};

fn main() -> Result<()> {
    let input = include_str!("input.txt");

    let answer = part_one(input).context("no solution for part one")?;
    println!("Part 1: {}", answer);

    let answer = part_two(input).context("no solution for part two")?;
    println!("Part 2:\n{}", answer);

    Ok(())
}

fn parse(input: &str) -> Result<Vec<i32>> {
    let mut values: Vec<i32> = vec![1];
    for line in input.lines() {
        match &line[..4] {
            "noop" => {
                values.push(*values.last().unwrap());
            }
            "addx" => {
                let (_, term) = line.split_once(' ').context("bad input")?;
                let term: i32 = term.parse().context("failed to convert to i32")?;
                let last = *values.last().unwrap();
                values.push(last);
                values.push(last + term);
            }
            _ => bail!("unexpected input '{}'", line),
        }
    }
    Ok(values)
}

fn part_one(input: &str) -> Result<i32> {
    let values = parse(input)?;
    let mut sum = 0;
    for i in (20..values.len()).step_by(40) {
        sum += i as i32 * values[i - 1];
    }
    Ok(sum)
}

fn part_two(input: &str) -> Result<String> {
    let values = parse(input)?;
    let mut output = String::new();
    for (i, value) in values.iter().enumerate().take(240) {
        if i % 40 == 0 && i != 0 {
            output.push('\n');
        }
        if (i as i32 % 40).abs_diff(*value as i32) <= 1 {
            output.push('#');
        } else {
            output.push('.');
        }
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_parse() {
        let values = parse(INPUT).unwrap();
        assert_eq!(values[19], 21);
        assert_eq!(values[59], 19);
        assert_eq!(values[99], 18);
        assert_eq!(values[139], 21);
        assert_eq!(values[179], 16);
        assert_eq!(values[219], 18);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 13140);
    }

    #[test]
    fn test_part_two() {
        let expected = "##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....";
        assert_eq!(part_two(INPUT).unwrap(), expected);
    }
}
