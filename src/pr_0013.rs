use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Euler;

const TRUNK_FACTOR: usize = 1_000;

impl Euler {
    pub fn large_sum<P>(path: P, n: usize) -> usize
    where
        P: AsRef<Path>,
    {
        Self::read_lines(path)
            .map(|line| line.ok().unwrap())
            .map(|s| s[0..=n].parse().unwrap_or(0))
            .sum::<usize>()
            / TRUNK_FACTOR
    }

    fn read_lines<P>(file: P) -> io::Lines<io::BufReader<File>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(file).unwrap();
        io::BufReader::new(file).lines()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(5_537_376_230, Euler::large_sum("src/pr_0013.txt", 10));
    }

    #[test]
    fn case_2() {
        assert_eq!(55_373, Euler::large_sum("src/pr_0013.txt", 5));
    }

    #[test]
    fn case_3() {
        assert_eq!(5, Euler::large_sum("src/pr_0013.txt", 1));
    }

    #[test]
    fn case_4() {
        assert_eq!(
            5_537_376_230_390_876,
            Euler::large_sum("src/pr_0013.txt", 16)
        );
    }
}
