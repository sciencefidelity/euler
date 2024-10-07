pub fn fibonacci() -> impl Iterator<Item = usize> {
    let mut state = (0, 1);
    std::iter::from_fn(move || {
        let current = state.0;
        (state.0, state.1) = (state.1, current + state.1);
        Some(current)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_sequence() {
        let mut fib = fibonacci().skip(2).take(9);
        assert_eq!(fib.next(), Some(1));
        assert_eq!(fib.next(), Some(2));
        assert_eq!(fib.next(), Some(3));
        assert_eq!(fib.next(), Some(5));
        assert_eq!(fib.next(), Some(8));
        assert_eq!(fib.next(), Some(13));
        assert_eq!(fib.next(), Some(21));
        assert_eq!(fib.next(), Some(34));
        assert_eq!(fib.next(), Some(55));
        assert_eq!(fib.next(), None);
    }

    #[test]
    fn test_fibonacci_last() {
        assert_eq!(fibonacci().nth(91), Some(4_660_046_610_375_530_309));
    }
}
