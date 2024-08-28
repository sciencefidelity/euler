use num_traits::{one, One, Zero};
use std::ops::{Add, Div, Mul, Rem, Sub};

/// Greatest Common Divisor.
///
/// Finds the greatest common divisor of two numbers.
///
/// ```
/// # use math::gcd;
///
/// assert_eq!(gcd(24, 81), 3)
/// ```
pub fn gcd<T>(mut a: T, mut b: T) -> T
where
    T: Rem<Output = T> + Copy + Zero,
{
    while !b.is_zero() {
        (a, b) = (b, a % b);
    }
    a
}

/// Lowest Common Multiple
pub fn lcm<T>(a: T, b: T) -> T
where
    T: Div<Output = T> + Mul<Output = T> + Rem<Output = T> + Copy + Zero,
{
    a * b / gcd::<T>(a, b)
}

/// Sum Divisible By.
pub fn sum_divisible_by<T>(target: T, divisor: T) -> T
where
    T: Div<Output = T> + Sub<Output = T> + Mul<Output = T> + Add<Output = T> + Copy + One,
{
    let p = (target - one()) / divisor;
    divisor * (p * (p + one())) / (one::<T>() + one())
}

/// Sum squares.
///
/// Sum of the squares of first natural numbers to n.
pub fn sum_squares<T>(target: T) -> T
where
    T: Add<Output = T> + Div<Output = T> + Mul<Output = T> + Copy + One,
{
    let two = one::<T>() + one();
    let six = two + two + two;
    (two * target + one()) * (target + one()) * target / six
}

/// Sum to.
///
/// Sum of first natural numbers to n.
pub fn sum_to<T>(target: T) -> T
where
    T: Add<Output = T> + Div<Output = T> + Mul<Output = T> + Copy + One,
{
    target * (target + one()) / (one::<T>() + one())
}

// TODO: allow for tolerance in tests with floats.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_int() {
        assert_eq!(gcd(24, 81), 3);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn test_gcd_float() {
        assert_eq!(gcd(24.0, 81.0), 3.0);
    }

    #[test]
    fn test_lcm_int() {
        assert_eq!(lcm(24, 81), 648);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn test_lcm_float() {
        assert_eq!(lcm(24.0, 81.0), 648.0);
    }

    #[test]
    fn test_sum_divisible_by_int() {
        assert_eq!(sum_divisible_by(10, 3), 18);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn test_sum_divisible_by_float() {
        assert_eq!(sum_divisible_by(10.0, 3.0), 18.0);
    }

    #[test]
    fn test_sum_squares_int() {
        assert_eq!(sum_squares(10), 385);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn test_squares_float() {
        assert_eq!(sum_squares(10.0), 385.0);
    }

    #[test]
    fn test_sum_to_int() {
        assert_eq!(sum_to(10), 55);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn test_sum_to_float() {
        assert_eq!(sum_to(10.0), 55.0);
    }
}
