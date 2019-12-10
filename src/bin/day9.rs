use aoc2019::day9;
use aoc2019::util;

fn main() {
    let input = util::read_input_file("day9.txt");
    let ram = day9::read_input(&input[..]);
    println!("part1: {}", day9::part1(&ram));
    println!("part2: {}", day9::part2(&ram));
}
