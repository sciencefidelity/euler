use integer::Integer;
use seq::prime::prime;

pub struct Euler;

impl Euler {
    pub fn truncatable_primes(mut n: usize) -> usize {
        let mut sum = 0;
        for prime in prime().skip(5) {
            let len = prime.len();
            if Self::truncate_r_to_l(prime / 10)
                && Self::truncate_l_to_r(prime, u32::try_from(len - 1).unwrap())
            {
                sum += prime;
                n -= 1;
                if n == 0 {
                    break;
                }
            }
        }
        sum
    }

    fn truncate_r_to_l(mut n: usize) -> bool {
        while n > 0 {
            if !n.is_prime() {
                return false;
            }
            n /= 10;
        }
        true
    }

    fn truncate_l_to_r(n: usize, mut i: u32) -> bool {
        while i > 0 {
            if !(n % 10_usize.pow(i)).is_prime() {
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
