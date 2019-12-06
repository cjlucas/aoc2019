use std::collections::HashMap;

fn build_orbit_graph(input: &Vec<String>) -> HashMap<String, String> {
    input
        .iter()
        .map(|line| {
            let mut split = line.splitn(2, ')');

            let orbitee = split.nth(0).unwrap().to_string();
            let orbiter = split.nth(0).unwrap().to_string();

            (orbiter, orbitee)
        })
        .collect()
}

pub fn part1(input: &Vec<String>) -> usize {
    let orbits = build_orbit_graph(input);
    let mut num_orbits = 0;

    for key in orbits.keys() {
        let mut curkey = key;
        while let Some(key) = orbits.get(curkey) {
            num_orbits += 1;
            curkey = key;
        }
    }
    num_orbits
}

pub fn part2(input: &Vec<String>) -> usize {
    let orbits = build_orbit_graph(input);

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
    use super::super::util;
    use super::*;

    fn read_input() -> Vec<String> {
        let input = util::read_input_file("day6.txt");
        String::from_utf8(input)
            .unwrap()
            .lines()
            .map(|line| line.trim().to_string())
            .collect()
    }

    #[test]
    fn part1_sample() {
        let input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\n"
            .lines()
            .map(|line| line.trim())
            .map(|line| line.to_string())
            .collect();

        assert_eq!(42, part1(&input));
    }

    #[test]
    fn part1_input() {
        assert_eq!(241064, part1(&read_input()));
    }

    #[test]
    fn part2_sample() {
        let input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN\n"
            .lines()
            .map(|line| line.trim())
            .map(|line| line.to_string())
            .collect();

        assert_eq!(4, part2(&input));
    }

    #[test]
    fn part2_input() {
        assert_eq!(418, part2(&read_input()));
    }
}
