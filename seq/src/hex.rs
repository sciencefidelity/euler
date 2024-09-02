pub fn hexagonal() -> impl Iterator<Item = i32> {
    let mut state = 1;
    std::iter::from_fn(move || {
        let hex = state * (2 * state - 1);
        state += 1;
        Some(hex)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hexagonal() {
        let mut iter = hexagonal();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(6));
        assert_eq!(iter.next(), Some(15));
        assert_eq!(iter.next(), Some(28));
        assert_eq!(iter.next(), Some(45));
        assert_eq!(iter.next(), Some(66));
        assert_eq!(iter.next(), Some(91));
        assert_eq!(iter.next(), Some(120));
        assert_eq!(iter.next(), Some(153));
        assert_eq!(iter.next(), Some(190));
    }
}
