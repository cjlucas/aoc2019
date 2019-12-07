#![feature(test)]
extern crate test;

use aoc2019::{day6, util};
use test::Bencher;

#[bench]
fn day6part1(b: &mut Bencher) {
    let input = util::read_input_file("day6.txt");
    let input: Vec<String> = String::from_utf8(input)
        .unwrap()
        .lines()
        .map(|line| line.trim().to_string())
        .collect();

    b.iter(|| day6::part1(&input));
}

#[bench]
fn day6part2(b: &mut Bencher) {
    let input = util::read_input_file("day6.txt");
    let input: Vec<String> = String::from_utf8(input)
        .unwrap()
        .lines()
        .map(|line| line.trim().to_string())
        .collect();

    b.iter(|| day6::part2(&input));
}
