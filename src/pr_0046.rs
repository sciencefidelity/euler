use seq::prime::{is_prime, Prime};
pub struct Euler;

impl Euler {
    pub fn goldbachs_other_conjecture() -> i32 {
        let mut n = 35;
        let mut found = false;
        loop {
            if Self::is_odd_composite(n) {
                let primes: Prime<i32> = Prime::new();
                for p in primes {
                    if p > n + 2 {
                        return n;
                    }
                    for i in 1..=(n - p + 2) {
                        let conj = p + 2 * i.pow(2);
                        if conj == n {
                            found = true;
                            break;
                        } else if conj > n {
                            break;
                        }
                    }
                    if found {
                        break;
                    }
                }
            }
            found = false;
            n += 1;
        }
    }

    fn is_odd_composite(n: i32) -> bool {
        if n % 2 != 0 && !is_prime(n) {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_goldbachs_other_conjecture() {
        assert_eq!(Euler::goldbachs_other_conjecture(), 5777);
    }
}
