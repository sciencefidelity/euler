use std::path::Path;
use utils::read_lines;

pub struct Euler;

const TRUNK_FACTOR: usize = 1000;

impl Euler {
    pub fn large_sum<P>(path: P, n: usize) -> usize
    where
        P: AsRef<Path>,
    {
        read_lines(path)
            .map(|line| line.ok().unwrap())
            .map(|s| s[0..=n].parse().unwrap_or(0))
            .sum::<usize>()
            / TRUNK_FACTOR
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PATH: &str = "data/pr_0013.txt";

    #[test]
    fn case_1() {
        assert_eq!(5_537_376_230, Euler::large_sum(PATH, 10));
    }

    #[test]
    fn case_2() {
        assert_eq!(55_373, Euler::large_sum(PATH, 5));
    }

    #[test]
    fn case_3() {
        assert_eq!(5, Euler::large_sum(PATH, 1));
    }

    #[test]
    fn case_4() {
        assert_eq!(5_537_376_230_390_876, Euler::large_sum(PATH, 16));
    }
}
