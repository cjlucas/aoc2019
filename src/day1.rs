fn calc_fuel(input: i32) -> i32 {
    input / 3 - 2
}

pub fn part1(input: &Vec<i32>) {
    let result: i32 = input.iter().map(|n| calc_fuel(*n)).sum();
    println!("{}", result);
}

pub fn part2(input: &Vec<i32>) {
    let result: i32 = input
        .iter()
        .map(|mass| {
            let mut fuel = calc_fuel(*mass);
            let mut total_fuel = 0;

            while fuel > 0 {
                total_fuel += fuel;
                fuel = calc_fuel(fuel);
            }

            total_fuel
        })
        .sum();

    println!("{}", result);
}
