use seq::prime::prime;

pub fn highly_divisible_triangular_number(p: usize) -> usize {
    let (mut n, mut dn, mut cnt) = (3, 2, 0);
    while cnt <= p {
        n += 1;
        let mut n1 = n;
        if n1 % 2 == 0 {
            n1 /= 2;
        }
        let mut dn1 = 1;
        for prime in prime().skip(1).take(p) {
            if prime.pow(2) > n1 {
                dn1 *= 2;
                break;
            }
            let mut exponent = 1;
            while n1 % prime == 0 {
                exponent += 1;
                n1 /= prime;
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
    n * (n - 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highly_divisible_triangular_number() {
        assert_eq!(76_576_500, highly_divisible_triangular_number(500));
    }
}
