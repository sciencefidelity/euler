pub struct Euler;

impl Euler {
    pub fn multiples_of_3_or_5(n: i32) -> i32 {
        let mut sum = 0;
        for i in 0..n {
            if i % 3 == 0 || i % 5 == 0 {
                sum += i;
            }
        }
        sum
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
        assert_eq!(233168, Euler::multiples_of_3_or_5(1000));
    }
}
