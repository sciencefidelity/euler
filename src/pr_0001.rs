use math::sum_divisible_by;

pub fn multiples_of_3_or_5(target: i32) -> i32 {
    sum_divisible_by(target, 3) + sum_divisible_by(target, 5) - sum_divisible_by(target, 15)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiples_of_3_or_5() {
        assert_eq!(233_168, multiples_of_3_or_5(1000));
    }
}
