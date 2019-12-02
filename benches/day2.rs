#![feature(test)]
extern crate test;

use aoc2019::day2;
use aoc2019::util;
use test::Bencher;

#[bench]
fn bench_part2(b: &mut Bencher) {
    let input = util::read_input_file("day2.txt");
    let input = day2::read_input(&input[..]);

    b.iter(|| day2::part2(input.clone()));
}
