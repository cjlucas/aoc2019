use crate::util;
use std::collections::{HashMap, HashSet};
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

#[derive(Debug)]
enum Direction {
    // TODO: rename segment
    Up(i32),
    Right(i32),
    Down(i32),
    Left(i32),
}

impl Direction {
    fn length(&self) -> i32 {
        match *self {
            Direction::Up(n) => n,
            Direction::Down(n) => n,
            Direction::Left(n) => n,
            Direction::Right(n) => n,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn origin() -> Self {
        Self { x: 0, y: 0 }
    }
}

#[derive(Debug)]
struct Wire {
    path: Vec<Direction>,
}

impl Wire {
    fn new(path: impl Iterator<Item = Direction>) -> Self {
        Self {
            path: path.collect(),
        }
    }

    fn steps_from_central_point(&self, point: &Point) -> i32 {
        let mut steps = 0;

        let mut current_point = Point::origin();
        for segment in &self.path {
            let next_point = match segment {
                Direction::Up(dist) => Point {
                    x: current_point.x,
                    y: current_point.y + dist,
                },
                Direction::Right(dist) => Point {
                    x: current_point.x + dist,
                    y: current_point.y,
                },
                Direction::Down(dist) => Point {
                    x: current_point.x,
                    y: current_point.y - dist,
                },
                Direction::Left(dist) => Point {
                    x: current_point.x - dist,
                    y: current_point.y,
                },
            };

            if point.x == current_point.x && (current_point.y..=next_point.y).contains(&point.y) {
                println!("hit it {:?}", segment);
                steps += (point.y.abs() - current_point.y.abs()).abs();
                break;
            }

            if point.y == current_point.y && (current_point.x..=next_point.x).contains(&point.x) {
                println!("hit it {:?}", segment);
                steps += (point.x.abs() - current_point.x.abs()).abs();
                break;
            }

            current_point = next_point;
            steps += segment.length();
        }

        println!("{:?}", point);
        println!("{:?}", self.path);
        println!("{:?}", steps);
        println!();

        steps
    }
}

#[derive(Default)]
struct FrontPanel {
    wires: Vec<Wire>,
}

impl FrontPanel {
    pub fn add_wire(&mut self, wire: Wire) {
        self.wires.push(wire);
    }

    fn intersection_points(&self) -> HashSet<Point> {
        let sets: Vec<HashSet<Point>> = self
            .wires
            .iter()
            .map(|wire| {
                let mut points = HashSet::new();
                let mut last_point = Point { x: 0, y: 0 };

                for direction in &wire.path {
                    match *direction {
                        Direction::Up(dist) => {
                            for _ in 0..dist {
                                last_point.y += 1;
                                points.insert(last_point);
                            }
                        }
                        Direction::Right(dist) => {
                            for _ in 0..dist {
                                last_point.x += 1;
                                points.insert(last_point);
                            }
                        }
                        Direction::Down(dist) => {
                            for _ in 0..dist {
                                last_point.y -= 1;
                                points.insert(last_point);
                            }
                        }
                        Direction::Left(dist) => {
                            for _ in 0..dist {
                                last_point.x -= 1;
                                points.insert(last_point);
                            }
                        }
                    }
                }
                points
            })
            .collect();

        let mut candidate_points = match sets.first() {
            Some(first) => first.clone(),
            None => return HashSet::new(),
        };

        for set in &sets[1..] {
            candidate_points = set.intersection(&candidate_points).cloned().collect();
        }

        candidate_points
    }

    fn distance_from_central_point(p: &Point) -> i32 {
        p.x.abs() + p.y.abs()
    }
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
    let mut panel = FrontPanel::default();

    for line in lines {
        let path = line.iter().map(|s| parse_direction(&s));
        panel.add_wire(Wire::new(path));
    }

    panel
        .intersection_points()
        .iter()
        .map(FrontPanel::distance_from_central_point)
        .min()
        .unwrap()
}

pub fn part2(lines: &Vec<Vec<String>>) -> i32 {
    let mut panel = FrontPanel::default();

    for line in lines {
        let path = line.iter().map(|s| parse_direction(&s));
        panel.add_wire(Wire::new(path));
    }

    panel
        .intersection_points()
        .iter()
        .map(|point| {
            panel
                .wires
                .iter()
                .map(|wire| wire.steps_from_central_point(&point))
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
