pub trait Palindrome {
    type Item;

    fn is_palindromic(&self) -> bool;
}

impl Palindrome for &str {
    type Item = Self;

    fn is_palindromic(&self) -> bool {
        let mut chars = self.chars().filter(|c| c.is_alphanumeric());
        while let (Some(c1), Some(c2)) = (chars.next(), chars.next_back()) {
            if !c1.eq_ignore_ascii_case(&c2) {
                return false;
            }
        }
        true
    }
}

impl Palindrome for String {
    type Item = Self;

    fn is_palindromic(&self) -> bool {
        let mut chars = self.chars().filter(|c| c.is_alphanumeric());
        while let (Some(c1), Some(c2)) = (chars.next(), chars.next_back()) {
            if !c1.eq_ignore_ascii_case(&c2) {
                return false;
            }
        }
        true
    }
}

macro_rules! signed_impl {
    ($t:ty) => {
        impl Palindrome for $t {
            type Item = Self;

            fn is_palindromic(&self) -> bool {
                if *self < 0 {
                    return false;
                }
                let (mut tmp, mut rev) = (*self, 0);
                while tmp != 0 {
                    rev = rev * 10 + tmp % 10;
                    tmp /= 10;
                }
                *self == rev
            }
        }
    };
}

macro_rules! unsigned_impl {
    ($t:ty) => {
        impl Palindrome for $t {
            type Item = Self;

            fn is_palindromic(&self) -> bool {
                let (mut tmp, mut rev) = (*self, 0);
                while tmp != 0 {
                    rev = rev * 10 + tmp % 10;
                    tmp /= 10;
                }
                *self == rev
            }
        }
    };
}

signed_impl!(isize);
signed_impl!(i8);
signed_impl!(i16);
signed_impl!(i32);
signed_impl!(i64);
signed_impl!(i128);

unsigned_impl!(usize);
unsigned_impl!(u8);
unsigned_impl!(u16);
unsigned_impl!(u32);
unsigned_impl!(u64);
unsigned_impl!(u128);

#[cfg(test)]
mod tests {
    use super::Palindrome;

    #[test]
    fn test_is_palindrome_string_slice() {
        assert!("1001001001".is_palindromic());
    }

    #[test]
    fn test_is_not_palindrome_string_slice() {
        assert!(!"1001101001".is_palindromic());
    }

    #[test]
    fn test_is_palindrome_string() {
        assert!("1001001001".to_owned().is_palindromic());
    }

    #[test]
    fn test_is_not_palindrome_string() {
        assert!(!"1001101001".to_owned().is_palindromic());
    }

    #[test]
    fn test_is_palindrome_i32() {
        assert!(585_i32.is_palindromic());
    }

    #[test]
    fn test_is_palindrome_u32() {
        assert!(585_u32.is_palindromic());
    }

    #[test]
    fn test_is_not_palindrome_i32() {
        assert!(!586_i32.is_palindromic());
    }

    #[test]
    fn test_is_not_palindrome_u32() {
        assert!(!586_u32.is_palindromic());
    }
}
