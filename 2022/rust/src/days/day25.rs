use std::ops;

pub fn part_one(input: &str) -> String {
    let mut total = Snafu::from(0);
    for line in input.lines() {
        let interpreted = Snafu::from_snafu(line);
        total += interpreted;
    }
    total.as_snafu()
}

#[derive(Debug, Clone, Copy)]
pub struct Snafu(u64);

impl From<u64> for Snafu {
    fn from(value: u64) -> Self {
        Snafu(value)
    }
}

impl Snafu {
    pub fn as_decimal(&self) -> u64 {
        self.0
    }

    pub fn as_snafu(&self) -> String {
        let place = 5;
        let mut carry = 0;
        let mut result = String::with_capacity((self.0.ilog(5) + 2) as usize);
        let mut total_so_far = self.0;

        loop {
            let mut rem = total_so_far % place;
            total_so_far -= rem;
            total_so_far /= 5;

            // Break conditions
            if carry != 0 {
                rem += carry;
            } else if rem == 0 && total_so_far == 0 {
                break;
            }

            // Add to result (this is backward)
            let push_char;
            (push_char, carry) = match rem {
                0 => ('0', 0),
                1 => ('1', 0),
                2 => ('2', 0),
                3 => ('=', 1),
                4 => ('-', 1),
                // Might happen because of a carry
                5 => {
                    println!("Got rem = 5");
                    ('0', 1)
                }
                // Shouldn't happen but backup
                _ => {
                    println!("ERROR: Got rem = {}", rem);
                    (['0', '1', '2', '=', '-'][(rem % 5) as usize], (rem - 2) / 5)
                }
            };
            result.push(push_char);
        }

        // Reverse since the result was backward
        result.chars().rev().collect()
    }

    pub fn from_snafu(value: &str) -> Self {
        let mut total = 0i64;
        let mut place = 1;
        for byte in value.bytes().rev() {
            total += place
                * match byte {
                    b'1' => 1,
                    b'2' => 2,
                    b'0' => 0,
                    b'-' => -1,
                    b'=' => -2,
                    _ => panic!("Invalid input"),
                };

            place *= 5;
        }
        Self(total as u64)
    }
}

impl ops::Add for Snafu {
    type Output = Snafu;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl ops::AddAssign for Snafu {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::Snafu;

    #[test]
    fn test_as_decimal() {
        assert_eq!(Snafu::from(1).as_decimal(), 1);
        assert_eq!(Snafu::from(0).as_decimal(), 0);
        assert_eq!(Snafu::from(10).as_decimal(), 10);
        assert_eq!(Snafu::from(234).as_decimal(), 234);
        assert_eq!(Snafu::from(2).as_decimal(), 2);
    }

    #[test]
    fn test_from_snafu() {
        assert_eq!(Snafu::from_snafu("1").as_decimal(), 1);
        assert_eq!(Snafu::from_snafu("2").as_decimal(), 2);
        assert_eq!(Snafu::from_snafu("1=").as_decimal(), 3);
        assert_eq!(Snafu::from_snafu("1-").as_decimal(), 4);
        assert_eq!(Snafu::from_snafu("10").as_decimal(), 5);
        assert_eq!(Snafu::from_snafu("11").as_decimal(), 6);
    }

    #[test]
    fn test_as_snafu() {
        assert_eq!(Snafu::from(1).as_snafu(), "1");
        assert_eq!(Snafu::from(2).as_snafu(), "2");
        assert_eq!(Snafu::from(3).as_snafu(), "1=");
        assert_eq!(Snafu::from(4).as_snafu(), "1-");
        assert_eq!(Snafu::from(5).as_snafu(), "10");
        assert_eq!(Snafu::from(6).as_snafu(), "11");
        assert_eq!(Snafu::from(10).as_snafu(), "20");
        assert_eq!(Snafu::from(20).as_snafu(), "1-0");
        assert_eq!(Snafu::from(12345).as_snafu(), "1-0---0");
        assert_eq!(Snafu::from(314159265).as_snafu(), "1121-1110-1=0");
    }
}
