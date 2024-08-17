pub struct Euler;

impl Euler {
    pub fn lexicographic_permutations(n: usize, k: u8) -> usize {
        let mut list = Vec::new();
        Self::backtrack(&mut list, &mut Vec::with_capacity(usize::from(k + 1)), k);
        list[n - 1]
    }

    fn backtrack(list: &mut Vec<usize>, comb: &mut Vec<u8>, k: u8) {
        if comb.len() == usize::from(k + 1) {
            list.push(Self::vec_to_digit(comb));
            return;
        }
        for i in 0..=k {
            if !comb.contains(&i) {
                comb.push(i);
                Self::backtrack(list, comb, k);
                comb.pop();
            }
        }
    }

    fn vec_to_digit(vec: &Vec<u8>) -> usize {
        let mut result = 0;
        for i in vec {
            result *= 10;
            result += usize::from(*i);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(201, Euler::lexicographic_permutations(5, 2));
    }

    #[test]
    fn case_2() {
        assert_eq!(
            2_783_915_460,
            Euler::lexicographic_permutations(1_000_000, 9)
        );
    }
}
