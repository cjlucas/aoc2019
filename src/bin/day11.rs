use aoc2019::day11;
use aoc2019::util;

fn main() {
    let ram = util::read_input_file("day11.txt");
    day11::part1(&ram[..]);
    day11::part2(&ram[..]);
}
