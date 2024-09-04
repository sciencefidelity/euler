#[allow(
    clippy::module_name_repetitions,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]
#[must_use]
pub fn is_prime(n: usize) -> bool {
    if n < 4 {
        n > 1
    } else if n % 2 == 0 || n % 3 == 0 {
        false
    } else {
        let max_p = (n as f64).sqrt().ceil() as usize;
        !(5..=max_p)
            .step_by(6)
            .any(|p| n % p == 0 || n % (p + 2) == 0)
    }
}

// We set index zero to zero because lists of prime numbers list nth prime 1 as 2, etc.
// This means that many consumers will need to call `skip(1)` to prevent some errors,
// it also means that calling `prime().nth(n)` returns the expected value.
pub fn prime() -> impl Iterator<Item = usize> {
    // key to state initial values = (index, prime 1, prime 2, prime 3, prime 4).
    let mut state = (0, 2, 3, 5, 7);
    std::iter::from_fn(move || {
        let mut prime = 0;
        if state.0 != 0 {
            (prime, state.1) = (state.1, state.2);
            loop {
                (state.2, state.3, state.4) = (state.3, state.4, state.2 + 6);
                if crate::prime::is_prime(state.2) {
                    break;
                }
            }
        }
        state.0 += 1;
        Some(prime)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_iter() {
        let mut primes = prime().take(12);
        assert_eq!(primes.next(), Some(0));
        assert_eq!(primes.next(), Some(2));
        assert_eq!(primes.next(), Some(3));
        assert_eq!(primes.next(), Some(5));
        assert_eq!(primes.next(), Some(7));
        assert_eq!(primes.next(), Some(11));
        assert_eq!(primes.next(), Some(13));
        assert_eq!(primes.next(), Some(17));
        assert_eq!(primes.next(), Some(19));
        assert_eq!(primes.next(), Some(23));
        assert_eq!(primes.next(), Some(29));
        assert_eq!(primes.next(), Some(31));
        assert_eq!(primes.next(), None);
    }

    #[test]
    fn test_is_prime() {
        assert!(is_prime(1_000_000_005_721));
    }
}
