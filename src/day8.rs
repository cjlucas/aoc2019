use crate::util;

use std::io::Read;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn layers<'a>(chars: &'a Vec<char>) -> impl Iterator<Item = &'a [char]> {
    chars.as_slice().chunks_exact(WIDTH * HEIGHT)
}

fn read_input() -> Vec<char> {
    let input = util::read_input_file("day8.txt");
    let mut s = String::new();
    input.as_slice().read_to_string(&mut s).unwrap();
    s.trim().chars().collect()
}

fn count_of<'a, T>(into_iter: impl IntoIterator<Item = &'a T>, t: T) -> usize
where
    T: Eq + 'a,
{
    into_iter.into_iter().filter(|x| **x == t).count()
}

pub fn part1() -> usize {
    let chars = read_input();

    let layer = layers(&chars)
        .min_by(|a, b| {
            let a = count_of(*a, '0');
            let b = count_of(*b, '0');

            a.cmp(&b)
        })
        .unwrap();

    let n_ones = count_of(layer, '1');
    let n_twos = count_of(layer, '2');

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
