use itertools::iproduct;
use num::Integer;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
struct Axis {
    position: i64,
    velocity: i64,
}

impl Axis {
    fn new(position: i64) -> Self {
        Self {
            position,
            velocity: 0,
        }
    }

    fn apply_gravity(&mut self, other_position: i64) {
        self.velocity = self.velocity + Self::calc_velocity_diff(self.position, other_position);
    }

    fn apply_velocity(&mut self) {
        self.position += self.velocity;
    }

    fn calc_velocity_diff(ours: i64, theirs: i64) -> i64 {
        if ours > theirs {
            -1
        } else if ours < theirs {
            1
        } else {
            0
        }
    }
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
struct Moon {
    x: Axis,
    y: Axis,
    z: Axis,
}

impl Moon {
    fn new(position: (i64, i64, i64)) -> Self {
        Self {
            x: Axis::new(position.0),
            y: Axis::new(position.1),
            z: Axis::new(position.2),
        }
    }

    fn potential_energy(&self) -> usize {
        self.x.position.abs() as usize
            + self.y.position.abs() as usize
            + self.z.position.abs() as usize
    }

    fn kinetic_energy(&self) -> usize {
        self.x.velocity.abs() as usize
            + self.y.velocity.abs() as usize
            + self.z.velocity.abs() as usize
    }

    fn total_energy(&self) -> usize {
        self.potential_energy() * self.kinetic_energy()
    }

    fn apply_velocity(&mut self) {
        self.x.apply_velocity();
        self.y.apply_velocity();
        self.z.apply_velocity();
    }

    fn apply_gravity(&mut self, other_position: (i64, i64, i64)) {
        self.x.apply_gravity(other_position.0);
        self.y.apply_gravity(other_position.1);
        self.z.apply_gravity(other_position.2);
    }
}

pub fn part1() -> usize {
    let input = [(14, 9, 14), (9, 11, 6), (-6, 14, -4), (4, -4, -3)];

    let mut moons: Vec<_> = input.iter().map(|position| Moon::new(*position)).collect();

    for _ in 0..1000 {
        for (a, b) in iproduct!(0..moons.len(), 0..moons.len()) {
            if a == b {
                continue;
            }

            let moon = &moons[b];
            let foo = (moon.x.position, moon.y.position, moon.z.position);
            moons[a].apply_gravity(foo);
        }

        for moon in &mut moons {
            moon.apply_velocity();
        }
    }

    moons.iter().map(|moon| moon.total_energy()).sum()
}

// Needed help on this one. The theorem behind this:
//
// Let S be the set of all states, and F: S -> S be the mapping from one state of the moons to the
// next (working as described in the problem statement). Notice that F is a bijection since we can
// easily calculate the inverse (the previous state from a state). Suppose F has a cycle (or the
// problem would not be solvable). Then the first repeating state must be the initial state,
// otherwise, F would not be one-to-one. Hence, F is periodic.

// The key is to notice that we can split F into axis components Fx, Fy, Fz, since a state of an
// axis is independent of states of all the other axes. Then the period of F is the largest common
// multiple of the periods of Fx, Fy, and Fz. So we just have to find independently the periods of
// Fx, Fy, and Fz which are hopefully much shorter than the period of F, and indeed they are shorter.
pub fn part2() -> usize {
    let input = [(14, 9, 14), (9, 11, 6), (-6, 14, -4), (4, -4, -3)];

    let moons: Vec<_> = input.iter().map(|position| Moon::new(*position)).collect();

    let x = calc_cycle(moons.iter().map(|moon| moon.x.clone()).collect());
    let y = calc_cycle(moons.iter().map(|moon| moon.y.clone()).collect());
    let z = calc_cycle(moons.iter().map(|moon| moon.z.clone()).collect());

    x.lcm(&y).lcm(&z)
}

fn calc_cycle(mut axes: Vec<Axis>) -> usize {
    let initial = axes.clone();

    for step in 1.. {
        for (a, b) in iproduct!(0..axes.len(), 0..axes.len()) {
            if a == b {
                continue;
            }

            let other_pos = axes[b].position;
            axes[a].apply_gravity(other_pos);
        }

        for axis in &mut axes {
            axis.apply_velocity();
        }

        if axes == initial {
            return step;
        }
    }

    unreachable!()
}
