pub struct Euler;

impl Euler {
    pub fn nth_prime(mut n: usize) -> i32 {
        let mut candidate = 1;
        n -= 1;
        loop {
            candidate += 2;
            if Self::is_prime(candidate) {
                n -= 1;
            }
            if n == 0 {
                return candidate;
            }
        }
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
        assert_eq!(13, Euler::nth_prime(6));
    }

    #[test]
    fn case_2() {
        assert_eq!(97, Euler::nth_prime(25));
    }

    #[test]
    fn case_3() {
        assert_eq!(104743, Euler::nth_prime(10_001));
    }
}
