use integer::Integer;
use seq::prime::{is_prime, prime};

pub fn circular_primes(target: usize) -> usize {
    let mut result = 0;
    for prime in prime().skip(1) {
        if prime >= target {
            break;
        }
        let n = prime.len();
        if n == 1 {
            result += 1;
            continue;
        }
        if prime.rotate().take(n - 1).all(is_prime) {
            result += 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test_circular_primes() {
        assert_eq!(circular_primes(100), 13);
    }

    #[test]
    fn test_circular_primes() {
        assert_eq!(circular_primes(1_000_000), 55);
    }
}
