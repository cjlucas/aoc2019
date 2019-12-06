use std::collections::HashMap;

#[derive(Default)]
struct UniversalOrbitMap {
    orbits: HashMap<String, String>,
}

impl UniversalOrbitMap {
    pub fn add_orbit(&mut self, orbiter: String, orbitee: String) {
        self.orbits.insert(orbiter, orbitee);
    }

    pub fn path_to_center_of_mass(&self, object_name: String) -> Vec<String> {
        let mut curkey = &object_name;
        let mut path = Vec::new();

        while let Some(val) = self.orbits.get(curkey) {
            path.push(val.clone());
            curkey = val;
        }

        path
    }

    pub fn orbiters(&self) -> impl Iterator<Item = &String> {
        self.orbits.keys()
    }
}

fn build_orbit_map(input: &Vec<String>) -> UniversalOrbitMap {
    let mut map = UniversalOrbitMap::default();

    input.iter().for_each(|line| {
        let mut split = line.splitn(2, ')');

        let orbitee = split.nth(0).unwrap().to_string();
        let orbiter = split.nth(0).unwrap().to_string();

        map.add_orbit(orbiter, orbitee);
    });

    map
}

pub fn part1(input: &Vec<String>) -> usize {
    let orbits = build_orbit_map(input);
    orbits
        .orbiters()
        .map(|x| orbits.path_to_center_of_mass(x.to_string()))
        .map(|x| x.len())
        .sum()
}

pub fn part2(input: &Vec<String>) -> usize {
    let map = build_orbit_map(input);

    let you_path = map.path_to_center_of_mass("YOU".to_string());
    let santa_path = map.path_to_center_of_mass("SAN".to_string());

    for (steps_from_you, i) in you_path.iter().enumerate() {
        for (steps_from_santa, j) in santa_path.iter().enumerate() {
            if i == j {
                return steps_from_you + steps_from_santa;
            }
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
