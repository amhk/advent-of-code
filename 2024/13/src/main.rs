use anyhow::Result;
use aoc::XY;
use regex::Regex;

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    aoc::run!(part_one(input), 29877)?;
    aoc::run!(part_two(input), 99423413811305)?;
    Ok(())
}

struct Button {
    delta: XY,
    cost: usize,
}

struct Machine {
    button_a: Button,
    button_b: Button,
    target: XY,
}

fn parse(input: &str) -> Result<Vec<Machine>> {
    let re = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .unwrap();
    let mut machines = vec![];
    for chunk in input.split("\n\n") {
        let (ax, ay, bx, by, targetx, targety): (i32, i32, i32, i32, i32, i32) = aoc::parse!(
            &re,
            chunk,
            |s| s.parse(),
            |s| s.parse(),
            |s| s.parse(),
            |s| s.parse(),
            |s| s.parse(),
            |s| s.parse()
        )?;
        machines.push(Machine {
            button_a: Button {
                delta: (ax, ay).into(),
                cost: 3,
            },
            button_b: Button {
                delta: (bx, by).into(),
                cost: 1,
            },
            target: (targetx, targety).into(),
        });
    }
    Ok(machines)
}

// Solves the following two equations for A and B:
//
//   A * ax + B * bx = tx
//   A * ay + B * by = ty
//
// where
//
//   A is the number of times button A is pushed
//   B is the number of times button B is pushed
//   ax, ay is the delta the claw moves every time button A is pushed
//   bx, by is the delta the claw moves every time button B is pushed
//   tx, ty is the target coordinates
//
// See https://en.wikipedia.org/wiki/Cramer%27s_rule
//
// Returns (A, B) if there is a solution where A and B are both integers
fn solve_equation(
    ax: i128,
    ay: i128,
    bx: i128,
    by: i128,
    tx: i128,
    ty: i128,
) -> Option<(usize, usize)> {
    let a = (tx * by - bx * ty) / (ax * by - bx * ay);
    let b = (ax * ty - tx * ay) / (ax * by - bx * ay);

    if a * ax + b * bx == tx && a * ay + b * by == ty {
        Some((a as usize, b as usize))
    } else {
        None
    }
}

fn part_one(input: &str) -> Result<usize> {
    let mut cost = 0;
    for machine in parse(input)? {
        if let Some((a, b)) = solve_equation(
            machine.button_a.delta.x.into(),
            machine.button_a.delta.y.into(),
            machine.button_b.delta.x.into(),
            machine.button_b.delta.y.into(),
            machine.target.x.into(),
            machine.target.y.into(),
        ) {
            cost += a * machine.button_a.cost + b * machine.button_b.cost;
        }
    }
    Ok(cost)
}

fn part_two(input: &str) -> Result<usize> {
    const OFFSET: i128 = 10000000000000;
    let mut cost = 0;
    for machine in parse(input)? {
        if let Some((a, b)) = solve_equation(
            machine.button_a.delta.x.into(),
            machine.button_a.delta.y.into(),
            machine.button_b.delta.x.into(),
            machine.button_b.delta.y.into(),
            OFFSET + machine.target.x as i128,
            OFFSET + machine.target.y as i128,
        ) {
            cost += a * machine.button_a.cost + b * machine.button_b.cost;
        }
    }
    Ok(cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_solve_equation() {
        assert_eq!(solve_equation(94, 34, 22, 67, 8400, 5400), Some((80, 40)));
        assert_eq!(solve_equation(26, 66, 67, 21, 12748, 12176), None);
        assert_eq!(
            solve_equation(94, 34, 22, 67, 10000000000000 * 8400, 10000000000000 * 5400),
            Some((80 * 10000000000000, 40 * 10000000000000))
        );
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT).unwrap(), 480);
    }
}
