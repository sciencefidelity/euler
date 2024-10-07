use integer::Integer;
use seq::prime::prime;
use std::collections::HashSet;

pub fn distinct_primes_factors(target: usize) -> usize {
    let mut consecutives = Vec::new();
    let mut n = 1;
    while consecutives.len() < target {
        n += 1;
        if n.is_prime() {
            consecutives.clear();
        } else {
            let mut checking = n;
            let mut factors = HashSet::new();
            for p in prime().skip(1) {
                if p > checking {
                    break;
                }
                while checking % p == 0 {
                    factors.insert(p);
                    checking /= p;
                }
            }
            if factors.len() == target {
                consecutives.push(n);
            } else {
                consecutives.clear();
            }
        }
    }
    consecutives[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distinct_prime_factors_two() {
        assert_eq!(distinct_primes_factors(2), 14);
    }

    #[test]
    fn test_distinct_prime_factors_three() {
        assert_eq!(distinct_primes_factors(3), 644);
    }

    #[test]
    fn test_distinct_prime_factors() {
        assert_eq!(distinct_primes_factors(4), 134_043);
    }
}
