use integer::Integer;

const MAX_PANDIGITAL: usize = 987_654_321;

pub fn pandigital_prime() -> usize {
    for n in (0..=MAX_PANDIGITAL).rev() {
        if n.is_pandigital() && n.is_prime() {
            return n;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pandigital_prime() {
        assert_eq!(pandigital_prime(), 7_652_413);
    }
}
