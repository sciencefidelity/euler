use integer::Integer;
use seq::prime::is_prime;

pub struct Euler;

const MAX_PANDIGITAL: i32 = 987_654_321;

impl Euler {
    pub fn pandigital_prime() -> i32 {
        for n in (0..=MAX_PANDIGITAL).rev() {
            if n.is_pandigital() && is_prime(n) {
                return n;
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pandigital_prime() {
        assert_eq!(Euler::pandigital_prime(), 7_652_413);
    }
}
