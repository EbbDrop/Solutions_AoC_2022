use std::{
    fmt::{Debug, Display},
    iter::Sum,
    ops::Add,
    str::FromStr,
};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct SNAFU(Vec<i8>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum SNAFUError {
    InvalidDigit(char),
}

impl FromStr for SNAFU {
    type Err = SNAFUError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut snafu = Vec::new();
        for c in s.chars() {
            let d = match c {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => return Err(SNAFUError::InvalidDigit(c)),
            };
            snafu.push(d);
        }
        snafu.reverse();
        Ok(SNAFU(snafu))
    }
}

impl Display for SNAFU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for d in self.0.iter().rev() {
            match d {
                2 => write!(f, "2"),
                1 => write!(f, "1"),
                0 => write!(f, "0"),
                -1 => write!(f, "-"),
                -2 => write!(f, "="),
                _ => unreachable!(),
            }?
        }
        Ok(())
    }
}

impl Debug for SNAFU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl Add<&SNAFU> for &SNAFU {
    type Output = SNAFU;

    fn add(self, rhs: &SNAFU) -> Self::Output {
        let mut result = Vec::new();

        let mut carry = 0;
        for i in 0..(self.0.len() + rhs.0.len()) {
            let a = self.0.get(i);
            let b = rhs.0.get(i);
            if a == None && b == None {
                if carry != 0 {
                    result.push(carry);
                }
                break;
            }

            let a = a.unwrap_or(&0);
            let b = b.unwrap_or(&0);

            let mut r = a + b + carry;
            carry = 0;

            while r < -2 || 2 < r {
                if r > 0 {
                    r -= 5;
                    carry += 1;
                } else {
                    r += 5;
                    carry -= 1;
                }
            }
            result.push(r);
        }

        SNAFU(result)
    }
}

impl Add<SNAFU> for SNAFU {
    type Output = SNAFU;

    fn add(self, rhs: SNAFU) -> Self::Output {
        &self + &rhs
    }
}

impl Add<SNAFU> for &SNAFU {
    type Output = SNAFU;

    fn add(self, rhs: SNAFU) -> Self::Output {
        self + &rhs
    }
}

impl Add<&SNAFU> for SNAFU {
    type Output = SNAFU;

    fn add(self, rhs: &SNAFU) -> Self::Output {
        &self + rhs
    }
}

impl Sum<SNAFU> for SNAFU {
    fn sum<I: Iterator<Item = SNAFU>>(iter: I) -> Self {
        iter.fold("0".parse::<SNAFU>().unwrap(), |acc, x| acc + x)
    }
}

impl<'a> Sum<&'a SNAFU> for SNAFU {
    fn sum<I: Iterator<Item = &'a SNAFU>>(iter: I) -> Self {
        iter.fold("0".parse::<SNAFU>().unwrap(), |acc, x| acc + x)
    }
}

fn main() {
    // let input = include_str!("./example.txt");
    let input = include_str!("./input.txt");

    let nums = input
        .lines()
        .map(|l| l.parse())
        .flatten()
        .collect::<Vec<SNAFU>>();

    dbg!(nums.iter().sum::<SNAFU>());
}
