#![feature(test)]
extern crate test;

use aoc2019::day4;
use test::Bencher;

#[bench]
fn day4part1(b: &mut Bencher) {
    b.iter(|| day4::part1());
}

#[bench]
fn day4part2(b: &mut Bencher) {
    b.iter(|| day4::part2());
}
