use integer::Integer;

pub fn summation_of_primes(n: i32) -> i64 {
    let mut candidate = 1;
    let mut sum: i64 = 2;
    while candidate < n {
        if candidate.is_prime() {
            sum += i64::from(candidate);
        }
        candidate += 2;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_summation_of_primes() {
        assert_eq!(142_913_828_922, summation_of_primes(2_000_000));
    }
}
