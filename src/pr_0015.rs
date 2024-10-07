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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lattice_paths() {
        assert_eq!(137_846_528_820, lattice_paths(20));
    }
}
