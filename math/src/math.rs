use std::ops::{Add, Div, Mul, Sub};

pub trait Num {
    const ONE: Self;
    const TWO: Self;
}

macro_rules! num_impl {
    ($t:ty, $v:expr, $w:expr) => {
        impl Num for $t {
            const ONE: Self = $v;
            const TWO: Self = $w;
        }
    };
}

num_impl!(usize, 1, 2);
num_impl!(u8, 1, 2);
num_impl!(u16, 1, 2);
num_impl!(u32, 1, 2);
num_impl!(u64, 1, 2);
num_impl!(u128, 1, 2);
num_impl!(isize, 1, 2);
num_impl!(i8, 1, 2);
num_impl!(i16, 1, 2);
num_impl!(i32, 1, 2);
num_impl!(i64, 1, 2);
num_impl!(i128, 1, 2);
num_impl!(f32, 1.0, 2.0);
num_impl!(f64, 1.0, 2.0);

pub fn sum_divisible_by<T>(target: T, n: T) -> T
where
    T: Div<Output = T> + Sub<Output = T> + Mul<Output = T> + Add<Output = T> + Copy + Num,
{
    let p = (target - Num::ONE) / n;
    n * (p * (p + Num::ONE)) / Num::TWO
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_divisible_by_int() {
        assert_eq!(sum_divisible_by(10, 3), 18);
    }

    #[test]
    fn test_sum_divisible_by_float() {
        assert_eq!(sum_divisible_by(10.0, 3.0), 18.0);
    }
}
