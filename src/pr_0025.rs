pub fn one_thousand_digit_fibonacci_number<const N: usize>(n: usize) -> i32 {
    let (fib, n1, n2) = (&mut [0; N], &mut [0; N], &mut [0; N]);
    let (mut carry, mut count) = (0, 2);
    (n1[0], n2[0]) = (1, 1);
    while fib[n - 1] == 0 {
        for i in 0..n {
            let temp = n2[i] + n1[i] + carry;
            (fib[i], carry) = (temp % 10, temp / 10);
        }
        n2.copy_from_slice(n1);
        n1.copy_from_slice(fib);
        count += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_thousand_digit_fibonacci_number() {
        const DIGITS: usize = 1000;
        assert_eq!(4782, one_thousand_digit_fibonacci_number::<DIGITS>(DIGITS));
    }
}
