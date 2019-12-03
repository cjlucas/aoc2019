use aoc2019::day3;

fn main() {
    let input = day3::parse_input();
    let refs = input
        .iter()
        .map(|v| v.iter().map(String::as_ref).collect())
        .collect();

    println!("part1: {}", day3::part1(refs));

    let refs = input
        .iter()
        .map(|v| v.iter().map(String::as_ref).collect())
        .collect();
    println!("part2: {}", day3::part2(refs));
}
