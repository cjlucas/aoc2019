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

type Point = (usize, usize);

fn distance(a: Point, b: Point) -> (i64, i64) {
    let (x1, y1) = a;
    let (x2, y2) = b;

    (x2 as i64 - x1 as i64, y2 as i64 - y1 as i64)
}

pub fn part1() {
    let input = read_input();
    let mut map = HashSet::new();

    for (i, row) in input.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            if *val == '#' {
                map.insert((i, j));
            }
        }
    }

    let mut max = 0;
    for obj in &map {
        let mut others: Vec<_> = map.iter().cloned().collect();
        others.sort_by(|a, b| {
            let (x1, y1) = distance(*obj, *a);
            let (x2, y2) = distance(*obj, *b);

            let d1 = x1.abs() + y1.abs();
            let d2 = x2.abs() + y2.abs();
            d1.cmp(&d2)
        });

        let mut slopes = HashSet::<(bool, bool, i64)>::new();

        assert!(*obj == others[0]);

        let mut visible = 0;
        for other in &others[1..] {
            let (x, y) = distance(*obj, *other);
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

    println!("{}", max);
}

pub fn part2(input: Vec<Vec<char>>) {
    let mut map = HashSet::new();

    for (i, row) in input.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            if *val == '#' {
                map.insert((j, i));
            }
        }
    }
    println!("map: {}", map.len());

    let mut maxobj = (0, 0);
    let mut max = 0;
    for obj in &map {
        let mut others: Vec<_> = map.iter().cloned().collect();
        others.sort_by(|a, b| {
            let (x1, y1) = distance(*obj, *a);
            let (x2, y2) = distance(*obj, *b);

            let d1 = x1.abs() + y1.abs();
            let d2 = x2.abs() + y2.abs();
            d1.cmp(&d2)
        });

        let mut slopes = HashSet::<(bool, bool, i64)>::new();

        assert!(*obj == others[0]);

        let mut visible = 0;
        for other in &others[1..] {
            let (x, y) = distance(*obj, *other);
            let thing = ((x as f64 / y as f64) * 1_000_000 as f64) as i64;
            let thing = (x > 0, y > 0, thing);

            if !slopes.contains(&thing) {
                visible += 1;
                slopes.insert(thing);
            }
        }

        if visible > max {
            maxobj = *obj;
            max = visible;
        }
    }
    println!("{:?}, {}", maxobj, max);

    let mut points = Vec::<(f64, u64, Point)>::new();

    let mut visible = 0;

    for other in &map {
        let (x, y) = distance(maxobj, *other);
        let d = x.abs() as u64 + y.abs() as u64;

        let mut angle = (x as f64).atan2(-y as f64).to_degrees();

        if angle < 0.0 {
            angle += 360f64;
        }

        points.push((angle, d, *other));
    }

    points.sort_unstable_by(|a, b| {
        let (angle_a, dista, _) = a;
        let (angle_b, distb, _) = b;

        (*angle_a, *dista).partial_cmp(&(*angle_b, *distb)).unwrap()
    });

    println!("{:?}", points);

    let mut destroyed = HashSet::<Point>::new();

    let mut last_angle = 360f64;
    loop {
        println!("top of loop");
        for (ang, _, point) in &points {
            if *point == maxobj {
                continue;
            }

            if (last_angle - ang).abs() < 0.000001 {
                println!("skipping {} {}", last_angle, ang);
                continue;
            }

            if destroyed.contains(point) {
                continue;
            }

            destroyed.insert(*point);
            println!("destroyed {:?}", *point);
            last_angle = *ang;

            if destroyed.len() == 200 {
                let (x, y) = *point;
                println!("{}", (x * 100) + y);
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_sample() {
        let input = parse_input(".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##".as_bytes());
        part2(input);
        assert!(false)
    }

    // #[test]
    // fn fuck() {
    //     let input = parse_input(".#..#..###\n####.###.#\n....###.#.\n..###.##.#\n##.##.#.#.\n....###..#\n..#.#..#.#\n#..#.#.###\n.##...##.#\n.....#.#..".as_bytes());
    //     part2(input);
    //     assert!(false)
    // }
}
