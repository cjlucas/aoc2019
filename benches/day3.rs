#![feature(test)]
extern crate test;

use aoc2019::day3;
use aoc2019::util;
use test::Bencher;

#[bench]
fn bench_part1(b: &mut Bencher) {
    let input = day3::parse_input();
    b.iter(|| day3::part1(&input));
}

#[bench]
fn bench_part2(b: &mut Bencher) {
    let input = day3::parse_input();

    b.iter(|| day3::part2(&input));
}
