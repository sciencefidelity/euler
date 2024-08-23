pub trait Num: Sized + PartialEq {
    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;
    const SIX: Self;
    const TEN: Self;
}

macro_rules! num_impl {
    ($t:ty, $zero: expr, $one:expr, $two:expr, $six:expr, $ten:expr) => {
        impl Num for $t {
            const ZERO: Self = $zero;
            const ONE: Self = $one;
            const TWO: Self = $two;
            const SIX: Self = $six;
            const TEN: Self = $ten;
        }
    };
}

num_impl!(usize, 0, 1, 2, 6, 10);
num_impl!(u8, 0, 1, 2, 6, 10);
num_impl!(u16, 0, 1, 2, 6, 10);
num_impl!(u32, 0, 1, 2, 6, 10);
num_impl!(u64, 0, 1, 2, 6, 10);
num_impl!(u128, 0, 1, 2, 6, 10);

num_impl!(isize, 0, 1, 2, 6, 10);
num_impl!(i8, 0, 1, 2, 6, 10);
num_impl!(i16, 0, 1, 2, 6, 10);
num_impl!(i32, 0, 1, 2, 6, 10);
num_impl!(i64, 0, 1, 2, 6, 10);
num_impl!(i128, 0, 1, 2, 6, 10);

num_impl!(f32, 0.0, 1.0, 2.0, 6.0, 10.0);
num_impl!(f64, 0.0, 1.0, 2.0, 6.0, 10.0);
