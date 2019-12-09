use crate::util;
use std::collections::HashSet;
use std::io::BufRead;

pub fn parse_input() -> Vec<Vec<String>> {
    let mut out = Vec::new();

    let mut input = &util::read_input_file("day3.txt")[..];

    let mut line = String::new();
    while let Ok(n) = input.read_line(&mut line) {
        if n == 0 {
            break;
        }

        let split: Vec<String> = line.trim().split(",").map(|x| x.to_owned()).collect();
        out.push(split);

        line = String::new();
    }

    out
}

enum Direction {
    Up(i32),
    Right(i32),
    Down(i32),
    Left(i32),
}

fn parse_direction<'a>(s: &'a str) -> Direction {
    let (dir, dist) = s.split_at(1);

    let dist = dist.parse::<i32>().unwrap();

    match dir {
        "U" => Direction::Up(dist),
        "R" => Direction::Right(dist),
        "D" => Direction::Down(dist),
        "L" => Direction::Left(dist),
        _ => unreachable!("unknown direction: {}", dir),
    }
}

pub fn part1(lines: &Vec<Vec<String>>) -> i32 {
    let mut coords: Vec<HashSet<(i32, i32)>> = Vec::new();

    for line in lines {
        let mut last_coord = (0, 0);
        let mut points = HashSet::new();

        for dir in line {
            match parse_direction(&dir) {
                Direction::Up(dist) => {
                    for _ in 0..dist {
                        last_coord = (last_coord.0, last_coord.1 + 1);
                        points.insert(last_coord);
                    }
                }
                Direction::Right(dist) => {
                    for _ in 0..dist {
                        last_coord = (last_coord.0 + 1, last_coord.1);
                        points.insert(last_coord);
                    }
                }
                Direction::Down(dist) => {
                    for _ in 0..dist {
                        last_coord = (last_coord.0, last_coord.1 - 1);
                        points.insert(last_coord);
                    }
                }
                Direction::Left(dist) => {
                    for _ in 0..dist {
                        last_coord = (last_coord.0 - 1, last_coord.1);
                        points.insert(last_coord);
                    }
                }
            }
        }

        coords.push(points);
    }

    let mut common = coords[0].clone();

    for set in &coords[1..] {
        common = common.intersection(set).cloned().collect();
    }

    let min = common
        .iter()
        .cloned()
        .min_by(|(x1, y1), (x2, y2)| (x1.abs() + y1.abs()).cmp(&(x2.abs() + y2.abs())))
        .unwrap();

    min.0.abs() + min.1.abs()
}

pub fn part2(lines: &Vec<Vec<String>>) -> usize {
    let mut coords: Vec<Vec<(i32, i32)>> = Vec::new();

    for line in lines {
        let mut last_coord = (0, 0);
        let mut points = Vec::new();

        for dir in line {
            match parse_direction(&dir) {
                Direction::Up(dist) => {
                    for _ in 0..dist {
                        last_coord = (last_coord.0, last_coord.1 + 1);
                        points.push(last_coord);
                    }
                }
                Direction::Right(dist) => {
                    for _ in 0..dist {
                        last_coord = (last_coord.0 + 1, last_coord.1);
                        points.push(last_coord);
                    }
                }
                Direction::Down(dist) => {
                    for _ in 0..dist {
                        last_coord = (last_coord.0, last_coord.1 - 1);
                        points.push(last_coord);
                    }
                }
                Direction::Left(dist) => {
                    for _ in 0..dist {
                        last_coord = (last_coord.0 - 1, last_coord.1);
                        points.push(last_coord);
                    }
                }
            }
        }

        coords.push(points);
    }

    let mut common: HashSet<(i32, i32)> = coords[0].iter().cloned().collect();

    for steps in &coords[1..] {
        common = steps
            .iter()
            .cloned()
            .collect::<HashSet<(i32, i32)>>()
            .intersection(&common)
            .cloned()
            .collect();
    }

    common
        .iter()
        .map(|point| {
            coords
                .iter()
                .map(|coord| coord.iter().position(|p| point == p).unwrap() + 1)
                .sum()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_line<'a>(s: &'a str) -> Vec<String> {
        s.split(',').map(|s| s.to_string()).collect()
    }

    #[test]
    fn part1_example1() {
        let input = vec![
            sample_line("R75,D30,R83,U83,L12,D49,R71,U7,L72"),
            sample_line("U62,R66,U55,R34,D71,R55,D58,R83"),
        ];

        assert_eq!(part1(&input), 159);
    }

    #[test]
    fn part1_example2() {
        let input = vec![
            sample_line("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
            sample_line("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"),
        ];

        assert_eq!(part1(&input), 135);
    }

    #[test]
    fn part1_real() {
        let input = parse_input();

        assert_eq!(part1(&input), 860);
    }

    #[test]
    fn part2_example1() {
        let input = vec![
            sample_line("R75,D30,R83,U83,L12,D49,R71,U7,L72"),
            sample_line("U62,R66,U55,R34,D71,R55,D58,R83"),
        ];

        assert_eq!(part2(&input), 610);
    }

    #[test]
    fn part2_example2() {
        let input = vec![
            sample_line("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
            sample_line("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"),
        ];

        assert_eq!(part2(&input), 410);
    }

    #[test]
    fn part2_real() {
        let input = parse_input();

        assert_eq!(part2(&input), 9238);
    }
}
