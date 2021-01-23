use regex::{Captures, Regex};
use std::str::FromStr;

fn main() {
    let input = include_str!("input.txt");

    let answer = part_one(&input).expect("no solution for part one");
    println!("part 1: {}", answer);

    let answer = part_two(&input).expect("no solution for part two");
    println!("part 2: {}", answer);
}

type RegisterId = char;

#[derive(Debug)]
struct Register {
    id: RegisterId,
    value: u32,
}

type RelativeOffset = i32;

#[derive(Debug)]
enum Instruction {
    Hlf(RegisterId),
    Tpl(RegisterId),
    Inc(RegisterId),
    Jmp(RelativeOffset),
    Jie(RegisterId, RelativeOffset),
    Jio(RegisterId, RelativeOffset),
}

struct Computer {
    registers: Vec<Register>,
    instructions: Vec<Instruction>,
}

impl Computer {
    fn value_of(&self, id: RegisterId) -> Option<u32> {
        self.registers.iter().find(|r| r.id == id).map(|r| r.value)
    }

    fn set(&mut self, id: RegisterId, value: u32) -> Result<(), Error> {
        self.registers
            .iter_mut()
            .find(|r| r.id == id)
            .ok_or(Error::NoSuchRegister(id))?
            .value = value;
        Ok(())
    }

    fn run(&mut self) -> Result<(), Error> {
        Computer::run_inner(&self.instructions, &mut self.registers)
    }

    fn run_inner(instructions: &[Instruction], registers: &mut [Register]) -> Result<(), Error> {
        fn get_mut(registers: &mut [Register], id: RegisterId) -> Result<&mut Register, Error> {
            match registers.iter_mut().find(|r| r.id == id) {
                Some(r) => Ok(r),
                None => Err(Error::NoSuchRegister(id)),
            }
        }

        let mut pc = 0i32;
        loop {
            if pc < 0 {
                return Ok(());
            }
            match instructions.get(pc as usize) {
                None => return Ok(()),
                Some(Instruction::Hlf(id)) => {
                    let r = get_mut(registers, *id)?;
                    r.value /= 2;
                    pc += 1;
                }
                Some(Instruction::Tpl(id)) => {
                    let r = get_mut(registers, *id)?;
                    r.value *= 3;
                    pc += 1;
                }
                Some(Instruction::Inc(id)) => {
                    let r = get_mut(registers, *id)?;
                    r.value += 1;
                    pc += 1;
                }
                Some(Instruction::Jmp(offset)) => {
                    pc += *offset;
                }
                Some(Instruction::Jie(id, offset)) => {
                    let r = get_mut(registers, *id)?;
                    if r.value % 2 == 0 {
                        pc += *offset;
                    } else {
                        pc += 1;
                    }
                }
                Some(Instruction::Jio(id, offset)) => {
                    let r = get_mut(registers, *id)?;
                    if r.value == 1 {
                        pc += *offset;
                    } else {
                        pc += 1;
                    }
                }
            }
        }
    }
}

impl FromStr for Computer {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn read_reg_id(caps: &Captures, index: usize) -> RegisterId {
            caps.get(index).unwrap().as_str().chars().next().unwrap()
        }
        fn read_offset(caps: &Captures, index: usize) -> i32 {
            caps.get(index).unwrap().as_str().parse::<i32>().unwrap()
        }

        let re_hlf = Regex::new(r"^hlf (.)$").unwrap();
        let re_tpl = Regex::new(r"^tpl (.)$").unwrap();
        let re_inc = Regex::new(r"^inc (.)$").unwrap();
        let re_jmp = Regex::new(r"^jmp ([-+]\d+)$").unwrap();
        let re_jie = Regex::new(r"^jie (.), ([-+]\d+)$").unwrap();
        let re_jio = Regex::new(r"^jio (.), ([-+]\d+)$").unwrap();

        let mut instructions = Vec::new();
        for line in s.lines() {
            if let Some(caps) = re_hlf.captures(line) {
                instructions.push(Instruction::Hlf(read_reg_id(&caps, 1)));
                continue;
            }
            if let Some(caps) = re_tpl.captures(line) {
                instructions.push(Instruction::Tpl(read_reg_id(&caps, 1)));
                continue;
            }
            if let Some(caps) = re_inc.captures(line) {
                instructions.push(Instruction::Inc(read_reg_id(&caps, 1)));
                continue;
            }
            if let Some(caps) = re_jmp.captures(line) {
                instructions.push(Instruction::Jmp(read_offset(&caps, 1)));
                continue;
            }
            if let Some(caps) = re_jie.captures(line) {
                instructions.push(Instruction::Jie(
                    read_reg_id(&caps, 1),
                    read_offset(&caps, 2),
                ));
                continue;
            }
            if let Some(caps) = re_jio.captures(line) {
                instructions.push(Instruction::Jio(
                    read_reg_id(&caps, 1),
                    read_offset(&caps, 2),
                ));
                continue;
            }
            return Err(Error::BadInput);
        }

        Ok(Computer {
            registers: vec![
                Register { id: 'a', value: 0 },
                Register { id: 'b', value: 0 },
            ],
            instructions,
        })
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum Error {
    BadInput,
    NoSuchRegister(RegisterId),
}

fn part_one(input: &str) -> Result<u32, Error> {
    let mut computer = Computer::from_str(input).unwrap();
    computer.run()?;
    computer.value_of('b').ok_or(Error::BadInput)
}

fn part_two(input: &str) -> Result<u32, Error> {
    let mut computer = Computer::from_str(input).unwrap();
    computer.set('a', 1)?;
    computer.run()?;
    computer.value_of('b').ok_or(Error::BadInput)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("test-input.txt");

    #[test]
    fn test_example_program() {
        let mut computer = Computer::from_str(INPUT).unwrap();
        computer.run().unwrap();
        assert_eq!(computer.value_of('a'), Some(2));
    }
}
