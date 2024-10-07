use std::cmp::max;
use std::path::Path;
use utils::read_lines;

pub fn largest_product_in_a_grid<P>(path: P, target: usize) -> i64
where
    P: AsRef<Path>,
{
    let grid = parse_grid(path);
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

#[cfg(test)]
mod tests {
    use super::*;

    const PATH: &str = "data/pr_0011.txt";

    #[test]
    fn test_largest_product_in_a_grid() {
        assert_eq!(70_600_674, largest_product_in_a_grid(PATH, 4));
    }
}
