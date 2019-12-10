use crate::util;
use std::collections::HashSet;
use std::io::Read;

fn read_input() -> Vec<Vec<char>> {
    let input = util::read_input_file("day10.txt");
    let mut input = &input[..];
    let mut s = String::new();

    input.read_to_string(&mut s).unwrap();

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
