use std::ops::Range;

fn has_adjecent_digit(password: [usize; 6]) -> bool {
    let mut num = password[0];

    for n in &password[1..] {
        if num == *n {
            return true;
        }

        num = *n;
    }

    false
}

fn min_adjacent_digits(password: [usize; 6]) -> usize {
    let mut min_adjacent_digits = 0;
    let mut cur_adjacent_digits = 1;
    let mut num = password[0];

    for n in &password[1..] {
        if num == *n {
            cur_adjacent_digits += 1;
        } else {
            if cur_adjacent_digits > 1 {
                if min_adjacent_digits == 0 || cur_adjacent_digits < min_adjacent_digits {
                    min_adjacent_digits = cur_adjacent_digits;
                }
            }

            cur_adjacent_digits = 1;
        }

        num = *n;
    }

    if cur_adjacent_digits > 1 {
        if min_adjacent_digits == 0 || cur_adjacent_digits < min_adjacent_digits {
            min_adjacent_digits = cur_adjacent_digits;
        }
    }

    min_adjacent_digits
}

fn always_increases(password: [usize; 6]) -> bool {
    let mut num = password[0];

    for n in &password[1..] {
        if num > *n {
            return false;
        }

        num = *n;
    }

    true
}

pub fn part1() -> usize {
    let mut matches = 0;

    let mut candidate: [usize; 6] = [1, 5, 2, 0, 8, 5];

    while candidate <= [6, 7, 0, 2, 8, 3] {
        if has_adjecent_digit(candidate) && always_increases(candidate) {
            matches += 1;
        }

        let mut pos = 5;
        loop {
            if candidate[pos] < 9 {
                candidate[pos] += 1;
                break;
            }

            candidate[pos] = 0;
            pos -= 1;
        }
    }

    matches
}

pub fn part2() -> usize {
    let mut matches = 0;

    let mut candidate: [usize; 6] = [1, 5, 2, 0, 8, 5];

    while candidate <= [6, 7, 0, 2, 8, 3] {
        if min_adjacent_digits(candidate) == 2 && always_increases(candidate) {
            matches += 1;
        }

        let mut pos = 5;
        loop {
            if candidate[pos] < 9 {
                candidate[pos] += 1;
                break;
            }

            candidate[pos] = 0;
            pos -= 1;
        }
    }

    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_adjacent_digits2() {
        assert_eq!(2, min_adjacent_digits([1, 1, 1, 1, 2, 2]));
    }

    #[test]
    fn test_min_adjacent_digits() {
        assert_eq!(3, min_adjacent_digits([1, 2, 3, 4, 4, 4]));
    }

    #[test]
    fn test_part1() {
        assert_eq!(1764, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(1196, part2());
    }
}
