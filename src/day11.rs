use std::collections::{HashMap, LinkedList};
use std::io::{self, Read};

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
            let op = self.read_op();

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
            (ParamMode::Immediate, _) => unreachable!("cant write given an immediate value"),
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

type Point = (i64, i64);

#[derive(Debug)]
enum Direction {
    N,
    E,
    S,
    W,
}

pub fn part1(input: impl Read) -> usize {
    let ram = read_input(input);
    let painted = run_program(ram, true);

    painted.len()
}

pub fn run_program(ram: Vec<i64>, first_panel_black: bool) -> HashMap<Point, bool> {
    let mut painted = HashMap::<Point, bool>::new();

    let mut computer = Computer::new(ram);
    let mut input = (if first_panel_black { 0i64 } else { 1i64 }).to_be_bytes();
    let mut point = (0, 0);
    let mut direction = Direction::N;

    let mut waiting_for_color = true;
    while let ReturnMode::Output(x) = computer.run_program(&input[..]) {
        if waiting_for_color {
            painted.insert(point, x == 1);
            waiting_for_color = false;
        } else {
            direction = match (&direction, x) {
                (Direction::N, 0) => Direction::W,
                (Direction::N, 1) => Direction::E,
                (Direction::E, 0) => Direction::N,
                (Direction::E, 1) => Direction::S,
                (Direction::S, 0) => Direction::E,
                (Direction::S, 1) => Direction::W,
                (Direction::W, 0) => Direction::S,
                (Direction::W, 1) => Direction::N,
                _ => unreachable!("unkown direction {:?} or cmd {}", direction, x),
            };

            point = match direction {
                Direction::N => (point.0, point.1 + 1),
                Direction::E => (point.0 + 1, point.1),
                Direction::S => (point.0, point.1 - 1),
                Direction::W => (point.0 - 1, point.1),
            };

            let is_white = painted.get(&point).unwrap_or(&false);
            input = (if *is_white { 1 } else { 0 } as i64).to_be_bytes();
            waiting_for_color = true;
        }
    }

    painted
}

pub fn part2(input: impl Read) {
    let ram = read_input(input);
    let painted = run_program(ram, false);

    let minx = painted.keys().min_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let maxx = painted.keys().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let miny = painted.keys().min_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    let maxy = painted.keys().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;

    println!("{} {} {} {}", minx, maxx, miny, maxy);

    for x in minx..=maxx {
        for y in miny..=maxy {
            let is_white = painted.get(&(x, y)).unwrap_or(&false);
            print!("{}", if *is_white { 'X' } else { ' ' });
        }

        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::util;

    #[test]
    fn part1_input() {
        let input = util::read_input_file("day11.txt");
        assert_eq!(2184, part1(&input[..]));
    }
}
