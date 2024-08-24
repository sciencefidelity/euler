use parse::int_to_binary_string;
use traits::Palindrome;

pub struct Euler;

impl Euler {
    pub fn double_base_palindromes(target: i32) -> i32 {
        let mut result = 0;
        for i in 1..target {
            let binary = int_to_binary_string(i);
            if i.is_palindromic() && binary.is_palindromic() {
                result += i;
            }
        }
        result
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
