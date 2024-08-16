pub struct Euler;

impl Euler {
    pub fn amicable_numbers(n: usize) -> usize {
        let mut dict = vec![0; n];
        let mut sum = 0;
        for n in 1..n {
            let spd = Self::sum_of_proper_divisors(n);
            dict[n] = spd;
        }
        for (k, v) in dict.iter().enumerate() {
            if *v < n && dict[*v] == k && k != *v {
                sum += k;
            }
        }
        sum
    }

    const fn sum_of_proper_divisors(n: usize) -> usize {
        Self::sum_of_divisors(n) - n
    }

    const fn sum_of_divisors(mut n: usize) -> usize {
        let (mut sum, mut p) = (1, 2);
        while p * p <= n && n > 1 {
            if n % p == 0 {
                let mut j = p.pow(2);
                n = n / p;
                while n % p == 0 {
                    j *= p;
                    n /= p;
                }
                sum *= j - 1;
                sum /= p - 1;
            }
            if p == 2 {
                p = 3;
            } else {
                p += 2;
            }
        }
        if n > 1 {
            sum *= n + 1;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(31626, Euler::amicable_numbers(10_000));
    }

    #[test]
    fn case_2() {
        assert_eq!(504, Euler::amicable_numbers(285));
    }

    #[test]
    fn sum_of_proper_divisors_of_220() {
        assert_eq!(284, Euler::sum_of_proper_divisors(220));
    }
}
