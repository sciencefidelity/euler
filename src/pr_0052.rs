use integer::Integer;

const MAX_PANDIGITAL: usize = 9_876_543_210;

pub fn permuted_multiples(target: usize) -> usize {
    'outer: for i in 1..=MAX_PANDIGITAL {
        let mut base_digits = i.digits();
        base_digits.sort_unstable();
        for j in 2..=target {
            let mut comp_digits = (i * j).digits();
            comp_digits.sort_unstable();
            if base_digits != comp_digits {
                continue 'outer;
            }
        }
        return i;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test_permuted_multiples() {
        assert_eq!(permuted_multiples(2), 125_874);
    }

    #[test]
    fn test_permuted_multiples() {
        assert_eq!(permuted_multiples(6), 142_857);
    }
}
