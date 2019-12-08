use crate::util;

use std::io::Read;

pub fn part1() -> usize {
    let input = util::read_input_file("day8.txt");
    let mut s = String::new();
    input.as_slice().read_to_string(&mut s).unwrap();
    let chars: Vec<_> = s.trim().chars().collect();

    let layer = chars
        .as_slice()
        .chunks_exact(25 * 6)
        .min_by(|a, b| {
            let a = a.iter().filter(|x| **x == '0').count();
            let b = b.iter().filter(|x| **x == '0').count();

            a.cmp(&b)
        })
        .unwrap();

    let n_ones = layer.iter().filter(|x| **x == '1').count();
    let n_twos = layer.iter().filter(|x| **x == '2').count();

    n_ones * n_twos
}

pub fn part2() {
    let input = util::read_input_file("day8.txt");
    let mut s = String::new();
    input.as_slice().read_to_string(&mut s).unwrap();
    let chars: Vec<_> = s.trim().chars().collect();

    let mut picture = vec!['2'; 25 * 6];
    for layer in chars.chunks_exact(25 * 6) {
        for (i, pixel) in layer.iter().enumerate() {
            if picture[i] != '2' {
                continue;
            }

            picture[i] = *pixel
        }
    }

    for row in 0..6 {
        for col in 0..25 {
            let pixel = picture[(row * 25) + col];
            print!("{}", if pixel == '0' { ' ' } else { 'X' });
        }

        println!();
    }
}
