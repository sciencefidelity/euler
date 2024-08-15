// use rustc_hash::FxHashMap;
pub struct Euler;

impl Euler {
    pub const fn longest_collatz_sequence(n: usize) -> usize {
        let (mut output, mut longest_chain, mut i) = (1, 0, n / 2);
        // let mut values = vec![0; 28_495_741_760 + 1];
        // let mut values = FxHashMap::default();
        while i < n {
            // let len = Self::count_chain(i, &mut values);
            let len = Self::count_chain(i);
            if len > longest_chain {
                longest_chain = len;
                output = i;
            }
            i += 1;
        }
        output
    }

    // fn count_chain(n: usize, values: &mut FxHashMap<usize, usize>) -> usize {
    const fn count_chain(mut n: usize) -> usize {
        // if let Some(val) = values.get(&n) {
        //     return *val;
        // }
        // if n == 1 {
        //     return 1;
        // }
        // if n % 2 == 0 {
        //     let val = Self::count_chain(n / 2, values);
        //     values.insert(n, val);
        // } else {
        //     let val = Self::count_chain((3 * n + 1) / 2, values);
        //     values.insert(n, val);
        // }
        // *values.get(&n).unwrap()

        let mut count = 1;
        while n != 1 {
            if n % 2 == 0 {
                n /= 2;
                count += 1;
            } else {
                n = (3 * n + 1) / 2;
                count += 2;
            }
        }
        count

        // let start: u64 = input - 1;
        // input *= input & 1;
        // let mut count: u64 = 0;
        // while input > 1 {
        //     count += 1;
        //     input *= start.wrapping_sub(input) >> 63;
        //     let m: u64 = (input & 1) ^ 1;
        //     input = m * (input >> 1) + (m ^ 1) * ((input * 3 + 1) / 2);
        // }
        // count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        const TARGET: usize = 20;
        assert_eq!(18, Euler::longest_collatz_sequence(TARGET));
    }

    #[test]
    fn case_2() {
        const TARGET: usize = 1_000_000;
        assert_eq!(837_799, Euler::longest_collatz_sequence(TARGET));
    }

    #[test]
    fn case_3() {
        const TARGET: usize = 100_000_000;
        assert_eq!(63_728_127, Euler::longest_collatz_sequence(TARGET));
    }
}
