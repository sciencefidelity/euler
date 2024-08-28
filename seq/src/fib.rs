use num_traits::{one, zero, One, Zero};
use std::ops::Add;

pub struct Fibonacci<T> {
    curr: T,
    next: T,
}

impl<T> Fibonacci<T>
where
    T: Zero + One,
{
    #[must_use]
    pub fn new() -> Self {
        Self {
            curr: zero(),
            next: one(),
        }
    }
}

impl<T> Iterator for Fibonacci<T>
where
    T: Add<Output = T> + Copy + One + Zero,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;
        (self.curr, self.next) = (self.next, current + self.next);
        Some(current)
    }
}

impl<T> Default for Fibonacci<T>
where
    T: One + Zero,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_sequence() {
        let mut fib = Fibonacci::new().skip(2).take(9);
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
}
