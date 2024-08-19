pub struct Euler;

impl Euler {
    pub fn reciprocal_cycles(n: usize) -> usize {
        let (mut max_cycle_len, mut result) = (0, 0);
        for i in 2..=n {
            let cycle_len = Self::longdiv(1, i);
            if cycle_len > max_cycle_len {
                max_cycle_len = cycle_len;
                result = i;
            }
        }
        result
    }

    pub fn longdiv(numerator: usize, denominator: usize) -> usize {
        let mut digits = vec![];
        let mut remainders = vec![];
        let mut n = numerator;

        while !remainders.contains(&n) {
            remainders.push(n);
            digits.push(n / denominator);
            n = n % denominator * 10;
        }
        if n != 0 {
            let recurring = remainders.iter().position(|&x| x == n).unwrap();
            return digits.len() - recurring;
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(7, Euler::reciprocal_cycles(10));
    }

    #[test]
    fn case_2() {
        assert_eq!(983, Euler::reciprocal_cycles(1000));
    }

    // #[test]
    // fn case_3() {
    //     assert_eq!(983, Euler::reciprocal_cycles(10_000));
    // }
}
