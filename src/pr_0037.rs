use iter::count_digits_in_number;
use seq::prime::{is_prime, Prime};

pub struct Euler;

impl Euler {
    pub fn truncatable_primes(mut n: i32) -> i32 {
        let p: Prime<i32> = Prime::new();
        let mut sum = 0;
        for prime in p.into_iter().skip(4) {
            let len = count_digits_in_number(prime);
            if Self::truncate_r_to_l(prime / 10)
                && Self::truncate_l_to_r(prime, u32::try_from(len - 1).unwrap())
            {
                sum += prime;
                n -= 1;
                if n <= 0 {
                    break;
                }
            }
        }
        sum
    }

    fn truncate_r_to_l(mut n: i32) -> bool {
        while n > 0 {
            if !is_prime(n) {
                return false;
            }
            n /= 10;
        }
        true
    }

    fn truncate_l_to_r(n: i32, mut i: u32) -> bool {
        while i > 0 {
            if !is_prime(n % 10_i32.pow(i)) {
                return false;
            }
            i -= 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_l_to_r() {
        assert!(Euler::truncate_l_to_r(3797, 4));
    }

    #[test]
    fn test_truncate_r_to_l() {
        assert!(Euler::truncate_r_to_l(3797));
    }

    #[test]
    fn test_truncatable_primes() {
        assert_eq!(Euler::truncatable_primes(11), 748_317);
    }
}
