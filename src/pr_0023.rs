pub fn non_abundant_sums(n: usize) -> usize {
    let mut abundant = Vec::new();
    for i in 1..=n {
        if sum_of_proper_divisors(i) > i {
            abundant.push(i);
        }
    }
    let m = abundant.len();
    let mut sum_of_sum_abundant = 0;
    let mut seen = vec![false; n + 1];
    for i in 0..m {
        for j in i..m {
            let abundant_sum = abundant[i] + abundant[j];
            if abundant_sum <= n && !seen[abundant_sum] {
                sum_of_sum_abundant += abundant_sum;
                seen[abundant_sum] = true;
            }
        }
    }
    let sum_of_all_nums = (1 + n) * n / 2;
    sum_of_all_nums - sum_of_sum_abundant
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
    fn test_non_abundant_sums() {
        assert_eq!(4_179_871, non_abundant_sums(28_123));
    }
}
