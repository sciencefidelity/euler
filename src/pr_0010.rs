pub struct Euler;

impl Euler {
    pub fn summation_of_primes(n: i32) -> i64 {
        let mut candidate = 1;
        let mut sum: i64 = 2;
        while candidate < n {
            if Self::is_prime(candidate) {
                sum += i64::from(candidate);
            }
            candidate += 2;
        }
        sum
    }

    fn is_prime(candidate: i32) -> bool {
        if candidate == 1 {
            return false;
        } else if candidate < 4 {
            return true;
        } else if candidate % 2 == 0 {
            return false;
        } else if candidate < 9 {
            return true;
        } else if candidate % 3 == 0 {
            return false;
        }
        #[allow(clippy::cast_possible_truncation)]
        let r = f64::from(candidate).sqrt().floor() as i32;
        let mut f = 5;
        while f <= r {
            if candidate % f == 0 {
                return false;
            }
            if candidate % (f + 2) == 0 {
                return false;
            }
            f += 6;
        }
        true
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
