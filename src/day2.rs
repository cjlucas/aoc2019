use std::io;
use std::io::Read;

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

    let mut pos = 0;
    loop {
        match ram[pos] {
            1 => {
                let out = ram[pos + 3] as usize;
                ram[out] = ram[ram[pos + 1] as usize] + ram[ram[pos + 2] as usize];
            }
            2 => {
                let out = ram[pos + 3] as usize;
                ram[out] = ram[ram[pos + 1] as usize] * ram[ram[pos + 2] as usize];
            }
            99 => {
                return ram[0];
            }
            n => {
                unreachable!("unknown op code: {}", n);
            }
        }

        pos += 4;
    }
}

pub fn part1(input: Vec<i32>) -> i32 {
    run_program(input, 12, 2)
}

const PART2_TARGET: i32 = 19690720;

pub fn part2(input: Vec<i32>) -> (i32, i32) {
    for noun in 0..=99 {
        for verb in 0..=99 {
            if run_program(input.clone(), noun, verb) == PART2_TARGET {
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
