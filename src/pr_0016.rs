pub fn power_digit_sum(n: usize) -> usize {
    let mut digits = Vec::with_capacity(n);
    digits.push(1);
    for _ in 0..n {
        let mut carry = 0;
        for dx in &mut digits {
            let d = 2 * *dx + carry;
            (carry, *dx) = (d / 10, d % 10);
        }
        if carry > 0 {
            digits.push(1);
        }
    }
    digits.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_digit_sum() {
        assert_eq!(1366, power_digit_sum(1000));
    }
}
