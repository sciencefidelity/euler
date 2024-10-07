use std::{cmp::max, path::Path};
use utils::read_lines;

pub fn maximum_path_sum_ii<P>(path: P) -> i32
where
    P: AsRef<Path>,
{
    let mut triangle: Vec<Vec<i32>> = read_lines(path)
        .map(|line| line.ok().unwrap())
        .map(|s| {
            s.split_whitespace()
                .map(|s| s.parse().unwrap_or(0))
                .collect::<Vec<i32>>()
        })
        .collect();

    for i in 1..triangle.len() {
        for j in 0..triangle[i].len() {
            if j == 0 {
                triangle[i][j] += triangle[i - 1][j];
            } else if j == triangle[i].len() - 1 {
                triangle[i][j] += triangle[i - 1][j - 1];
            } else {
                triangle[i][j] += max(triangle[i - 1][j], triangle[i - 1][j - 1]);
            }
        }
    }
    *triangle[triangle.len() - 1].iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_path_sum_ii() {
        assert_eq!(7273, maximum_path_sum_ii("data/pr_0067.txt"));
    }
}
