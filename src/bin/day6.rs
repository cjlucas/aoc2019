use aoc2019::day6;
use aoc2019::util;

fn main() {
    let input = util::read_input_file("day6.txt");
    let input: Vec<String> = String::from_utf8(input)
        .unwrap()
        .lines()
        .map(|line| line.trim().to_string())
        .collect();

    println!("part1: {}", day6::part1(&input));
    println!("part2: {}", day6::part2(&input));
}
