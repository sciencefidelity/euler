use seq::figurative::pentagonal;

pub struct Euler;

impl Euler {
    pub fn pentagonal_numbers() -> i64 {
        let mut nums = vec![1, 5, 12];
        for pn in pentagonal() {
            nums.push(pn);
            for b in &nums {
                let a = pn - b;
                if Self::is_pentagonal(a) && Self::is_pentagonal((a - b).abs()) {
                    return a - b;
                }
            }
        }
        0
    }

    #[allow(clippy::cast_precision_loss, clippy::float_cmp)]
    fn is_pentagonal(x: i64) -> bool {
        let r = ((1 + 24 * x) as f64).sqrt();
        r % 6.0 == 5.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pentagonal_numbers() {
        assert_eq!(Euler::pentagonal_numbers(), 5_482_660);
    }

    #[test]
    fn test_is_pentagonal() {
        assert!(Euler::is_pentagonal(145));
        assert!(Euler::is_pentagonal(117));
        assert!(Euler::is_pentagonal(92));
        assert!(Euler::is_pentagonal(70));
        assert!(Euler::is_pentagonal(51));

        assert!(!Euler::is_pentagonal(144));
        assert!(!Euler::is_pentagonal(116));
        assert!(!Euler::is_pentagonal(91));
        assert!(!Euler::is_pentagonal(71));
        assert!(!Euler::is_pentagonal(52));
    }
}
