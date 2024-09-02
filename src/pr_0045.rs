use seq::{hex::hexagonal, pent::pentagonal, tri::triangle};
use std::collections::HashSet;

pub struct Euler;

impl Euler {
    pub fn triangular_pentagonal_and_hexagonal(k: i32) -> i32 {
        let triangles = HashSet::new();
        let pentagonals = HashSet::new();
        for (t, p, h) in triangle().zip(pentagonal()).zip(hexagonal()) {
            println!("{t} {p} {h}");
            if t == 40755 {
                break;
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangular_pentagonal_and_hexagonal_first() {
        assert_eq!(Euler::triangular_pentagonal_and_hexagonal(1), 40755);
    }
}
