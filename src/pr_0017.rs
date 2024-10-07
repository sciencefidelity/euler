const AND: usize = 3;
const DIGITS: &[usize] = &[0, 3, 3, 5, 4, 4, 3, 5, 5, 4, 3, 6, 6, 8, 8, 7, 7, 9, 8, 8];
const TENS: &[usize] = &[0, 0, 6, 6, 5, 5, 5, 7, 6, 6];
const MULTIPLIER: &[usize] = &[0, 8, 7, 7];
const HUNDRED: usize = 7;

pub const fn number_letter_counts(n: usize) -> usize {
    let mut result = 0;
    let mut i = 1;
    while i <= n {
        result += count_letters_in_number(i);
        i += 1;
    }
    result
}

const fn count_letters_in_number(mut num: usize) -> usize {
    let mut output = 0;
    let mut i = 0;
    while i < MULTIPLIER.len() {
        if num % 1000 != 0 {
            output += helper(num % 1000) + MULTIPLIER[i];
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
        TENS[n / 10] + helper(n % 10)
    } else {
        let rem = n % 100;
        DIGITS[n / 100] + HUNDRED + if rem == 0 { 0 } else { AND } + helper(rem)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_letter_counts() {
        assert_eq!(21_124, number_letter_counts(1000));
    }

    #[test]
    fn test_count_letters_in_number_342() {
        assert_eq!(23, count_letters_in_number(342));
    }

    #[test]
    fn test_count_letters_in_number_115() {
        assert_eq!(20, count_letters_in_number(115));
    }

    #[test]
    fn test_count_letters_in_number_1() {
        assert_eq!(3, count_letters_in_number(1));
    }

    #[test]
    fn test_count_letters_in_number_400() {
        assert_eq!(11, count_letters_in_number(400));
    }

    #[test]
    fn test_count_letters_in_number_1000() {
        assert_eq!(11, count_letters_in_number(1000));
    }

    #[test]
    fn test_count_letters_in_number_999_999_999() {
        assert_eq!(87, count_letters_in_number(999_999_999));
    }
}
