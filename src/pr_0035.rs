use iter::Rotate;
use seq::{prime::is_prime, Prime};

pub struct Euler;

impl Euler {
    pub fn circular_primes(target: i32) -> usize {
        let mut result = 0;
        let primes: Prime<i32> = Prime::new();
        for prime in primes {
            if prime >= target {
                break;
            }
            let p = Rotate::new(prime);
            let n = p.len();
            if n == 1 {
                result += 1;
                continue;
            }
            if p.into_iter().take(n - 1).all(|r| is_prime(r)) {
                result += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Euler::circular_primes(100), 13);
    }

    #[test]
    fn case_2() {
        assert_eq!(Euler::circular_primes(1_000_000), 55);
    }
}
