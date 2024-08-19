pub struct Euler;

impl Euler {
    pub const fn digit_fifth_powers(n: u32) -> i32 {
        let (mut result, mut candidate) = (0, 2);
        while candidate < 500_000 {
            let mut temp_candidate = candidate;
            let mut sum_of_pows = 0;
            while temp_candidate > 0 {
                sum_of_pows += (temp_candidate % 10_i32).pow(n);
                temp_candidate /= 10;
            }
            if candidate == sum_of_pows {
                result += candidate;
            }
            candidate += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Euler::digit_fifth_powers(4), 19_316);
    }

    #[test]
    fn case_2() {
        assert_eq!(Euler::digit_fifth_powers(5), 443_839);
    }
}
