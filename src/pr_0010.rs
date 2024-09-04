use integer::Integer;

pub struct Euler;

impl Euler {
    pub fn summation_of_primes(n: i32) -> i64 {
        let mut candidate = 1;
        let mut sum: i64 = 2;
        while candidate < n {
            if candidate.is_prime() {
                sum += i64::from(candidate);
            }
            candidate += 2;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(17, Euler::summation_of_primes(10));
    }

    #[test]
    fn case_2() {
        assert_eq!(1060, Euler::summation_of_primes(100));
    }

    #[test]
    fn case_3() {
        assert_eq!(142_913_828_922, Euler::summation_of_primes(2_000_000));
    }
}
