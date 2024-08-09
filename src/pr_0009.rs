pub struct Euler;

impl Euler {
    pub fn special_pythagorian_triplet(s: i32) -> i32 {
        for a in 3..(s - 3) / 3 {
            for b in (a + 1)..(s - 1 - a) / 2 {
                let c = s - a - b;
                if c.pow(2) == a.pow(2) + b.pow(2) {
                    return a * b * c;
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(31875000, Euler::special_pythagorian_triplet(1000));
    }
}
