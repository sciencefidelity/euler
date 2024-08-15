pub struct Euler;

impl Euler {
    // Pascals Triangle
    // pub fn lattice_paths(n: usize) -> usize {
    //     let mut grid = vec![vec![1; n + 1]; n + 1];
    //     for i in 1..=n {
    //         for j in 1..=n {
    //             grid[i][j] = grid[i - 1][j] + grid[i][j - 1];
    //         }
    //     }
    //     grid[n][n]
    // }

    // Combinatorial
    pub const fn lattice_paths(n: usize) -> usize {
        let mut result = 1;
        let mut i = 1;
        while i <= n {
            result = result * (n + i) / i;
            i += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(6, Euler::lattice_paths(2));
    }

    #[test]
    fn case_2() {
        assert_eq!(137_846_528_820, Euler::lattice_paths(20));
    }

    #[test]
    fn case_3() {
        assert_eq!(465_428_353_255_261_088, Euler::lattice_paths(31));
    }
}
