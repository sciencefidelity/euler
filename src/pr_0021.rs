pub const fn amicable_numbers<const N: usize>(n: usize) -> usize {
    let (mut dict, mut sum, mut i) = ([0; N], 0, 1);
    while i < n {
        let spd = sum_of_proper_divisors(i);
        dict[i] = spd;
        i += 1;
    }
    i = 0;
    while i < n {
        if dict[i] < n && dict[dict[i]] == i && i != dict[i] {
            sum += i;
        }
        i += 1;
    }
    sum
}

const fn sum_of_proper_divisors(n: usize) -> usize {
    sum_of_divisors(n) - n
}

const fn sum_of_divisors(mut n: usize) -> usize {
    let (mut sum, mut p) = (1, 2);
    while p * p <= n && n > 1 {
        if n % p == 0 {
            let mut j = p.pow(2);
            n /= p;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amicable_numbers() {
        const TARGET: usize = 10_000;
        assert_eq!(31626, amicable_numbers::<TARGET>(TARGET));
    }

    #[test]
    fn sum_of_proper_divisors_of_220() {
        assert_eq!(284, sum_of_proper_divisors(220));
    }
}
