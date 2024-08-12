pub struct Euler;

impl Euler {
    pub fn highly_divisible_triangular_number(p: usize) -> usize {
        let (mut n, mut dn, mut cnt) = (3, 2, 0);
        let prime_array = Self::generate_prime_array(p / 2);
        let mut max_i = 0;
        while cnt <= p {
            n += 1;
            let mut n1 = n;
            if n1 % 2 == 0 {
                n1 /= 2;
            }
            let mut dn1 = 1;
            for i in 0..p {
                max_i = max_i.max(i);
                if prime_array[i].pow(2) > n1 {
                    dn1 *= 2;
                    break;
                }
                let mut exponent = 1;
                while n1 % prime_array[i] == 0 {
                    exponent += 1;
                    n1 /= prime_array[i];
                }
                if exponent > 1 {
                    dn1 *= exponent;
                }
                if n1 == 1 {
                    break;
                }
            }
            cnt = dn * dn1;
            dn = dn1;
        }
        println!("{}", prime_array.len());
        println!("{max_i}");
        n * (n - 1) / 2
    }

    fn generate_prime_array(mut n: usize) -> Vec<usize> {
        let mut primes = Vec::from([2]);
        let mut candidate = 1;
        n -= 1;
        loop {
            candidate += 2;
            if Self::is_prime(i32::try_from(candidate).unwrap()) {
                primes.push(candidate);
            }
            if candidate > n {
                return primes;
            }
        }
    }

    fn is_prime(candidate: i32) -> bool {
        if candidate == 1 {
            return false;
        } else if candidate < 4 {
            return true;
        } else if candidate % 2 == 0 {
            return false;
        } else if candidate < 9 {
            return true;
        } else if candidate % 3 == 0 {
            return false;
        }
        #[allow(clippy::cast_possible_truncation)]
        let r = f64::from(candidate).sqrt().floor() as i32;
        let mut f = 5;
        while f <= r {
            if candidate % f == 0 {
                return false;
            }
            if candidate % (f + 2) == 0 {
                return false;
            }
            f += 6;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(28, Euler::highly_divisible_triangular_number(5));
    }

    #[test]
    fn case_2() {
        assert_eq!(76576500, Euler::highly_divisible_triangular_number(500));
    }

    // #[test]
    // fn case_3() {
    //     assert_eq!(842161320, Euler::highly_divisible_triangular_number(1000));
    // }
    //
    // #[test]
    // fn case_4() {
    //     assert_eq!(842161320, Euler::highly_divisible_triangular_number(10000));
    // }
}
