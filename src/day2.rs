use std::io;
use std::io::Read;

enum Op {
    Add(usize, usize, usize),
    Mul(usize, usize, usize),
    Halt,
}

struct Computer<'a> {
    ram: &'a mut [i32],
    inst_ptr: usize,
}

impl<'a> Computer<'a> {
    pub fn new(ram: &'a mut [i32]) -> Self {
        Self { ram, inst_ptr: 0 }
    }

    pub fn run_program(&'a mut self) {
        loop {
            let op = self.read_op();

            match op {
                Ok(Op::Add(a, b, out)) => {
                    self.ram[out] = self.ram[a] + self.ram[b];
                }
                Ok(Op::Mul(a, b, out)) => {
                    self.ram[out] = self.ram[a] * self.ram[b];
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
        let op = match self.ram[self.inst_ptr] {
            1 => {
                if self.ram[self.inst_ptr..].len() < 4 {
                    return Err(io::Error::new(io::ErrorKind::UnexpectedEof, ""));
                }

                Op::Add(
                    self.ram[self.inst_ptr + 1] as usize,
                    self.ram[self.inst_ptr + 2] as usize,
                    self.ram[self.inst_ptr + 3] as usize,
                )
            }
            2 => {
                if self.ram[self.inst_ptr..].len() < 4 {
                    return Err(io::Error::new(io::ErrorKind::UnexpectedEof, ""));
                }

                Op::Mul(
                    self.ram[self.inst_ptr + 1] as usize,
                    self.ram[self.inst_ptr + 2] as usize,
                    self.ram[self.inst_ptr + 3] as usize,
                )
            }
            99 => Op::Halt,
            n => panic!("unknown op code {}", n),
        };

        self.inst_ptr += match op {
            Op::Add(_, _, _) | Op::Mul(_, _, _) => 4,
            Op::Halt => 0,
        };

        Ok(op)
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

pub fn run_program(mut ram: Vec<i32>, input1: i32, input2: i32) -> i32 {
    ram[1] = input1;
    ram[2] = input2;

    let mut computer = Computer::new(ram.as_mut_slice());
    computer.run_program();

    ram[0]
}

pub fn part1(input: Vec<i32>) -> i32 {
    run_program(input, 12, 2)
}

const TARGET: i32 = 19690720;

pub fn part2(input: Vec<i32>) -> (i32, i32) {
    for noun in 0..=99 {
        for verb in 0..=99 {
            if run_program(input.clone(), noun, verb) == TARGET {
                return (noun, verb);
            }
        }
    }

    unreachable!("failed to solve")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn test_part1() {
        let input = util::read_input_file("day2.txt");
        let input = read_input(&input[..]);

        assert_eq!(6327510, part1(input));
    }

    #[test]
    fn test_part2() {
        let input = util::read_input_file("day2.txt");
        let input = read_input(&input[..]);

        assert_eq!((41, 12), part2(input));
    }
}
