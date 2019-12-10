use crate::util;
use std::collections::HashSet;
use std::io::Read;

pub fn read_input() -> Vec<Vec<char>> {
    let input = util::read_input_file("day10.txt");
    parse_input(&input[..])
}

fn parse_input(mut r: impl Read) -> Vec<Vec<char>> {
    let mut s = String::new();

    r.read_to_string(&mut s).unwrap();

    s.lines().map(|line| line.chars().collect()).collect()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn distance(&self, other: &Point) -> usize {
        (other.x as i64 - self.x as i64).abs() as usize
            + (other.y as i64 - self.y as i64).abs() as usize
    }

    fn direction(&self, other: &Point) -> f64 {
        let dx = other.x as i64 - self.x as i64;
        let dy = other.y as i64 - self.y as i64;

        let mut angle = (dx as f64).atan2(-dy as f64).to_degrees();

        if angle < 0.0 {
            angle += 360f64;
        }

        angle
    }
}

fn build_map(input: Vec<Vec<char>>) -> HashSet<Point> {
    let mut map = HashSet::new();

    for (i, row) in input.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            if *val == '#' {
                map.insert(Point { x: j, y: i });
            }
        }
    }

    map
}

fn find_ideal_pos(map: &HashSet<Point>) -> (Point, usize) {
    let mut max: (Point, usize) = (Point { x: 0, y: 0 }, 0);
    for obj in map {
        let mut others: Vec<_> = map.iter().cloned().collect();
        others.sort_by(|a, b| {
            let d1 = obj.distance(a);
            let d2 = obj.distance(b);

            d1.cmp(&d2)
        });

        let mut slopes = HashSet::<i64>::new();

        assert!(*obj == others[0]);

        let mut visible = 0;
        for other in &others[1..] {
            let d = obj.direction(other);

            let thing = (d * 1_000_000_f64) as i64;
            if !slopes.contains(&thing) {
                visible += 1;
                slopes.insert(thing);
            }
        }

        if visible > max.1 {
            max = (obj.clone(), visible);
        }
    }

    max
}

pub fn part1(input: Vec<Vec<char>>) -> usize {
    let map = build_map(input);
    find_ideal_pos(&map).1
}

pub fn part2(input: Vec<Vec<char>>) -> usize {
    let map = build_map(input);
    let maxobj = find_ideal_pos(&map).0;

    let mut points = Vec::<(f64, usize, Point)>::new();

    for other in &map {
        let dir = maxobj.direction(other);
        let dist = maxobj.distance(other);

        points.push((dir, dist, other.clone()));
    }

    points.sort_unstable_by(|a, b| {
        let (angle_a, dista, _) = a;
        let (angle_b, distb, _) = b;

        (*angle_a, *dista).partial_cmp(&(*angle_b, *distb)).unwrap()
    });

    let mut destroyed = HashSet::<Point>::new();

    let mut last_angle = 360f64;
    loop {
        for (ang, _, point) in &points {
            if *point == maxobj {
                continue;
            }

            if (last_angle - ang).abs() < 0.000001 {
                continue;
            }

            if destroyed.contains(point) {
                continue;
            }

            destroyed.insert(point.clone());
            last_angle = *ang;

            if destroyed.len() == 200 {
                return (point.x * 100) + point.y;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::util;

    #[test]
    fn part1_input() {
        let input = util::read_input_file("day10.txt");
        let input = parse_input(&input[..]);
        assert_eq!(263, part1(input));
    }

    #[test]
    fn part2_input() {
        let input = util::read_input_file("day10.txt");
        let input = parse_input(&input[..]);
        assert_eq!(1110, part2(input));
    }
}
