use math::sum_divisible_by;

pub struct Euler;

impl Euler {
    pub fn multiples_of_3_or_5(target: i32) -> i32 {
        sum_divisible_by(target, 3) + sum_divisible_by(target, 5) - sum_divisible_by(target, 15)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(23, Euler::multiples_of_3_or_5(10));
    }

    #[test]
    fn case_2() {
        assert_eq!(2318, Euler::multiples_of_3_or_5(100));
    }

    #[test]
    fn case_3() {
        assert_eq!(233_168, Euler::multiples_of_3_or_5(1000));
    }

    #[test]
    fn case_4() {
        assert_eq!(1_493_266_668, Euler::multiples_of_3_or_5(80_000));
    }
}
