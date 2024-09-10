use integer::Integer;
use seq::prime::prime;
use std::collections::HashSet;

pub struct Euler;

const UNDER_FOUR_DIGITS: usize = 169;
const FOUR_DIGIT_PRIMES: usize = 1061;
const INCREASE: usize = 3330;
const LIMIT: usize = 10_000;

impl Euler {
    pub fn prime_permutations() -> usize {
        let (mut set, mut map) = (HashSet::new(), Vec::new());
        let mut sequences = Vec::new();
        for p in prime().skip(UNDER_FOUR_DIGITS).take(FOUR_DIGIT_PRIMES) {
            set.insert(p);
            map.push(p);
        }
        for p in map {
            let (mut q, mut seq) = (p + INCREASE, Vec::from([p]));
            if q + INCREASE > LIMIT {
                break;
            }
            while q < LIMIT {
                if set.contains(&q) && Self::is_permutation(p, q) {
                    seq.push(q);
                }
                q += INCREASE;
            }
            if seq.len() > 2 {
                sequences.push(seq);
            }
        }
        Self::join(&sequences[1])
    }

    fn is_permutation(a: usize, b: usize) -> bool {
        let (mut a, mut b) = (a.digits(), b.digits());
        a.sort();
        b.sort();
        a == b
    }

    fn join(vec: &[usize]) -> usize {
        let mut result = 0;
        for i in vec.into_iter() {
            result *= 10_000;
            result += usize::try_from(*i).unwrap();
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_permutations() {
        assert_eq!(Euler::prime_permutations(), 296_962_999_629);
    }
}
