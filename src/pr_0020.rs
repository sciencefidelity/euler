pub struct Euler;

impl Euler {
    pub fn factorial_digit_sum(n: usize) -> usize {
        let mut digits = vec![0; n * 3];
        digits[0] = 1;
        let mut size = 1;
        for i in 2..=n {
            let mut carry = 0;
            for digit in digits.iter_mut().take(size) {
                let prod = *digit * i + carry;
                *digit = prod % 10;
                carry = prod / 10;
            }
            while carry != 0 {
                digits[size] = carry % 10;
                carry /= 10;
                size += 1;
            }
        }
        digits.into_iter().take(size).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(27, Euler::factorial_digit_sum(10));
    }

    #[test]
    fn case_2() {
        assert_eq!(648, Euler::factorial_digit_sum(100));
    }

    #[test]
    fn case_3() {
        assert_eq!(10539, Euler::factorial_digit_sum(1000));
    }
}
