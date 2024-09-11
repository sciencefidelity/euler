use integer::Integer;

pub struct Euler;

const MAX_PANDIGITAL: usize = 9_876_543_210;

impl Euler {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permuted_multiples_2() {
        assert_eq!(Euler::permuted_multiples(2), 125_874);
    }

    #[test]
    fn test_permuted_multiples_6() {
        assert_eq!(Euler::permuted_multiples(6), 142_857);
    }
}
