use seq::prime::prime;

pub fn nth_prime(n: usize) -> usize {
    prime().nth(n).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nth_prime() {
        assert_eq!(104_743, nth_prime(10_001));
    }
}
