pub const fn largest_prime_factor(mut n: i64) -> i64 {
    let mut i = 2;
    while i * i < n {
        while n % i == 0 {
            n /= i;
        }
        i += 1;
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_prime_factor() {
        assert_eq!(6857, largest_prime_factor(600_851_475_143));
    }
}
