use aoc2019::day2;

fn main() {
    let input = day2::read_input(std::io::stdin());

    println!("part1");
    println!("{}", day2::part1(input.clone()));

    println!("\npart2");
    let (noun, verb) = day2::part2(input);
    println!("{}", 100 * noun + verb);
}
