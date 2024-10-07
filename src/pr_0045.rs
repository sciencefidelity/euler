use seq::figurative::{hexagonal, pentagonal, triangle};
use std::collections::HashSet;

pub fn triangular_pentagonal_and_hexagonal(mut k: i32) -> usize {
    let mut pentagonals = HashSet::new();
    let mut hexagonals = HashSet::new();
    for ((t, p), h) in triangle().zip(pentagonal()).zip(hexagonal()).skip(1) {
        pentagonals.insert(p);
        hexagonals.insert(h);
        if pentagonals.contains(&t) && hexagonals.contains(&t) {
            k -= 1;
            if k == 0 {
                return t;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test_triangular_pentagonal_and_hexagonal() {
        assert_eq!(triangular_pentagonal_and_hexagonal(1), 40_755);
    }

    #[test]
    fn test_triangular_pentagonal_and_hexagonal() {
        assert_eq!(triangular_pentagonal_and_hexagonal(2), 1_533_776_805);
    }
}
