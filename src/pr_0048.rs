pub fn self_powers(target: usize) -> usize {
    let (mut result, m) = (0, 10_000_000_000);
    for i in 1..=target {
        let mut crr = 1;
        for _ in 0..i {
            crr = crr * i % m;
        }
        result = ((result % m) + (crr % m)) % m;
    }
    result % m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test_self_powers() {
        assert_eq!(self_powers(10), 405_071_317);
    }

    #[test]
    fn test_self_powers() {
        assert_eq!(self_powers(1000), 9_110_846_700);
    }
}
