use math::gcd;

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
    dp / gcd(np, dp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit_cancelling_fractions() {
        assert_eq!(digit_cancelling_fractions(), 100);
    }
}
