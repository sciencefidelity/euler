use num_traits::{one, zero, One, Pow, PrimInt, Zero};
use std::ops::{Add, DivAssign, Mul, Rem};

pub fn count_digits_in_number<T>(n: T) -> usize
where
    T: DivAssign + Zero + One,
{
    let mut n = n;
    let mut result = zero();
    while !n.is_zero() {
        n /= one::<T>() + one() + one() + one() + one() + one() + one() + one() + one() + one();
        result += 1;
    }
    result
}

pub struct Rotate<T> {
    original: T,
    current: T,
    length: usize,
    index: usize,
}

impl<T> Rotate<T>
where
    T: Add<Output = T>
        + Copy
        + DivAssign
        + Mul<Output = T>
        + One
        + Pow<u32, Output = T>
        + Rem<Output = T>
        + Zero,
{
    pub fn new(num: T) -> Self {
        let length = count_digits_in_number(num);
        Self {
            original: num,
            current: num,
            length,
            index: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.original.is_zero()
    }

    pub fn rotate(&mut self) {
        let n = u32::try_from(self.len()).unwrap();
        let multiplier: T = Self::ten();
        let rem = self.current % Self::ten();
        self.current /= Self::ten();
        self.current = rem * multiplier.pow(n - 1) + self.current;
    }

    pub fn ten() -> T {
        one::<T>() + one() + one() + one() + one() + one() + one() + one() + one() + one()
    }
}

impl<T> Iterator for Rotate<T>
where
    T: Add<Output = T>
        + Copy
        + DivAssign
        + Mul<Output = T>
        + One
        + Pow<u32, Output = T>
        + PrimInt
        + Rem<Output = T>
        + Zero,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.len() - 1 {
            self.index = 0;
        }
        self.rotate();
        self.index += 1;
        Some(self.current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_iter_1_digit() {
        let r = Rotate::new(1);
        let mut iter = r.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(1));
    }

    #[test]
    fn test_rotate_iter_5_digits() {
        let r = Rotate::new(54321);
        let mut iter = r.into_iter();
        assert_eq!(iter.next(), Some(15432));
        assert_eq!(iter.next(), Some(21543));
        assert_eq!(iter.next(), Some(32154));
        assert_eq!(iter.next(), Some(43215));
        assert_eq!(iter.next(), Some(54321));
    }

    #[test]
    fn test_rotate_len() {
        let r = Rotate::new(123456789);
        assert_eq!(r.len(), 9);
    }

    #[test]
    fn test_rotate_1_digit() {
        let mut r = Rotate::new(7);
        r.rotate();
        assert_eq!(r.current, 7);
    }

    #[test]
    fn test_rotate_2_digit() {
        let mut r = Rotate::new(97);
        r.rotate();
        assert_eq!(r.current, 79);
    }

    #[test]
    fn test_rotate_3_digit() {
        let mut r = Rotate::new(197);
        r.rotate();
        assert_eq!(r.current, 719);
    }

    #[test]
    fn test_rotate_4_digit() {
        let mut r = Rotate::new(1234);
        r.rotate();
        assert_eq!(r.current, 4123);
    }

    #[test]
    fn test_rotate_5_digit() {
        let mut r = Rotate::new(56789);
        r.rotate();
        assert_eq!(r.current, 95678);
    }
}
