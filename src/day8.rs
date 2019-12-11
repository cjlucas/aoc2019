use crate::util;

use std::io::Read;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

trait CountOf<'a, T: Eq + 'a> {
    fn count_of(&self, t: T) -> usize;
}

impl<'a, T> CountOf<'a, T> for &'a [T]
where
    T: Eq + 'a,
{
    fn count_of(&self, t: T) -> usize {
        self.into_iter().filter(|x| **x == t).count()
    }
}

fn layers<'a>(chars: &'a Vec<char>) -> impl Iterator<Item = &'a [char]> {
    chars.as_slice().chunks_exact(WIDTH * HEIGHT)
}

fn read_input() -> Vec<char> {
    let input = util::read_input_file("day8.txt");
    let mut s = String::new();
    input.as_slice().read_to_string(&mut s).unwrap();
    s.trim().chars().collect()
}

pub fn part1() -> usize {
    let chars = read_input();

    let layer = layers(&chars)
        .min_by(|a, b| {
            let a = a.count_of('0');
            let b = b.count_of('0');

            a.cmp(&b)
        })
        .unwrap();

    let n_ones = layer.count_of('1');
    let n_twos = layer.count_of('2');

    n_ones * n_twos
}

pub fn part2() {
    let chars = read_input();
    let mut picture = vec!['2'; WIDTH * HEIGHT];
    for layer in layers(&chars) {
        for (i, pixel) in layer.iter().enumerate() {
            if picture[i] != '2' {
                continue;
            }

            picture[i] = *pixel
        }
    }

    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            let pixel = picture[(row * WIDTH) + col];
            print!("{}", if pixel == '0' { ' ' } else { 'X' });
        }

        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_input() {
        assert_eq!(1806, part1());
    }
}
