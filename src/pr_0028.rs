pub const fn number_spiral_diagonals(n: usize) -> usize {
    assert!(n % 2 != 0);
    let (mut result, mut i, mut m, mut round) = (1, 2, 2, 1);
    while i <= n.pow(2) {
        if (i - 1) % m == 0 {
            result += i;
            round += 1;
        }
        if round == 5 {
            m += 2;
            round = 1;
        }
        i += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test_number_spiral_diagonals() {
        assert_eq!(number_spiral_diagonals(5), 101);
    }

    #[test]
    fn test_number_spiral_diagonals() {
        assert_eq!(number_spiral_diagonals(1001), 669_171_001);
    }
}
