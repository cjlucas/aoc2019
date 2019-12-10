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

fn distance(a: &Point, b: &Point) -> (i64, i64) {
    (b.x as i64 - a.x as i64, b.y as i64 - a.y as i64)
}

pub fn part1(input: Vec<Vec<char>>) -> usize {
    let mut map = HashSet::new();

    for (i, row) in input.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            if *val == '#' {
                map.insert(Point { x: j, y: i });
            }
        }
    }

    let mut max = 0;
    for obj in &map {
        let mut others: Vec<_> = map.iter().cloned().collect();
        others.sort_by(|a, b| {
            let (x1, y1) = distance(obj, a);
            let (x2, y2) = distance(obj, b);

            let d1 = x1.abs() + y1.abs();
            let d2 = x2.abs() + y2.abs();
            d1.cmp(&d2)
        });

        let mut slopes = HashSet::<(bool, bool, i64)>::new();

        assert!(*obj == others[0]);

        let mut visible = 0;
        for other in &others[1..] {
            let (x, y) = distance(obj, other);
            let thing = ((x as f64 / y as f64) * 1_000_000 as f64) as i64;
            let thing = (x > 0, y > 0, thing);

            if !slopes.contains(&thing) {
                visible += 1;
                slopes.insert(thing);
            }
        }

        if visible > max {
            max = visible;
        }
    }

    max
}

pub fn part2(input: Vec<Vec<char>>) -> usize {
    let mut map = HashSet::new();

    for (i, row) in input.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            if *val == '#' {
                map.insert(Point { x: j, y: i });
            }
        }
    }
    let mut maxobj = Point { x: 0, y: 0 };
    let mut max = 0;
    for obj in &map {
        let mut others: Vec<_> = map.iter().cloned().collect();
        others.sort_by(|a, b| {
            let (x1, y1) = distance(obj, a);
            let (x2, y2) = distance(obj, b);

            let d1 = x1.abs() + y1.abs();
            let d2 = x2.abs() + y2.abs();
            d1.cmp(&d2)
        });

        let mut slopes = HashSet::<(bool, bool, i64)>::new();

        assert!(*obj == others[0]);

        let mut visible = 0;
        for other in &others[1..] {
            let (x, y) = distance(obj, other);
            let thing = ((x as f64 / y as f64) * 1_000_000 as f64) as i64;
            let thing = (x > 0, y > 0, thing);

            if !slopes.contains(&thing) {
                visible += 1;
                slopes.insert(thing);
            }
        }

        if visible > max {
            maxobj = obj.clone();
            max = visible;
        }
    }

    let mut points = Vec::<(f64, u64, Point)>::new();

    for other in &map {
        let (x, y) = distance(&maxobj, other);
        let d = x.abs() as u64 + y.abs() as u64;

        let mut angle = (x as f64).atan2(-y as f64).to_degrees();

        if angle < 0.0 {
            angle += 360f64;
        }

        points.push((angle, d, other.clone()));
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
