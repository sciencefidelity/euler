pub struct Euler;

impl Euler {
    pub fn longest_collatz_sequence(n: usize) -> i32 {
        let (mut output, mut longest_chain, mut i) = (1, 0, n / 2);
        let mut values = vec![0; 28_495_741_760 + 1];
        while i < n {
            let len = Self::count_chain(i, &mut values);
            if len > longest_chain {
                longest_chain = len;
                output = i32::try_from(i).unwrap();
            }
            i += 1;
        }
        output
    }

    fn count_chain(n: usize, values: &mut [i32]) -> i32 {
        if values[n] != 0 {
            return values[n];
        }
        if n == 1 {
            return 1;
        }
        if n % 2 == 0 {
            values[n] = 1 + Self::count_chain(n / 2, values);
        } else {
            values[n] = 2 + Self::count_chain((3 * n + 1) / 2, values);
        }
        values[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        const TARGET: usize = 20;
        assert_eq!(18, Euler::longest_collatz_sequence(TARGET))
    }

    #[test]
    fn case_2() {
        const TARGET: usize = 1_000_000;
        assert_eq!(837799, Euler::longest_collatz_sequence(TARGET))
    }
}
