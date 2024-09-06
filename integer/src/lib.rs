#![allow(clippy::cast_lossless, clippy::cast_precision_loss)]

#[allow(clippy::len_without_is_empty)]
pub trait Integer: Sized {
    /// The maximum pandigital number that an integer type can hold.
    const MAX_PANDIGITAL: Self;

    fn len(&self) -> usize;

    fn is_zero(&self) -> bool;

    fn is_prime(&self) -> bool;

    /// Check if an integer is pandigital.
    ///
    /// An _n_-digit number is pandigital if it makes use of all the digits 1 to _n_ exactly once;
    /// for example, the 5-digit number, 15234, is 1 through 5 pandigital.
    ///
    /// ```rust
    /// # use integer::Integer;
    /// assert!(15_234_i32.is_pandigital());
    /// ```
    fn is_pandigital(&self) -> bool;

    fn is_zero_nine_pandigital(&self) -> bool;

    /// Check if an integer is a triangle number.
    ///
    /// The first 10 triangle numbers are 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
    /// The nth term of the sequence can be expressed as `t = n(n + 1) / 2`.
    /// We can rearrange the equation to use a standard quadratic form `n.pow(2) + n - 2t = 0`
    /// and use the quadratic formula to solve it.
    /// If `(1 + 8t).sqrt()` is a perfect square we get an integer value for `n`.
    ///
    /// ```rust
    /// # use integer::Integer;
    /// assert!(55_i32.is_triangle());
    /// ```
    fn is_triangle(&self) -> bool;

    fn rotate(&self) -> impl Iterator<Item = Self>;
}

macro_rules! int_impl {
    ($t:ty, $max:expr) => {
        impl Integer for $t {
            const MAX_PANDIGITAL: Self = $max;

            fn len(&self) -> usize {
                let n = self.abs();
                if n < 10 {
                    1
                } else if n < 100 {
                    2
                } else if n < 1000 {
                    3
                } else if n < 10_000 {
                    4
                } else if n < 100_000 {
                    5
                } else if n < 1_000_000 {
                    6
                } else if n < 10_000_000 {
                    7
                } else if n < 100_000_000 {
                    8
                } else if n < 1_000_000_000 {
                    9
                } else {
                    10
                }
            }

            fn is_zero(&self) -> bool {
                *self == 0
            }

            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            fn is_prime(&self) -> bool {
                if *self < 4 {
                    *self > 1
                } else if self % 2 == 0 || self % 3 == 0 {
                    false
                } else {
                    let max_p = (*self as f64).sqrt().ceil() as $t;
                    !(5..=max_p)
                        .step_by(6)
                        .any(|p| self % p == 0 || self % (p + 2) == 0)
                }
            }

            fn is_pandigital(&self) -> bool {
                let (mut n, mut has_digits, mut len) = (self.saturating_abs(), [false; 10], 0);
                if n == 0 || n > Self::MAX_PANDIGITAL {
                    return false;
                }
                has_digits[0] = true;
                while n != 0 {
                    let tmp = usize::try_from(n % 10).unwrap();
                    if has_digits[tmp] == true {
                        return false;
                    }
                    has_digits[tmp] = true;
                    n /= 10;
                    len += 1;
                }
                !has_digits[1..=len].contains(&false)
            }

            fn is_zero_nine_pandigital(&self) -> bool {
                let (mut n, mut has_digits) = (*self, [false; 10]);
                while n != 0 {
                    let tmp = usize::try_from(n % 10).unwrap();
                    if has_digits[tmp] == true {
                        return false;
                    }
                    has_digits[tmp] = true;
                    n /= 10;
                }
                !has_digits.contains(&false)
            }

            fn is_triangle(&self) -> bool {
                if *self <= 0 {
                    return false;
                }
                (*self as f64).mul_add(8.0, 1.0).sqrt().fract() == 0.0
            }

            fn rotate(&self) -> impl Iterator<Item = $t> {
                let mut current = *self;
                std::iter::from_fn(move || {
                    let n = u32::try_from(current.len()).unwrap();
                    let multiplier: $t = 10;
                    let rem = current % 10;
                    current /= 10;
                    current += rem * multiplier.pow(n - 1);
                    Some(current)
                })
            }
        }
    };
}

macro_rules! uint_impl {
    ($t:ty, $max:expr) => {
        impl Integer for $t {
            const MAX_PANDIGITAL: Self = $max;

            fn len(&self) -> usize {
                if *self < 10 {
                    1
                } else if *self < 100 {
                    2
                } else if *self < 1000 {
                    3
                } else if *self < 10_000 {
                    4
                } else if *self < 100_000 {
                    5
                } else if *self < 1_000_000 {
                    6
                } else if *self < 10_000_000 {
                    7
                } else if *self < 100_000_000 {
                    8
                } else if *self < 1_000_000_000 {
                    9
                } else {
                    10
                }
            }

            fn is_zero(&self) -> bool {
                *self == 0
            }

            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            fn is_prime(&self) -> bool {
                if *self < 4 {
                    *self > 1
                } else if self % 2 == 0 || self % 3 == 0 {
                    false
                } else {
                    let max_p = (*self as f64).sqrt().ceil() as $t;
                    !(5..=max_p)
                        .step_by(6)
                        .any(|p| self % p == 0 || self % (p + 2) == 0)
                }
            }

            fn is_pandigital(&self) -> bool {
                let (mut n, mut has_digits, mut len) = (*self, [false; 10], 0);
                if n == 0 || n > Self::MAX_PANDIGITAL {
                    return false;
                }
                has_digits[0] = true;
                while n != 0 {
                    let tmp = usize::try_from(n % 10).unwrap();
                    if has_digits[tmp] == true {
                        return false;
                    }
                    has_digits[tmp] = true;
                    n /= 10;
                    len += 1;
                }
                !has_digits[1..=len].contains(&false)
            }

            fn is_zero_nine_pandigital(&self) -> bool {
                let (mut n, mut has_digits) = (*self, [false; 10]);
                while n != 0 {
                    let tmp = usize::try_from(n % 10).unwrap();
                    if has_digits[tmp] == true {
                        return false;
                    }
                    has_digits[tmp] = true;
                    n /= 10;
                }
                !has_digits.contains(&false)
            }

            fn is_triangle(&self) -> bool {
                (*self as f64).mul_add(8.0, 1.0).sqrt().fract() == 0.0
            }

            fn rotate(&self) -> impl Iterator<Item = $t> {
                let mut current = *self;
                std::iter::from_fn(move || {
                    let n = u32::try_from(current.len()).unwrap();
                    let multiplier: $t = 10;
                    let rem = current % 10;
                    current /= 10;
                    current += rem * multiplier.pow(n - 1);
                    Some(current)
                })
            }
        }
    };
}

// int_impl!(i8, 123);
// int_impl!(i16, 32_541);
int_impl!(i32, 987_654_321);
int_impl!(i64, 987_654_321);
int_impl!(i128, 987_654_321);
int_impl!(isize, 987_654_321);
// uint_impl!(u8, 231);
// uint_impl!(u16, 54_321);
uint_impl!(u32, 987_654_321);
uint_impl!(u64, 987_654_321);
uint_impl!(u128, 987_654_321);
uint_impl!(usize, 987_654_321);

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_is_pandigital_i8() {
    //     assert!(1_i8.is_pandigital());
    //     assert!(12_i8.is_pandigital());
    //     assert!(123_i8.is_pandigital());
    //     assert!((-1_i8).is_pandigital());
    //     assert!((-12_i8).is_pandigital());
    //     assert!((-123_i8).is_pandigital());
    //     assert!(!13_i8.is_pandigital());
    //     assert!(!124_i8.is_pandigital());
    //     assert!(!(-11_i8).is_pandigital());
    //     assert!(!(-122_i8).is_pandigital());
    // }

    // #[test]
    // fn test_is_pandigital_i16() {
    //     assert!(32541_i16.is_pandigital());
    //     assert!((-32541_i16).is_pandigital());
    //     assert!(4321_i16.is_pandigital());
    //     assert!((-4321_i16).is_pandigital());
    //     assert!(12_i16.is_pandigital());
    //     assert!((-12_i16).is_pandigital());
    //     assert!(!32561_i16.is_pandigital());
    //     assert!(!13_i16.is_pandigital());
    //     assert!(!987_i16.is_pandigital());
    // }

    #[test]
    fn test_is_pandigital_i32() {
        assert!(987_654_321_i32.is_pandigital());
        assert!((-987_654_321_i32).is_pandigital());
        assert!(4321_i32.is_pandigital());
        assert!((-4321_i32).is_pandigital());
    }

    #[test]
    fn test_is_pandigital_i64() {
        assert!(987_654_321_i64.is_pandigital());
        assert!((-987_654_321_i64).is_pandigital());
        assert!(!i64::MAX.is_pandigital());
        assert!(!i64::MIN.is_pandigital());
    }

    #[test]
    fn test_is_pandigital_i128() {
        assert!(987_654_321_i128.is_pandigital());
        assert!((-987_654_321_i128).is_pandigital());
        assert!(!i128::MAX.is_pandigital());
        assert!(!i128::MIN.is_pandigital());
    }

    #[test]
    fn test_is_pandigital_isize() {
        assert!(987_654_321_isize.is_pandigital());
        assert!((-987_654_321_isize).is_pandigital());
        assert!(!isize::MAX.is_pandigital());
        assert!(!isize::MIN.is_pandigital());
    }

    // #[test]
    // fn test_is_pandigital_u8() {
    //     assert!(1_u8.is_pandigital());
    //     assert!(12_u8.is_pandigital());
    //     assert!(123_u8.is_pandigital());
    //     assert!(!13_u8.is_pandigital());
    //     assert!(!124_u8.is_pandigital());
    //     assert!(!0_u8.is_pandigital());
    // }

    // #[test]
    // fn test_is_pandigital_u16() {
    //     assert!(54321_u16.is_pandigital());
    //     assert!(!32_561_u16.is_pandigital());
    // }

    #[test]
    fn test_is_pandigital_u32() {
        assert!(987_654_321_u32.is_pandigital());
        assert!(4321_u32.is_pandigital());
    }

    #[test]
    fn test_is_pandigital_u64() {
        assert!(987_654_321_u64.is_pandigital());
        assert!(!u64::MAX.is_pandigital());
        assert!(!u64::MIN.is_pandigital());
    }

    #[test]
    fn test_is_pandigital_usize() {
        assert!(987_654_321_usize.is_pandigital());
        assert!(!987_654_320_usize.is_pandigital());
        assert!(!876_543_210_usize.is_pandigital());
        assert!(!usize::MAX.is_pandigital());
        assert!(!usize::MIN.is_pandigital());
    }

    #[test]
    fn test_is_triangle_number() {
        // assert!(1_i8.is_triangle());
        // assert!(3_i16.is_triangle());
        assert!(6_i32.is_triangle());
        assert!(10_i64.is_triangle());
        assert!(15_i128.is_triangle());
        assert!(21_isize.is_triangle());
        // assert!(28_u8.is_triangle());
        // assert!(36_u16.is_triangle());
        assert!(45_u32.is_triangle());
        assert!(55_u64.is_triangle());
        assert!(66_u128.is_triangle());
        assert!(78_usize.is_triangle());
    }

    #[test]
    fn test_is_not_triangle_number() {
        // assert!(!2_i8.is_triangle());
        // assert!(!4_i16.is_triangle());
        assert!(!7_i32.is_triangle());
        assert!(!11_i64.is_triangle());
        assert!(!16_i128.is_triangle());
        assert!(!22_isize.is_triangle());
        // assert!(!29_u8.is_triangle());
        // assert!(!37_u16.is_triangle());
        assert!(!46_u32.is_triangle());
        assert!(!56_u64.is_triangle());
        assert!(!67_u128.is_triangle());
        assert!(!79_usize.is_triangle());
    }

    #[test]
    fn test_is_zero_nine_pandigital() {
        assert!(1_234_567_890_i32.is_zero_nine_pandigital());
        assert!(!123_456_789_i32.is_zero_nine_pandigital());
        assert!(!1_234_567_891_i32.is_zero_nine_pandigital());
    }

    #[test]
    fn test_is_prime() {
        // assert!(127_i8.is_prime());
        // assert!(32_749_i16.is_prime());
        assert!(2_147_483_629_i32.is_prime());
        assert!(99_999_999_977_i64.is_prime());
        assert!(99_999_999_947_i128.is_prime());
        assert!(99_999_999_943_isize.is_prime());
        // assert!(251_u8.is_prime());
        // assert!(65_521_u16.is_prime());
        assert!(4_294_967_291_u32.is_prime());
        assert!(99_999_999_907_u64.is_prime());
        assert!(99_999_999_871_u128.is_prime());
        assert!(99_999_999_851_usize.is_prime());
    }

    #[test]
    fn test_rotate_iter_1_digit() {
        let mut iter = 1_u32.rotate();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(1));
    }

    #[test]
    fn test_rotate_iter_5_digits() {
        let mut iter = 54321_i32.rotate();
        assert_eq!(iter.next(), Some(15432));
        assert_eq!(iter.next(), Some(21543));
        assert_eq!(iter.next(), Some(32154));
        assert_eq!(iter.next(), Some(43215));
        assert_eq!(iter.next(), Some(54321));
    }

    #[test]
    fn test_rotate_iter_5_digits_enumerate() {
        let mut iter = 54321_i32.rotate().enumerate();
        assert_eq!(iter.next(), Some((0, 15432)));
        assert_eq!(iter.next(), Some((1, 21543)));
        assert_eq!(iter.next(), Some((2, 32154)));
        assert_eq!(iter.next(), Some((3, 43215)));
        assert_eq!(iter.next(), Some((4, 54321)));
        assert_eq!(iter.next(), Some((5, 15432)));
        assert_eq!(iter.next(), Some((6, 21543)));
        assert_eq!(iter.next(), Some((7, 32154)));
        assert_eq!(iter.next(), Some((8, 43215)));
        assert_eq!(iter.next(), Some((9, 54321)));
    }
}
