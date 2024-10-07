use seq::figurative::pentagonal;

pub fn pentagonal_numbers() -> i32 {
    let mut nums = vec![1, 5, 12];
    for pn in pentagonal() {
        let pn = i32::try_from(pn).unwrap();
        nums.push(pn);
        for b in &nums {
            let a = pn - b;
            if is_pentagonal(a) && is_pentagonal(a - b) {
                return a - b;
            }
        }
    }
    0
}

#[allow(clippy::cast_precision_loss, clippy::float_cmp)]
fn is_pentagonal(x: i32) -> bool {
    let r = (f64::from(1 + 24 * x)).sqrt();
    r % 6.0 == 5.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pentagonal_numbers() {
        assert_eq!(pentagonal_numbers(), 5_482_660);
    }

    #[test]
    fn test_is_pentagonal() {
        assert!(is_pentagonal(145));
        assert!(is_pentagonal(117));
        assert!(is_pentagonal(92));
        assert!(is_pentagonal(70));
        assert!(is_pentagonal(51));

        assert!(!is_pentagonal(144));
        assert!(!is_pentagonal(116));
        assert!(!is_pentagonal(91));
        assert!(!is_pentagonal(71));
        assert!(!is_pentagonal(52));
    }
}
