use std::collections::HashSet;

pub struct Euler;

impl Euler {
    pub fn pandigital_products(k: usize) -> i32 {
        let mut set = HashSet::new();
        for i in 1..50 {
            for j in 1..=2000 {
                if Self::is_pandigital(k, i, j) {
                    set.insert(i * j);
                }
            }
        }
        set.iter().sum()
    }

    fn is_pandigital(k: usize, mut multiplicand: i32, mut multiplier: i32) -> bool {
        let mut count = 0;
        let mut digits = vec![-1; k + 1];
        digits[0] = 0;
        let mut product = multiplicand * multiplier;
        for n in [&mut multiplicand, &mut multiplier, &mut product] {
            while *n != 0 {
                let digit = *n % 10;
                if digit < 1 || digit > i32::try_from(k).unwrap() {
                    return false;
                }
                digits[usize::try_from(digit).unwrap()] = digit;
                count += 1;
                *n /= 10;
            }
        }
        !digits.contains(&-1) && count == k
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Euler::pandigital_products(9), 45228);
    }

    #[test]
    fn test_is_pandigital() {
        assert!(Euler::is_pandigital(9, 39, 186));
    }

    #[test]
    fn test_is_not_pandigital() {
        assert!(!Euler::is_pandigital(9, 39, 176));
    }
}
