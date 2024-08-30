use iter::count_digits_in_number;
pub struct Euler;

impl Euler {
    pub fn pandigital_multiples(search_space: usize) -> usize {
        let mut max_pandigital = 0;
        for i in 1..=search_space {
            let pandigital = Self::into_pandigital(i);
            max_pandigital = max_pandigital.max(pandigital);
        }
        max_pandigital
    }

    fn is_pandigital(mut n: usize) -> bool {
        if count_digits_in_number(n) != 9 {
            return false;
        }
        let dict = &mut [0; 10];
        dict[0] = 1;
        while n > 0 {
            let digit = n % 10;
            if dict[digit] == 1 {
                return false;
            }
            dict[digit] = 1;
            n /= 10;
        }
        true
    }

    fn into_pandigital(n: usize) -> usize {
        let mut concat = 0;
        for j in 1..=9 {
            let prod = n * j;
            let len = count_digits_in_number(prod);
            if count_digits_in_number(concat) > 9 {
                return 0;
            }
            if j != 1 {
                concat *= 10_usize.pow(u32::try_from(len).unwrap());
            }
            concat += prod;
            if Self::is_pandigital(concat) {
                return concat;
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_pandigital() {
        assert!(Euler::is_pandigital(192_384_576));
        assert!(Euler::is_pandigital(123_456_789));
        assert!(Euler::is_pandigital(987_654_321));
        assert!(Euler::is_pandigital(381_654_729));

        assert!(!Euler::is_pandigital(123));
        assert!(!Euler::is_pandigital(1_234_567_890));
        assert!(!Euler::is_pandigital(380_654_729));
    }

    #[test]
    fn test_into_pandigital() {
        assert_eq!(Euler::into_pandigital(192), 192_384_576);
        assert_eq!(Euler::into_pandigital(9), 918_273_645);
        assert_eq!(Euler::into_pandigital(1), 123_456_789);
    }

    #[test]
    fn test_pandigital_multiples() {
        assert_eq!(Euler::pandigital_multiples(10_000), 932_718_654);
    }
}
