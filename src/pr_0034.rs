pub struct Euler;

const FACTORIALS: [usize; 10] = [1, 1, 2, 6, 24, 120, 720, 5_040, 40_320, 362_880];

impl Euler {
    pub fn digit_factorials() -> usize {
        let mut result = 0;
        for i in 10..50_000 {
            let digits = Self::split_digits(i);
            if i == Self::sum_factorial_of_digits(&digits) {
                result += i;
            }
        }
        result
    }

    fn split_digits(n: usize) -> Vec<usize> {
        let mut n = n;
        let mut digits = Vec::new();
        while n > 0 {
            digits.push(n % 10);
            n /= 10;
        }
        digits
    }

    fn sum_factorial_of_digits(v: &[usize]) -> usize {
        v.iter().map(|d| FACTORIALS[*d]).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_digits() {
        assert_eq!(Euler::split_digits(145), vec![5, 4, 1]);
        assert_eq!(Euler::split_digits(40_585), vec![5, 8, 5, 0, 4]);
    }

    #[test]
    fn test_sum_factorials_of_digits() {
        let digits = Euler::split_digits(145);
        assert_eq!(Euler::sum_factorial_of_digits(&digits), 145);

        let digits = Euler::split_digits(40_585);
        assert_eq!(Euler::sum_factorial_of_digits(&digits), 40_585);
    }

    #[test]
    fn test_digit_factorials() {
        assert_eq!(Euler::digit_factorials(), 40_730);
    }
}
