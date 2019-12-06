use itertools::Itertools;
use std::collections::HashMap;

pub fn part1(input: Vec<String>) -> usize {
    let mut orbits: HashMap<String, String> = input
        .iter()
        .map(|line| {
            let mut split = line.splitn(2, ')');

            let orbitee = split.nth(0).unwrap().to_string();
            let orbiter = split.nth(0).unwrap().to_string();

            (orbiter, orbitee)
        })
        .collect();

    let mut num_orbits = 0;

    for key in orbits.keys() {
        println!("{:?}", key);
        let mut curkey = key;
        while let Some(key) = orbits.get(curkey) {
            println!("here");
            num_orbits += 1;
            curkey = key;
        }
    }

    println!("{:?}", orbits);
    num_orbits
}

pub fn part2(input: Vec<String>) -> usize {
    let mut orbits: HashMap<String, String> = input
        .iter()
        .map(|line| {
            let mut split = line.splitn(2, ')');

            let orbitee = split.nth(0).unwrap().to_string();
            let orbiter = split.nth(0).unwrap().to_string();

            (orbiter, orbitee)
        })
        .collect();

    let mut you_path = Vec::new();
    let mut curkey = &"YOU".to_string();
    while let Some(val) = orbits.get(curkey) {
        you_path.push(val.clone());
        curkey = val;
    }

    let mut santa_path = Vec::new();
    let mut curkey = &"SAN".to_string();
    while let Some(val) = orbits.get(curkey) {
        santa_path.push(val.clone());
        curkey = val;
    }

    for (cur, i) in you_path.iter().enumerate() {
        let mut cur = cur;

        for j in &santa_path {
            if i == j {
                println!("FOUND a common thing: {}", i);
                return cur;
            }

            cur += 1;
        }
    }

    unreachable!("couldn't find a common ancestor")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        let input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\n"
            .lines()
            .map(|line| line.trim())
            .map(|line| line.to_string())
            .collect();

        assert_eq!(42, part1(input));
    }

    #[test]
    fn part2_sample() {
        let input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN\n"
            .lines()
            .map(|line| line.trim())
            .map(|line| line.to_string())
            .collect();

        assert_eq!(4, part2(input));
    }
}
