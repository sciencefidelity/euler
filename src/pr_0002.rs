pub use seq::Fibonacci;

pub struct Euler;

impl Euler {
    pub fn even_fibonacci_numbers(limit: usize) -> usize {
        let fib: Fibonacci<usize> = Fibonacci::new();
        let mut sum = 0;
        for num in fib {
            if num > limit {
                break;
            }
            if num % 2 == 0 {
                sum += num;
            }
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let sum: usize = [2, 8, 34].iter().sum();
        assert_eq!(sum, Euler::even_fibonacci_numbers(90));
    }

    #[test]
    fn case_2() {
        let sum: usize = 4_613_732;
        assert_eq!(sum, Euler::even_fibonacci_numbers(4_000_000));
    }
}
