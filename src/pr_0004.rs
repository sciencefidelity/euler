pub struct Euler;

impl Euler {
    pub const fn largest_palindrome_product(n: i32) -> i32 {
        let (high, low) = Self::get_largest_number_with_n_digits(n);
        let (mut i, mut max) = (high, -1);
        while i > low {
            if max >= i * high {
                break;
            }
            let mut j = i;
            while j > low {
                let p = i * j;
                if max < p && Self::is_palindromic(p) {
                    max = p;
                }
                j -= 1;
            }
            i -= 1;
        }
        max
    }

    const fn is_palindromic(n: i32) -> bool {
        let mut x = n;
        let mut rev = 0;
        while x > 0 {
            rev = rev * 10 + x % 10;
            x /= 10;
        }
        rev == n
    }

    const fn get_largest_number_with_n_digits(mut n: i32) -> (i32, i32) {
        let mut multiplier = 1;
        let mut largest_number = 0;
        while n > 0 {
            largest_number += 9 * multiplier;
            multiplier *= 10;
            n -= 1;
        }
        (largest_number, multiplier / 10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(9009, Euler::largest_palindrome_product(2));
    }

    #[test]
    fn case_2() {
        assert_eq!(906_609, Euler::largest_palindrome_product(3));
    }

    #[test]
    fn case_3() {
        assert_eq!(99_000_099, Euler::largest_palindrome_product(4));
    }

    #[test]
    fn case_4() {
        assert_eq!(9, Euler::largest_palindrome_product(1));
    }
}
