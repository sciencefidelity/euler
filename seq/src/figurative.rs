pub fn triangle() -> impl Iterator<Item = i64> {
    let mut state = 1;
    std::iter::from_fn(move || {
        let tri = state * (state + 1) / 2;
        state += 1;
        Some(tri)
    })
}

pub fn pentagonal() -> impl Iterator<Item = i64> {
    let mut state = 1;
    std::iter::from_fn(move || {
        let pent = state * (3 * state - 1) / 2;
        state += 1;
        Some(pent)
    })
}

pub fn hexagonal() -> impl Iterator<Item = i64> {
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
    fn test_triangle() {
        let mut iter = triangle();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(6));
        assert_eq!(iter.next(), Some(10));
        assert_eq!(iter.next(), Some(15));
        assert_eq!(iter.next(), Some(21));
        assert_eq!(iter.next(), Some(28));
        assert_eq!(iter.next(), Some(36));
        assert_eq!(iter.next(), Some(45));
        assert_eq!(iter.next(), Some(55));
    }

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
