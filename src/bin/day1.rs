use aoc2019::day1;
use aoc2019::util;

fn main() {
    let input = util::read_stdin_numbers();

    println!("part1");
    day1::part1(&input);

    println!("");
    println!("part2");
    day1::part2(&input);
}
