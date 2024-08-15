use std::cmp::max;
use std::path::Path;
use utils::read_lines;

pub struct Euler;

impl Euler {
    pub fn largest_product_in_a_grid<P>(path: P, target: usize) -> i64
    where
        P: AsRef<Path>,
    {
        let grid = Self::parse_grid(path);
        let mut largest_product = 0;
        let (m, n) = (grid.len(), grid[0].len());

        for i in 0..m {
            for j in 0..n {
                let (mut left, mut down, mut diag_a, mut diag_b) = (1, 1, 1, 1);
                for k in 0..target {
                    if j <= n - target {
                        left *= grid[i][j + k];
                    }
                    if i <= m - target {
                        down *= grid[i + k][j];
                    }
                    if i <= m - target && j <= n - target {
                        diag_a *= grid[i + k][j + k];
                    }
                    if j <= n - target && i >= target - 1 {
                        diag_b *= grid[i - k][j + k];
                    }
                }
                largest_product = max(largest_product, left);
                largest_product = max(largest_product, down);
                largest_product = max(largest_product, diag_a);
                largest_product = max(largest_product, diag_b);
            }
        }
        largest_product
    }

    fn parse_grid<P>(file: P) -> Vec<Vec<i64>>
    where
        P: AsRef<Path>,
    {
        const GRID_X: usize = 20;
        const GRID_Y: usize = 20;
        let mut grid = vec![vec![0; GRID_X]; GRID_Y];
        for (i, line) in read_lines(file).map_while(Result::ok).enumerate() {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .enumerate()
                .for_each(|(j, v)| grid[i][j] = v);
        }
        grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let path = "src/pr_0011.txt";
        assert_eq!(70_600_674, Euler::largest_product_in_a_grid(path, 4));
    }

    #[test]
    fn case_2() {
        let path = "src/pr_0011.txt";
        assert_eq!(811_502, Euler::largest_product_in_a_grid(path, 3));
    }

    #[test]
    fn case_3() {
        let path = "src/pr_0011.txt";
        assert_eq!(3_318_231_678, Euler::largest_product_in_a_grid(path, 5));
    }

    #[test]
    fn case_4() {
        let path = "src/pr_0011.txt";
        assert_eq!(188_210_512_710, Euler::largest_product_in_a_grid(path, 6));
    }
}
