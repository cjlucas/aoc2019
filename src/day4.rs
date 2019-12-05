use std::iter::Iterator;

type Password = [usize; 6];

struct PasswordIterator {
    end: Password,
    current: Password,
}

impl PasswordIterator {
    pub fn new(start: Password, end: Password) -> Self {
        let mut current = start.clone();
        Self::next_always_increasing_number(&mut current);

        Self { end, current }
    }

    fn next_always_increasing_number(password: &mut Password) {
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
}

impl Iterator for PasswordIterator {
    type Item = Password;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        if current > self.end {
            return None;
        }

        let mut pos = 5;
        loop {
            if self.current[pos] < 9 {
                self.current[pos] += 1;
                break;
            }

            self.current[pos] = 0;
            pos -= 1;
        }

        for i in pos + 1..6 {
            self.current[i] = self.current[pos];
        }

        Some(current)
    }
}

fn min_adjacent_digits(password: &Password) -> usize {
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

pub fn part1() -> usize {
    PasswordIterator::new([1, 5, 2, 0, 8, 5], [6, 7, 0, 2, 8, 3])
        .filter(|candidate| min_adjacent_digits(candidate) > 1)
        .count()
}

pub fn part2() -> usize {
    PasswordIterator::new([1, 5, 2, 0, 8, 5], [6, 7, 0, 2, 8, 3])
        .filter(|candidate| min_adjacent_digits(candidate) == 2)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_adjacent_digits2() {
        assert_eq!(2, min_adjacent_digits(&[1, 1, 1, 1, 2, 2]));
    }

    #[test]
    fn test_min_adjacent_digits() {
        assert_eq!(3, min_adjacent_digits(&[1, 2, 3, 4, 4, 4]));
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
