pub struct Euler;

impl Euler {
    pub fn digit_cancelling_fractions() -> i32 {
        let mut dp = 1;
        let mut np = 1;
        for c in 1..=9 {
            for d in 1..c {
                for n in 1..d {
                    if (n * 10 + c) * d == (c * 10 + d) * n {
                        np *= n;
                        dp *= d;
                    }
                }
            }
        }
        dp / Self::gcd(np, dp)
    }

    const fn gcd(mut a: i32, mut b: i32) -> i32 {
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
        assert_eq!(Euler::digit_cancelling_fractions(), 100);
    }
}
