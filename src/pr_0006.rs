pub struct Euler;

impl Euler {
    pub const fn sum_square_difference(n: i32) -> i32 {
        Self::sum(n).pow(2) - Self::sum_of_squares(n)
    }

    const fn sum_of_squares(n: i32) -> i32 {
        (2 * n + 1) * (n + 1) * n / 6
    }

    const fn sum(n: i32) -> i32 {
        n * (n + 1) / 2
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
        assert_eq!(25164150, Euler::sum_square_difference(100));
    }
}
