pub fn pentagonal() -> impl Iterator<Item = i32> {
    let mut state = 1;
    std::iter::from_fn(move || {
        let pent = state * (3 * state - 1) / 2;
        state += 1;
        Some(pent)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pantagonal() {
        let mut iter = pentagonal();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), Some(12));
        assert_eq!(iter.next(), Some(22));
        assert_eq!(iter.next(), Some(35));
        assert_eq!(iter.next(), Some(51));
        assert_eq!(iter.next(), Some(70));
        assert_eq!(iter.next(), Some(92));
        assert_eq!(iter.next(), Some(117));
        assert_eq!(iter.next(), Some(145));
    }
}
