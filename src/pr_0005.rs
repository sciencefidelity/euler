pub struct Euler;

impl Euler {
    pub fn smallest_multiple(n: u64) -> u64 {
        (1..=n).reduce(Self::lcm).unwrap_or(0)
    }

    /// Lowest Common Multiple
    const fn lcm(a: u64, b: u64) -> u64 {
        a * b / Self::gcd(a, b)
    }

    /// Greatest Common Divisor
    const fn gcd(mut a: u64, mut b: u64) -> u64 {
        while b != 0 {
            (a, b) = (b, a % b);
        }
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(2520, Euler::smallest_multiple(10));
    }

    #[test]
    fn case_2() {
        assert_eq!(232792560, Euler::smallest_multiple(20));
    }
}
