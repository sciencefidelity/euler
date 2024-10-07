use std::path::Path;
use utils::read_lines;

const TRUNK_FACTOR: usize = 1000;

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

#[cfg(test)]
mod tests {
    use super::*;

    const PATH: &str = "data/pr_0013.txt";

    #[test]
    fn test_large_sum() {
        assert_eq!(5_537_376_230, large_sum(PATH, 10));
    }
}
