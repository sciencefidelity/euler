pub struct Euler;

impl Euler {
    pub fn even_fibonacci_numbers(limit: usize) -> usize {
        let mut sum = 0;
        for num in Self::fibonacci() {
            if num > limit {
                break;
            }
            if num % 2 == 0 {
                sum += num;
            }
        }
        sum
    }

    fn fibonacci() -> impl Iterator<Item = usize> {
        let mut state = (0, 1);
        std::iter::from_fn(move || {
            state = (state.1, state.0 + state.1);
            Some(state.0)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let sum: usize = [2, 8, 34].iter().sum();
        assert_eq!(sum, Euler::even_fibonacci_numbers(90));
    }

    #[test]
    fn case_2() {
        let sum: usize = 4613732;
        assert_eq!(sum, Euler::even_fibonacci_numbers(4_000_000));
    }
}
