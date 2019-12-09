use permutate::Permutator;
use std::collections::{HashSet, LinkedList};
use std::io::{self, Read, Write};

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

struct Computer {
    ram: Vec<i32>,
    inst_ptr: usize,
}

enum ReturnMode {
    Output(i32),
    Halt,
}

impl Computer {
    pub fn new(ram: Vec<i32>) -> Self {
        Self { ram, inst_ptr: 0 }
    }

    pub fn run_program<I: Read>(&mut self, mut input: I) -> ReturnMode {
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
                    let mut buf = [0u8; 4];

                    input.read(&mut buf[..]).unwrap();

                    self.ram[pos] = i32::from_be_bytes(buf);
                    // println!("read input as {}", self.ram[pos]);
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

pub fn read_input(mut r: impl Read) -> Vec<i32> {
    let mut raw = String::new();
    r.read_to_string(&mut raw).unwrap();

    raw.trim()
        .split(",")
        .map(|part| part.parse::<i32>().unwrap())
        .collect()
}

fn part1_inner(phases: &Vec<i32>, ram: &Vec<i32>) -> i32 {
    let mut signal: i32 = 0;
    for phase in phases {
        let ram = ram.clone();

        let mut input = vec![];
        input.write(&phase.to_be_bytes()).unwrap();
        input.write(&signal.to_be_bytes()).unwrap();

        // println!("input: {:?}", input);

        let mut computer = Computer::new(ram);
        match computer.run_program(&input[..]) {
            ReturnMode::Output(out) => signal = out,
            ReturnMode::Halt => return signal,
        }
    }

    signal
}

pub fn part1(ram: &Vec<i32>) -> (Vec<i32>, i32) {
    let phases: Vec<&i32> = vec![&0, &1, &2, &3, &4];
    let phases = [&phases[..]];

    let mut permutator = Permutator::new(&phases[..]);

    let mut max_phases = vec![];
    let mut max = 0;
    while let Some(phases) = permutator.next() {
        let phases: Vec<_> = phases.iter().map(|x| **x).collect();

        let set: HashSet<_> = phases.iter().collect();
        if set.len() < 5 {
            continue;
        }

        let cur = part1_inner(&phases, &ram);
        if cur > max {
            max = cur;
            max_phases = phases
        }
    }
    (max_phases, max)
}

fn part2_inner(phases: &Vec<i32>, ram: &Vec<i32>) -> i32 {
    let mut signal: i32 = 0;

    let mut computers = vec![];
    for phase in phases {
        let ram = ram.clone();

        let mut input = vec![];
        input.write(&phase.to_be_bytes()).unwrap();
        input.write(&signal.to_be_bytes()).unwrap();

        // println!("input: {:?}", input);

        let mut computer = Computer::new(ram);

        match computer.run_program(&input[..]) {
            ReturnMode::Output(out) => {
                signal = out;
                computers.push((true, computer));
            }
            ReturnMode::Halt => {
                unreachable!("this should never have halted on the first iteration")
            }
        }
    }

    loop {
        if computers.iter().all(|(running, _)| !running) {
            return signal;
        }

        for (ref mut running, ref mut computer) in &mut computers {
            let mut input = vec![];
            input.write(&signal.to_be_bytes()).unwrap();

            match computer.run_program(&input[..]) {
                ReturnMode::Output(out) => {
                    signal = out;
                }
                ReturnMode::Halt => {
                    *running = false;
                }
            }
        }
    }
}

pub fn part2(ram: &Vec<i32>) -> (Vec<i32>, i32) {
    let phases: Vec<&i32> = vec![&5, &6, &7, &8, &9];
    let phases = [&phases[..]];

    let mut permutator = Permutator::new(&phases[..]);

    let mut max_phases = vec![];
    let mut max = 0;
    while let Some(phases) = permutator.next() {
        let phases: Vec<_> = phases.iter().map(|x| **x).collect();

        let set: HashSet<_> = phases.iter().collect();
        if set.len() < 5 {
            continue;
        }

        let cur = part2_inner(&phases, &ram);
        if cur > max {
            max = cur;
            max_phases = phases
        }
    }
    (max_phases, max)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_sample<'a>(input: &'a str) -> Vec<i32> {
        input
            .trim()
            .split(",")
            .map(|part| part.parse::<i32>().unwrap())
            .collect()
    }

    #[test]
    fn part1_sample1() {
        let ram = read_sample("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
        let expected = (vec![4, 3, 2, 1, 0], 43210);
        assert_eq!(expected, part1(&ram));
    }

    #[test]
    fn part1inner_sample2() {
        let ram =
            read_sample("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0");
        let phases = vec![0, 1, 2, 3, 4];
        assert_eq!(54321, part1_inner(&phases, &ram));
    }

    #[test]
    fn part1_sample2() {
        let ram =
            read_sample("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0");
        let expected = (vec![0, 1, 2, 3, 4], 54321);
        assert_eq!(expected, part1(&ram));
    }

    #[test]
    fn part1_sample3() {
        let ram = read_sample("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0");
        let expected = (vec![1, 0, 4, 3, 2], 65210);
        assert_eq!(expected, part1(&ram));
    }

    #[test]
    fn part1_input() {
        let input = util::read_input_file("day7.txt");
        let ram = read_input(&input[..]);

        let expected = (vec![0, 2, 4, 3, 1], 21000);
        assert_eq!(expected, part1(&ram));
    }

    #[test]
    fn part2_sample1() {
        let ram = read_sample(
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
        );
        let expected = (vec![9, 8, 7, 6, 5], 139629729);
        assert_eq!(expected, part2(&ram));
    }

    #[test]
    fn part2_input() {
        let input = util::read_input_file("day7.txt");
        let ram = read_input(&input[..]);

        let expected = (vec![6, 7, 9, 8, 5], 61379886);
        assert_eq!(expected, part2(&ram));
    }
}
