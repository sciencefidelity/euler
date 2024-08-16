pub struct Euler;

const AND: usize = 3;
const DIGITS: &[usize] = &[0, 3, 3, 5, 4, 4, 3, 5, 5, 4, 3, 6, 6, 8, 8, 7, 7, 9, 8, 8];
const TENS: &[usize] = &[0, 0, 6, 6, 5, 5, 5, 7, 6, 6];
const MULTIPLIER: &[usize] = &[0, 8, 7, 7];
const HUNDRED: usize = 7;

impl Euler {
    pub const fn number_letter_counts(n: usize) -> usize {
        let mut result = 0;
        let mut i = 1;
        while i <= n {
            result += Self::count_letters_in_number(i);
            i += 1;
        }
        result
    }

    const fn count_letters_in_number(mut num: usize) -> usize {
        let mut output = 0;
        let mut i = 0;
        while i < MULTIPLIER.len() {
            if num % 1000 != 0 {
                output += Self::helper(num % 1000) + MULTIPLIER[i];
            }
            num /= 1000;
            i += 1;
        }
        output
    }

    const fn helper(n: usize) -> usize {
        if n < 20 {
            DIGITS[n]
        } else if n < 100 {
            TENS[n / 10] + Self::helper(n % 10)
        } else {
            let rem = n % 100;
            DIGITS[n / 100] + HUNDRED + if rem == 0 { 0 } else { AND } + Self::helper(rem)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(21_124, Euler::number_letter_counts(1000));
    }

    #[test]
    fn case_2() {
        assert_eq!(23, Euler::count_letters_in_number(342));
    }

    #[test]
    fn case_3() {
        assert_eq!(20, Euler::count_letters_in_number(115));
    }

    #[test]
    fn case_4() {
        assert_eq!(3, Euler::count_letters_in_number(1));
    }

    #[test]
    fn case_5() {
        assert_eq!(11, Euler::count_letters_in_number(400));
    }

    #[test]
    fn case_6() {
        assert_eq!(11, Euler::count_letters_in_number(1000));
    }

    #[test]
    fn case_7() {
        assert_eq!(87, Euler::count_letters_in_number(999_999_999));
    }

    // #[test]
    // fn case_8() {
    //     assert_eq!(50_218_010, Euler::number_letter_counts(1_000_000));
    // }
}
