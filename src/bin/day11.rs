use aoc2019::day11;
use aoc2019::util;

fn main() {
    let ram = util::read_input_file("day11.txt");
    println!("part1: {}", day11::part1(&ram[..]));
    println!("part2:");
    day11::part2(&ram[..]);
}
