use std::collections::LinkedList;
use std::io;
use std::io::{Read, Write};

use crate::util;

#[derive(Debug)]
enum ParamMode {
    Position,
    Immediate,
}

type ParamWithMode = (ParamMode, i32);

#[derive(Debug)]
enum Op {
    Add(ParamWithMode, ParamWithMode, usize),
    Mul(ParamWithMode, ParamWithMode, usize),
    Input(usize),
    Output(ParamWithMode),
    JumpIfTrue(ParamWithMode, ParamWithMode),
    JumpIfFalse(ParamWithMode, ParamWithMode),
    LessThan(ParamWithMode, ParamWithMode, usize),
    Equals(ParamWithMode, ParamWithMode, usize),
    Halt,
}

struct Computer<'a, I: Read, O: Write> {
    ram: &'a mut [i32],
    inst_ptr: usize,
    input: I,
    output: O,
}

impl<'a, I: Read, O: Write> Computer<'a, I, O> {
    pub fn new(ram: &'a mut [i32], input: I, output: O) -> Self {
        Self {
            ram,
            inst_ptr: 0,
            input,
            output,
        }
    }

    pub fn run_program(&'a mut self) {
        loop {
            let op = self.read_op();

            match op {
                Ok(Op::Add(a, b, out)) => {
                    self.ram[out] = self.read_param(&a) + self.read_param(&b);
                }
                Ok(Op::Mul(a, b, out)) => {
                    self.ram[out] = self.read_param(&a) * self.read_param(&b);
                }
                Ok(Op::Input(pos)) => {
                    let mut input = String::new();
                    self.input
                        .read_to_string(&mut input)
                        .expect("this will always succeed maybe");

                    let input = input.trim().parse::<i32>().expect("shuold be a num");

                    self.ram[pos] = input;
                }
                Ok(Op::Output(a)) => {
                    write!(self.output, "{}\n", self.read_param(&a)).unwrap();
                    self.output.flush().unwrap();
                }
                Ok(Op::JumpIfTrue(a, b)) => {
                    if self.read_param(&a) != 0 {
                        self.inst_ptr = self.read_param(&b) as usize;
                    }
                }
                Ok(Op::JumpIfFalse(a, b)) => {
                    if self.read_param(&a) == 0 {
                        self.inst_ptr = self.read_param(&b) as usize;
                    }
                }
                Ok(Op::LessThan(a, b, out)) => {
                    let val = if self.read_param(&a) < self.read_param(&b) {
                        1
                    } else {
                        0
                    };
                    self.ram[out] = val;
                }

                Ok(Op::Equals(a, b, out)) => {
                    let val = if self.read_param(&a) == self.read_param(&b) {
                        1
                    } else {
                        0
                    };
                    self.ram[out] = val;
                }
                Ok(Op::Halt) => return,
                Err(e) => panic!(
                    "Received error: {:?}. Current inst_ptr position: {}",
                    e, self.inst_ptr
                ),
            }
        }
    }

    fn read_op<'b>(&'b mut self) -> io::Result<Op> {
        let raw_op = self.ram[self.inst_ptr];
        let opcode = raw_op % 100;
        let mut raw_param_modes = raw_op / 100;
        let mut param_modes = LinkedList::new();

        while raw_param_modes > 0 {
            let param_mode = match raw_param_modes % 10 {
                0 => ParamMode::Position,
                1 => ParamMode::Immediate,
                _ => unreachable!("unknown param mode {}", raw_param_modes % 10),
            };

            param_modes.push_back(param_mode);
            raw_param_modes /= 10;
        }

        let op = match opcode {
            1 => {
                if self.ram[self.inst_ptr..].len() < 4 {
                    return Err(io::Error::new(io::ErrorKind::UnexpectedEof, ""));
                }

                Op::Add(
                    (
                        param_modes.pop_front().unwrap_or(ParamMode::Position),
                        self.ram[self.inst_ptr + 1],
                    ),
                    (
                        param_modes.pop_front().unwrap_or(ParamMode::Position),
                        self.ram[self.inst_ptr + 2],
                    ),
                    self.ram[self.inst_ptr + 3] as usize,
                )
            }
            2 => {
                if self.ram[self.inst_ptr..].len() < 4 {
                    return Err(io::Error::new(io::ErrorKind::UnexpectedEof, ""));
                }

                Op::Mul(
                    (
                        param_modes.pop_front().unwrap_or(ParamMode::Position),
                        self.ram[self.inst_ptr + 1],
                    ),
                    (
                        param_modes.pop_front().unwrap_or(ParamMode::Position),
                        self.ram[self.inst_ptr + 2],
                    ),
                    self.ram[self.inst_ptr + 3] as usize,
                )
            }
            3 => {
                if self.ram[self.inst_ptr..].len() < 1 {
                    return Err(io::Error::new(io::ErrorKind::UnexpectedEof, ""));
                }

                Op::Input(self.ram[self.inst_ptr + 1] as usize)
            }
            4 => {
                if self.ram[self.inst_ptr..].len() < 1 {
                    return Err(io::Error::new(io::ErrorKind::UnexpectedEof, ""));
                }

                Op::Output((
                    param_modes.pop_front().unwrap_or(ParamMode::Position),
                    self.ram[self.inst_ptr + 1],
                ))
            }
            5 => {
                if self.ram[self.inst_ptr..].len() < 3 {
                    return Err(io::Error::new(io::ErrorKind::UnexpectedEof, ""));
                }

                Op::JumpIfTrue(
                    (
                        param_modes.pop_front().unwrap_or(ParamMode::Position),
                        self.ram[self.inst_ptr + 1],
                    ),
                    (
                        param_modes.pop_front().unwrap_or(ParamMode::Position),
                        self.ram[self.inst_ptr + 2],
                    ),
                )
            }
            6 => {
                if self.ram[self.inst_ptr..].len() < 3 {
                    return Err(io::Error::new(io::ErrorKind::UnexpectedEof, ""));
                }

                Op::JumpIfFalse(
                    (
                        param_modes.pop_front().unwrap_or(ParamMode::Position),
                        self.ram[self.inst_ptr + 1],
                    ),
                    (
                        param_modes.pop_front().unwrap_or(ParamMode::Position),
                        self.ram[self.inst_ptr + 2],
                    ),
                )
            }
            7 => {
                if self.ram[self.inst_ptr..].len() < 4 {
                    return Err(io::Error::new(io::ErrorKind::UnexpectedEof, ""));
                }

                Op::LessThan(
                    (
                        param_modes.pop_front().unwrap_or(ParamMode::Position),
                        self.ram[self.inst_ptr + 1],
                    ),
                    (
                        param_modes.pop_front().unwrap_or(ParamMode::Position),
                        self.ram[self.inst_ptr + 2],
                    ),
                    self.ram[self.inst_ptr + 3] as usize,
                )
            }
            8 => {
                if self.ram[self.inst_ptr..].len() < 4 {
                    return Err(io::Error::new(io::ErrorKind::UnexpectedEof, ""));
                }

                Op::Equals(
                    (
                        param_modes.pop_front().unwrap_or(ParamMode::Position),
                        self.ram[self.inst_ptr + 1],
                    ),
                    (
                        param_modes.pop_front().unwrap_or(ParamMode::Position),
                        self.ram[self.inst_ptr + 2],
                    ),
                    self.ram[self.inst_ptr + 3] as usize,
                )
            }
            99 => Op::Halt,
            n => panic!("unknown op code {}", n),
        };

        self.inst_ptr += match op {
            Op::Add(_, _, _) | Op::Mul(_, _, _) | Op::LessThan(_, _, _) | Op::Equals(_, _, _) => 4,
            Op::Input(_) | Op::Output(_) => 2,
            Op::JumpIfTrue(_, _) | Op::JumpIfFalse(_, _) => 3,
            Op::Halt => 0,
        };

        Ok(op)
    }

    fn read_param(&self, param: &ParamWithMode) -> i32 {
        match param {
            (ParamMode::Position, pos) => self.ram[*pos as usize],
            (ParamMode::Immediate, val) => *val,
        }
    }
}

fn read_input(mut r: impl Read) -> Vec<i32> {
    let mut raw = String::new();
    r.read_to_string(&mut raw).unwrap();

    raw.trim()
        .split(",")
        .map(|part| part.parse::<i32>().unwrap())
        .collect()
}

pub fn run_with_io<'a, I>(input: I) -> i32
where
    I: Read,
{
    let r = util::read_input_file("day5.txt");
    let mut ram = read_input(&r[..]);

    let mut output = vec![];
    let mut computer = Computer::new(&mut ram, input, &mut output);
    computer.run_program();

    std::str::from_utf8(&output[..])
        .unwrap()
        .lines()
        .nth_back(0)
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap()
}

pub fn part1() -> i32 {
    let input = [0x31]; // 1;
    run_with_io(&input[..])
}

pub fn part2() -> i32 {
    let input = [0x35]; // 5;
    run_with_io(&input[..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(9006673, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(3629692, part2());
    }
}
