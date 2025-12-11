use anyhow::{anyhow, bail, Result};

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 4364617236318)?;
    aoc::run!(part_two(input), 9077004354241)?;
    Ok(())
}

enum Operation {
    Addition,
    Multiplication,
}

struct Column {
    op: Operation,
    values: Vec<usize>,
}

fn parse_part_one(input: &str) -> Result<Vec<Column>> {
    let mut str_matrix: Vec<Vec<&str>> = vec![vec![]];
    for line in input.lines() {
        for (index, chunk) in line.split_whitespace().enumerate() {
            if str_matrix.len() <= index {
                str_matrix.push(vec![]);
            }
            str_matrix[index].push(chunk);
        }
    }

    let mut matrix: Vec<Column> = Vec::with_capacity(str_matrix.len());
    for mut str_col in str_matrix {
        let op = match str_col.pop().ok_or_else(|| anyhow!("bad input"))? {
            "+" => Operation::Addition,
            "*" => Operation::Multiplication,
            _ => bail!("bad input"),
        };
        let values = str_col
            .into_iter()
            .map(|s| s.parse::<usize>().map_err(|_| anyhow!("bad input")))
            .collect::<Result<Vec<usize>>>()?;
        matrix.push(Column { op, values });
    }

    Ok(matrix)
}

fn parse_part_two(input: &str) -> Result<Vec<Column>> {
    let line_length = input.find("\n").ok_or_else(|| anyhow!("bad input"))?;
    let lines = input.lines().count();

    let input = input.as_bytes();
    let mut transposed: Vec<u8> = Vec::with_capacity(input.len());
    for i in 0..line_length {
        let mut all_blanks = true;
        for j in 0..lines {
            let offset = j * (line_length + 1) + i;
            let ch = input[offset];
            match ch {
                b' ' => {
                    transposed.push(ch);
                }
                b'0'..=b'9' => {
                    transposed.push(ch);
                    all_blanks = false;
                }
                b'+' | b'*' => {
                    transposed.push(b' ');
                    transposed.push(ch);
                    transposed.push(b' ');
                    all_blanks = false;
                }
                _ => bail!("bad input: {}", ch as char),
            }
        }
        if all_blanks {
            transposed.push(b'|');
        }
    }

    let transposed = str::from_utf8(&transposed)?;
    let mut out = vec![];
    for chunk in transposed.split('|') {
        let mut op = None;
        let mut values = vec![];
        for part in chunk.split_whitespace() {
            match part {
                "+" => op = Some(Operation::Addition),
                "*" => op = Some(Operation::Multiplication),
                _ => values.push(part.parse::<usize>().map_err(|_| anyhow!("bad input"))?),
            }
        }
        out.push(Column {
            op: op.ok_or_else(|| anyhow!("missing operation"))?,
            values,
        });
    }

    Ok(out)
}

fn part_x<T: Fn(&str) -> Result<Vec<Column>>>(input: &str, parse_fn: T) -> Result<usize> {
    let mut sum = 0;
    for col in parse_fn(input)? {
        match col.op {
            Operation::Addition => {
                sum += col.values.iter().sum::<usize>();
            }
            Operation::Multiplication => {
                sum += col.values.iter().product::<usize>();
            }
        }
    }
    Ok(sum)
}

fn part_one(input: &str) -> Result<usize> {
    part_x(input, parse_part_one)
}

fn part_two(input: &str) -> Result<usize> {
    part_x(input, parse_part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 4277556);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT).unwrap(), 3263827);
    }
}
