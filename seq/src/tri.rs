pub fn triangle() -> impl Iterator<Item = i32> {
    let mut state = 1;
    std::iter::from_fn(move || {
        let tri = state * (state + 1) / 2;
        state += 1;
        Some(tri)
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
}
