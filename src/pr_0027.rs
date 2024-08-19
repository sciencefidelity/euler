pub struct Euler;

impl Euler {
    pub fn quadratic_primes(target: i32) -> i32 {
        let (mut max_primes, mut a_best, mut b_best) = (0, 0, 0);
        for a in -target..target {
            for b in -target..=target {
                let (mut n, mut primes) = (0_i32, 0);
                loop {
                    let candidate = (n.pow(2) + (a * n) + b).abs();
                    if Self::is_prime(candidate) {
                        primes += 1;
                    } else {
                        if primes > max_primes {
                            (max_primes, a_best, b_best) = (primes, a, b);
                        }
                        break;
                    }
                    n += 1;
                }
            }
        }
        println!("{max_primes}");
        a_best * b_best
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
        assert_eq!(Euler::quadratic_primes(1000), -59231);
    }
}
