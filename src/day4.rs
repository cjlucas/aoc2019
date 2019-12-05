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

pub fn part1() -> usize {
    PasswordIterator::new([1, 5, 2, 0, 8, 5], [6, 7, 0, 2, 8, 3])
        .filter(|pw| {
            pw[0] == pw[1] || pw[1] == pw[2] || pw[2] == pw[3] || pw[3] == pw[4] || pw[4] == pw[5]
        })
        .count()
}

pub fn part2() -> usize {
    PasswordIterator::new([1, 5, 2, 0, 8, 5], [6, 7, 0, 2, 8, 3])
        .filter(|pw| {
            (pw[0] == pw[1] && pw[1] != pw[2])
                || (pw[0] != pw[1] && pw[1] == pw[2] && pw[2] != pw[3])
                || (pw[1] != pw[2] && pw[2] == pw[3] && pw[3] != pw[4])
                || (pw[2] != pw[3] && pw[3] == pw[4] && pw[4] != pw[5])
                || (pw[3] != pw[4] && pw[4] == pw[5])
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(1764, part1());
    }

    #[test]
    fn test_part2() {
        assert_eq!(1196, part2());
    }
}
