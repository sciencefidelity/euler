pub use math;

pub struct Euler;

impl Euler {
    pub fn sum_square_difference(n: i32) -> i32 {
        math::sum_to(n).pow(2) - math::sum_squares(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(2640, Euler::sum_square_difference(10));
    }

    #[test]
    fn case_2() {
        assert_eq!(25_164_150, Euler::sum_square_difference(100));
    }
}
