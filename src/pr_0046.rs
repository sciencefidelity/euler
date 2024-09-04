use integer::Integer;
use seq::prime::prime;
use std::cmp::Ordering;

pub struct Euler;

impl Euler {
    pub fn goldbachs_other_conjecture() -> usize {
        let mut n = 35;
        let mut found = false;
        loop {
            if Self::is_odd_composite(n) {
                for p in prime().skip(1) {
                    if p > n + 2 {
                        return n;
                    }
                    for i in 1..=(n - p + 2) {
                        let conj = p + 2 * i.pow(2);
                        match conj.cmp(&n) {
                            Ordering::Less => {}
                            Ordering::Equal => {
                                found = true;
                                break;
                            }
                            Ordering::Greater => break,
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

    fn is_odd_composite(n: usize) -> bool {
        n % 2 != 0 && !n.is_prime()
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
