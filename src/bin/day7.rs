use aoc2019::day7;
use aoc2019::util;

fn main() {
    let input = util::read_input_file("day7.txt");
    let ram = day7::read_input(&input[..]);

    println!("part1: {:?}", day7::part1(&ram));
    println!("part2: {:?}", day7::part2(&ram));
}
