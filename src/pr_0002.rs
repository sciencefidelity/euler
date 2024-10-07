pub use seq::fibonacci::fibonacci;

pub fn even_fibonacci_numbers(limit: usize) -> usize {
    let mut sum = 0;
    for num in fibonacci() {
        if num > limit {
            break;
        }
        if num % 2 == 0 {
            sum += num;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_even_fibonacci_numbers() {
        let sum: usize = 4_613_732;
        assert_eq!(sum, even_fibonacci_numbers(4_000_000));
    }
}
