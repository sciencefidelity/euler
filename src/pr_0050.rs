use integer::Integer;
use seq::prime::prime;
pub struct Euler;

impl Euler {
    pub fn consecutive_prime_sum(target: usize) -> usize {
        let (mut max_count, mut max_prime) = (0, 0);
        for i in 1..=4 {
            let (mut count, mut sum) = (0, 0);
            for p in prime().skip(i) {
                sum += p;
                if sum > target {
                    break;
                }
                if count > 1 && sum.is_prime() && count > max_count {
                    (max_count, max_prime) = (count, sum);
                }
                count += 1;
            }
        }
        max_prime
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consecutive_prime_sum_100() {
        assert_eq!(Euler::consecutive_prime_sum(100), 41);
    }

    #[test]
    fn test_consecutive_prime_sum_1000() {
        assert_eq!(Euler::consecutive_prime_sum(1000), 953);
    }

    #[test]
    fn test_consecutive_prime_sum_million() {
        assert_eq!(Euler::consecutive_prime_sum(1_000_000), 997_651);
    }
}
