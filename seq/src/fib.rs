use std::ops::Add;

use math::Num;

pub struct Fibonacci<T> {
    curr: T,
    next: T,
}

impl<T> Fibonacci<T>
where
    T: Num,
{
    pub fn new() -> Self {
        Self {
            curr: Num::ZERO,
            next: Num::ONE,
        }
    }
}

impl<T> Iterator for Fibonacci<T>
where
    T: Add<Output = T> + Copy + Num,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;
        (self.curr, self.next) = (self.next, current + self.next);
        Some(current)
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
