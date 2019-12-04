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

fn next_always_increasing_number(password: &mut [usize; 6]) {
    let mut num = password[0];

    for i in 1..6 {
        let n = password[i];

        if n < num {
            for j in i..6 {
                password[j] = num;
            }

            break;
        } else {
            num = n;
        }
    }
}

pub fn part1() -> usize {
    let mut matches = 0;

    let mut candidate: [usize; 6] = [1, 5, 2, 0, 8, 5];
    next_always_increasing_number(&mut candidate);

    while candidate <= [6, 7, 0, 2, 8, 3] {
        if min_adjacent_digits(candidate) > 1 {
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

        for i in pos + 1..6 {
            candidate[i] = candidate[pos];
        }
    }

    matches
}

pub fn part2() -> usize {
    let mut matches = 0;

    let mut candidate: [usize; 6] = [1, 5, 2, 0, 8, 5];
    next_always_increasing_number(&mut candidate);

    while candidate <= [6, 7, 0, 2, 8, 3] {
        if min_adjacent_digits(candidate) == 2 {
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

        for i in pos + 1..6 {
            candidate[i] = candidate[pos];
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
