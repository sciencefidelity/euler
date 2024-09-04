use seq::prime::prime;

pub struct Euler;

impl Euler {
    pub fn nth_prime(n: usize) -> usize {
        prime().nth(n).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(13, Euler::nth_prime(6));
    }

    #[test]
    fn case_2() {
        assert_eq!(97, Euler::nth_prime(25));
    }

    #[test]
    fn case_3() {
        assert_eq!(104_743, Euler::nth_prime(10_001));
    }
}
