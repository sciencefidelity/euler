use std::collections::HashSet;

pub fn distinct_powers(a: usize, b: i32) -> usize {
    let mut set = HashSet::new();
    for i in 2..=a {
        for j in 2..=b {
            let mut power = vec![0; a * 2];
            power[0] = i;
            let mut carry = 0;
            for _ in 1..j {
                for num in &mut power {
                    let tmp = *num * i + carry;
                    *num = tmp % 10;
                    carry = tmp / 10;
                }
            }
            set.insert(power);
        }
    }
    set.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test_distinct_powers() {
        assert_eq!(distinct_powers(5, 5), 15);
    }

    #[test]
    fn test_distinct_powers() {
        assert_eq!(distinct_powers(100, 100), 9183);
    }
}
