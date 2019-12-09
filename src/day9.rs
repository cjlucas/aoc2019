use permutate::Permutator;
use std::collections::{HashSet, LinkedList};
use std::io::{self, Read, Write};

use crate::util;

#[derive(Debug)]
enum ParamMode {
    Position,
    Immediate,
    Relative,
}

type ParamWithMode = (ParamMode, i64);

#[derive(Debug)]
enum Op {
    Add(ParamWithMode, ParamWithMode, ParamWithMode),
    Mul(ParamWithMode, ParamWithMode, ParamWithMode),
    Input(ParamWithMode),
    Output(ParamWithMode),
    JumpIfTrue(ParamWithMode, ParamWithMode),
    JumpIfFalse(ParamWithMode, ParamWithMode),
    LessThan(ParamWithMode, ParamWithMode, ParamWithMode),
    Equals(ParamWithMode, ParamWithMode, ParamWithMode),
    ModifyRelativeBase(ParamWithMode),
    Halt,
}

struct Computer {
    ram: Vec<i64>,
    inst_ptr: usize,
    relative_base: i64,
}

#[derive(Debug)]
enum ReturnMode {
    Output(i64),
    Halt,
}

impl Computer {
    pub fn new(ram: Vec<i64>) -> Self {
        Self {
            ram,
            inst_ptr: 0,
            relative_base: 0,
        }
    }

    fn grow_ram_if_necessary(&mut self, pos: usize) {
        if pos + 1 > self.ram.len() {
            let grow = pos + 1 - self.ram.len();
            for _ in 0..grow {
                self.ram.push(0);
            }
        }
    }

    pub fn run_program<I: Read>(&mut self, mut input: I) -> ReturnMode {
        loop {
            let raw_op = self.ram[self.inst_ptr];
            // println!("Running raw_op: {:?}", raw_op);
            let op = self.read_op();

            // println!("Running op: {:?}", op);

            match op {
                Ok(Op::Add(a, b, out)) => {
                    let a = self.read_param(&a);
                    let b = self.read_param(&b);
                    self.write_param(&out, a + b);
                }
                Ok(Op::Mul(a, b, out)) => {
                    let a = self.read_param(&a);
                    let b = self.read_param(&b);
                    self.write_param(&out, a * b);
                }
                Ok(Op::Input(out)) => {
                    let mut buf = [0u8; 8];

                    input.read(&mut buf[..]).unwrap();

                    self.write_param(&out, i64::from_be_bytes(buf));
                }
                Ok(Op::Output(a)) => {
                    return ReturnMode::Output(self.read_param(&a));
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
                    self.write_param(&out, val);
                }

                Ok(Op::Equals(a, b, out)) => {
                    let val = if self.read_param(&a) == self.read_param(&b) {
                        1
                    } else {
                        0
                    };

                    self.write_param(&out, val);
                }
                Ok(Op::ModifyRelativeBase(a)) => self.relative_base += self.read_param(&a),
                Ok(Op::Halt) => return ReturnMode::Halt,
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
                2 => ParamMode::Relative,
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
                    (
                        param_modes.pop_front().unwrap_or(ParamMode::Position),
                        self.ram[self.inst_ptr + 3],
                    ),
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
                    (
                        param_modes.pop_front().unwrap_or(ParamMode::Position),
                        self.ram[self.inst_ptr + 3],
                    ),
                )
            }
            3 => {
                if self.ram[self.inst_ptr..].len() < 2 {
                    return Err(io::Error::new(io::ErrorKind::UnexpectedEof, ""));
                }

                Op::Input((
                    param_modes.pop_front().unwrap_or(ParamMode::Position),
                    self.ram[self.inst_ptr + 1],
                ))
            }
            4 => {
                if self.ram[self.inst_ptr..].len() < 2 {
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
                    (
                        param_modes.pop_front().unwrap_or(ParamMode::Position),
                        self.ram[self.inst_ptr + 3],
                    ),
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
                    (
                        param_modes.pop_front().unwrap_or(ParamMode::Position),
                        self.ram[self.inst_ptr + 3],
                    ),
                )
            }
            9 => {
                if self.ram[self.inst_ptr..].len() < 2 {
                    return Err(io::Error::new(io::ErrorKind::UnexpectedEof, ""));
                }

                Op::ModifyRelativeBase((
                    param_modes.pop_front().unwrap_or(ParamMode::Position),
                    self.ram[self.inst_ptr + 1],
                ))
            }
            99 => Op::Halt,
            n => panic!("unknown op code {}", n),
        };

        self.inst_ptr += match op {
            Op::Add(_, _, _) | Op::Mul(_, _, _) | Op::LessThan(_, _, _) | Op::Equals(_, _, _) => 4,
            Op::Input(_) | Op::Output(_) | Op::ModifyRelativeBase(_) => 2,
            Op::JumpIfTrue(_, _) | Op::JumpIfFalse(_, _) => 3,
            Op::Halt => 0,
        };

        Ok(op)
    }

    fn read_param(&mut self, param: &ParamWithMode) -> i64 {
        match param {
            (ParamMode::Position, pos) => {
                let pos = *pos as usize;
                self.grow_ram_if_necessary(pos);
                self.ram[pos]
            }
            (ParamMode::Immediate, val) => *val,
            (ParamMode::Relative, pos) => {
                let pos = (*pos + self.relative_base) as usize;
                self.grow_ram_if_necessary(pos);
                self.ram[pos]
            }
        }
    }

    fn write_param(&mut self, param: &ParamWithMode, val: i64) {
        match param {
            (ParamMode::Position, pos) => {
                let pos = *pos as usize;
                self.grow_ram_if_necessary(pos);
                self.ram[pos] = val;
            }
            (ParamMode::Immediate, val) => unreachable!("cant write given an immediate value"),
            (ParamMode::Relative, pos) => {
                let pos = (*pos + self.relative_base) as usize;
                self.grow_ram_if_necessary(pos);
                self.ram[pos] = val;
            }
        }
    }
}

pub fn read_input(mut r: impl Read) -> Vec<i64> {
    let mut raw = String::new();
    r.read_to_string(&mut raw).unwrap();

    raw.trim()
        .split(",")
        .map(|part| part.parse::<i64>().unwrap())
        .collect()
}

fn run_with_input(ram: &Vec<i64>, input: i64) -> i64 {
    let input = input.to_be_bytes();
    let mut computer = Computer::new(ram.clone());
    let mut output = 0;
    while let ReturnMode::Output(x) = computer.run_program(&input[..]) {
        output = x;
    }

    output
}

pub fn part1(ram: &Vec<i64>) -> i64 {
    run_with_input(ram, 1)
}

pub fn part2(ram: &Vec<i64>) -> i64 {
    run_with_input(ram, 2)
}
