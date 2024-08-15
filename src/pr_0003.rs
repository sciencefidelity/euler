pub struct Euler;

impl Euler {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(29, Euler::largest_prime_factor(13_195));
    }

    #[test]
    fn case_2() {
        assert_eq!(6857, Euler::largest_prime_factor(600_851_475_143));
    }
}
