use std::str::FromStr;

fn main() {
    let input = include_str!("input.txt");

    let answer = part_one(input);
    println!("part 1: {}", answer);

    let answer = part_two(input);
    println!("part 2: {}", answer);
}

#[derive(Debug)]
enum Error {
    BadInput(String),
    ExecOutOfBounds(usize),
    JumpOutOfBounds(usize, i64),
    InfiniteLoop(usize),
}

#[derive(Debug, PartialEq, Eq)]
enum State {
    Running,
    Terminated,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Op {
    Nop(i64),
    Acc(i64),
    Jmp(i64),
    End,
}

struct Program {
    ops: Vec<Option<Op>>,
    acc: i64,
    pc: usize,
}

impl Program {
    fn step(&mut self) -> Result<(State, i64), Error> {
        let ops_len = self.ops.len() as i64;
        let op = self
            .ops
            .get_mut(self.pc)
            .ok_or(Error::ExecOutOfBounds(self.pc))?;
        match *op {
            Some(Op::Nop(_)) => {
                self.pc += 1;
            }
            Some(Op::Acc(v)) => {
                self.pc += 1;
                self.acc += v;
            }
            Some(Op::Jmp(v)) => {
                let new_pc = self.pc as i64 + v;
                if new_pc < 0 || new_pc >= ops_len {
                    return Err(Error::JumpOutOfBounds(self.pc, v));
                }
                self.pc = new_pc as usize;
            }
            Some(Op::End) => return Ok((State::Terminated, self.acc)),
            None => {
                return Err(Error::InfiniteLoop(self.pc));
            }
        }
        *op = None;
        Ok((State::Running, self.acc))
    }

    fn run(&mut self) -> Result<i64, Error> {
        loop {
            match self.step() {
                Ok((State::Running, _)) => { /* loop */ }
                Ok((State::Terminated, acc)) => return Ok(acc),
                Err(e) => return Err(e),
            }
        }
    }
}

impl FromStr for Program {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ops = Vec::new();
        for (lineno, line) in s.lines().enumerate() {
            if line.len() < 6 {
                return Err(Error::BadInput(format!(
                    "{}: {}: line too short",
                    lineno, line
                )));
            }
            let value = &line[4..]
                .parse::<i64>()
                .map_err(|_| Error::BadInput(format!("{}: {}: bad int arg", lineno, line)))?;
            let op = match &line[..3] {
                "nop" => Op::Nop(*value),
                "acc" => Op::Acc(*value),
                "jmp" => Op::Jmp(*value),
                _ => {
                    return Err(Error::BadInput(format!(
                        "{}: {}: unknown instruction",
                        lineno, line
                    )))
                }
            };
            ops.push(Some(op));
        }
        ops.push(Some(Op::End));
        Ok(Program { ops, acc: 0, pc: 0 })
    }
}

impl std::fmt::Debug for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Program(acc={}, pc={}, size={}, line={:?})",
            self.acc,
            self.pc,
            self.ops.len(),
            self.ops.get(self.pc),
        )
    }
}

impl Clone for Program {
    fn clone(&self) -> Program {
        Program {
            ops: self.ops.to_vec(),
            acc: self.acc,
            pc: self.pc,
        }
    }
}

fn part_one(input: &str) -> i64 {
    let mut p = Program::from_str(input).unwrap();
    let mut acc = 0;
    loop {
        match p.step() {
            Ok((_, v)) => {
                acc = v;
            }
            Err(Error::InfiniteLoop(_)) => {
                return acc;
            }
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
}

fn part_two(input: &str) -> i64 {
    let original = Program::from_str(input).unwrap();
    let mut patch_pos = 0;
    loop {
        let mut p = original.clone();
        loop {
            let op = p.ops.get_mut(patch_pos).unwrap();
            match *op {
                Some(Op::Nop(v)) => {
                    *op = Some(Op::Jmp(v));
                    patch_pos += 1;
                    break;
                }
                Some(Op::Jmp(v)) => {
                    *op = Some(Op::Nop(v));
                    patch_pos += 1;
                    break;
                }
                Some(_) => {
                    patch_pos += 1;
                }
                None => {
                    panic!("patching exhausted but no solution found");
                }
            }
        }
        if let Ok(acc) = p.run() {
            return acc;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_program_from_str() {
        Program::from_str(INPUT).unwrap();

        let p = Program::from_str("jmp -10").unwrap();
        assert_eq!(p.ops.len(), 2);
        assert_eq!(p.ops[0], Some(Op::Jmp(-10)));
        assert_eq!(p.ops[1], Some(Op::End));
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 5);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(INPUT), 8);
    }
}
