pub trait Palindrome {
    /// Returns `true` if the number is palindromic.
    ///
    /// # example
    ///
    /// ```rust
    /// # use traits::Palindrome;
    ///
    /// assert!(121.is_palindrome(10));
    /// ```
    fn is_palindrome(&self, base: Self) -> bool;

    /// Creates a palindromic number from `self` based on the `radix`.
    ///
    /// # example
    ///
    /// ```rust
    /// # use traits::Palindrome;
    ///
    /// assert_eq!(123.into_palindrome(2, false), 15_855);
    /// assert_eq!(123.into_palindrome(2, true), 7919);
    /// assert_eq!(123.into_palindrome(10, false), 123_321);
    /// assert_eq!(123.into_palindrome(10, true), 12_321);
    /// ```
    #[must_use]
    fn into_palindrome(self, base: Self, odd_length: bool) -> Self;
}

macro_rules! int_impl {
    ($($t:ty)*) => ($(
        impl Palindrome for $t {
            fn is_palindrome(&self, radix: Self) -> bool {
                debug_assert!(*self > 0, "negative numbers cannot be palindromic");
                debug_assert!(radix == 2 || radix == 10, "radix must be base 2 or base 10");
                let (mut k, mut rev) = (*self, 0);
                while k != 0 {
                    rev = rev * radix + k % radix;
                    k /= radix;
                }
                *self == rev
            }

            #[inline]
            fn into_palindrome(self, radix: Self, odd_length: bool) -> Self {
                debug_assert!(self > 0, "negative numbers cannot be palindromic");
                debug_assert!(radix == 2 || radix == 10, "radix must be base 2 or base 10");
                let (mut n, mut res) = (self, self);
                if odd_length {
                    n /= radix;
                }
                while n > 0 {
                    res = radix * res + n % radix;
                    n /= radix;
                }
                res
            }
        }
    )*)
}

int_impl! { isize i8 i16 i32 i64 i128 usize u8 u16 u32 u64 u128 }

#[cfg(test)]
mod tests {
    use super::Palindrome;

    #[test]
    fn test_is_palindrome() {
        assert!(585.is_palindrome(10));
        assert!(585.is_palindrome(2));
        assert!(!586.is_palindrome(10));
        assert!(!586.is_palindrome(2));
    }

    #[test]
    fn test_into_palindrome() {
        assert_eq!(15855, 123.into_palindrome(2, false));
        assert_eq!(7919, 123.into_palindrome(2, true));
        assert_eq!(123321, 123.into_palindrome(10, false));
        assert_eq!(12321, 123.into_palindrome(10, true));
    }
}
