pub use math;

pub fn sum_square_difference(n: i32) -> i32 {
    math::sum_to(n).pow(2) - math::sum_squares(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_square_difference() {
        assert_eq!(25_164_150, sum_square_difference(100));
    }
}
