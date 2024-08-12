pub struct Euler;

impl Euler {
    pub fn highly_divisible_triangular_number(n: usize) -> usize {
        let (mut t, mut a, mut cnt) = (1, 1, 0);
        let prime_array = Self::generate_prime_array(65500);
        while cnt <= n {
            cnt = 1;
            a += 1;
            t += a;
            let mut tt = t;
            for prime in prime_array.iter().take(n) {
                if prime * prime > tt {
                    cnt *= 2;
                    break;
                }
                let mut exponent = 1;
                while tt % prime == 0 {
                    exponent += 1;
                    tt /= prime;
                }
                if exponent > 1 {
                    cnt *= exponent;
                }
                if tt == 1 {
                    break;
                }
            }
        }
        t
    }

    fn generate_prime_array(mut n: usize) -> Vec<usize> {
        let mut primes = Vec::from([2]);
        let mut candidate = 1;
        n -= 1;
        loop {
            candidate += 2;
            if Self::is_prime(i32::try_from(candidate).unwrap()) {
                primes.push(candidate);
            }
            if candidate == n {
                return primes;
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
        assert_eq!(28, Euler::highly_divisible_triangular_number(5));
    }

    #[test]
    fn case_2() {
        assert_eq!(76576500, Euler::highly_divisible_triangular_number(500));
    }
}
