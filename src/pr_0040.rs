pub struct Euler;

const NTH: [usize; 7] = [1, 10, 100, 1000, 10_000, 100_000, 1_000_000];

impl Euler {
    pub fn champernownes_constant(target: usize) -> usize {
        let mut constant = Vec::from([0]);
        for mut n in 0..target {
            let mut digits = Vec::new();
            while n != 0 {
                digits.push(n % 10);
                n /= 10;
            }
            digits.into_iter().rev().for_each(|d| {
                constant.push(d);
            })
        }
        let mut result = constant[NTH[0]];
        for n in NTH.iter().skip(1) {
            result *= constant[*n];
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_champernownes_constant() {
        assert_eq!(Euler::champernownes_constant(186_000), 210);
    }
}
