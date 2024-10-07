use integer::Integer;

pub fn quadratic_primes(target: i32) -> i32 {
    let (mut max_primes, mut a_best, mut b_best) = (0, 0, 0);
    for a in -target..target {
        for b in -target..=target {
            let (mut n, mut primes) = (0_i32, 0);
            loop {
                let candidate = (n.pow(2) + (a * n) + b).abs();
                if candidate.is_prime() {
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
    a_best * b_best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quadratic_primes() {
        assert_eq!(quadratic_primes(1000), -59231);
    }
}
