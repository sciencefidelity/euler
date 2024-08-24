use traits::Palindrome;

pub struct Euler;

impl Euler {
    pub fn double_base_palindromes(limit: u32) -> u32 {
        let (mut i, mut sum) = (1, 0);
        let mut p = i.into_palindrome(2, true);
        while p < limit {
            if p.is_palindrome(10) {
                sum += p;
            }
            i += 1;
            p = i.into_palindrome(2, true);
        }

        i = 1;
        let mut p = i.into_palindrome(2, false);
        while p < limit {
            if p.is_palindrome(10) {
                sum += p;
            }
            i += 1;
            p = i.into_palindrome(2, false);
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_base_palindromes() {
        assert_eq!(Euler::double_base_palindromes(1_000_000), 872_187);
    }
}
