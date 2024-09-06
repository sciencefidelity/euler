use integer::Integer;

pub fn rotate(num: usize) -> impl Iterator<Item = usize> {
    let mut current = num;
    std::iter::from_fn(move || {
        let n = u32::try_from(current.len()).unwrap();
        let multiplier: usize = 10;
        let rem = current % 10;
        current /= 10;
        current = rem * multiplier.pow(n - 1) + current;
        Some(current)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_iter_1_digit() {
        let mut iter = rotate(1);
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(1));
    }

    #[test]
    fn test_rotate_iter_5_digits() {
        let mut iter = rotate(54321);
        assert_eq!(iter.next(), Some(15432));
        assert_eq!(iter.next(), Some(21543));
        assert_eq!(iter.next(), Some(32154));
        assert_eq!(iter.next(), Some(43215));
        assert_eq!(iter.next(), Some(54321));
    }

    #[test]
    fn test_rotate_iter_5_digits_enumerate() {
        let mut iter = rotate(54321).enumerate();
        assert_eq!(iter.next(), Some((0, 15432)));
        assert_eq!(iter.next(), Some((1, 21543)));
        assert_eq!(iter.next(), Some((2, 32154)));
        assert_eq!(iter.next(), Some((3, 43215)));
        assert_eq!(iter.next(), Some((4, 54321)));
        assert_eq!(iter.next(), Some((5, 15432)));
        assert_eq!(iter.next(), Some((6, 21543)));
        assert_eq!(iter.next(), Some((7, 32154)));
        assert_eq!(iter.next(), Some((8, 43215)));
        assert_eq!(iter.next(), Some((9, 54321)));
    }
}
