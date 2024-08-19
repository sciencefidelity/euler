pub struct Euler;

const DIRS: &[(i32, i32); 4] = &[(0, -1), (1, 0), (0, 1), (-1, 0)];

impl Euler {
    pub fn number_spiral_diagonals(n: usize) -> usize {
        assert!(n % 2 != 0);
        let mut grid = vec![vec![0; n]; n];
        grid[n / 2][n / 2] = 1;
        let mut m = i32::try_from(n).unwrap();
        let (mut curr, mut direction) = (n.pow(2), 0);
        let mut i = 0;
        let mut step = n - 1;
        loop {
            let ring = step * 4;
            let (mut row, mut col) = (i, m - 1);
            for _ in 0..ring {
                grid[usize::try_from(row).unwrap()][usize::try_from(col).unwrap()] = curr;
                row += DIRS[direction].0;
                col += DIRS[direction].1;
                if Self::is_border(m, i, row, col) {
                    direction = (direction + 1) % 4;
                }
                curr -= 1;
            }
            if curr == 1 {
                break;
            }
            step -= 2;
            m -= 1;
            i += 1;
        }
        let mut result = 0;
        let mut l = 0_i32;
        let mut r = i32::try_from(n - 1).unwrap();
        for row in grid {
            result += row[usize::try_from(l).unwrap()];
            result += row[usize::try_from(r).unwrap()];
            l += 1;
            r -= 1;
        }
        result - 1
    }

    const fn is_border(n: i32, i: i32, row: i32, col: i32) -> bool {
        (row == i && col == i)
            || (row == n - 1 && col == i)
            || (row == n - 1 && col == n - 1)
            || (row == i && col == n - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Euler::number_spiral_diagonals(3), 25);
    }

    #[test]
    fn case_2() {
        assert_eq!(Euler::number_spiral_diagonals(5), 101);
    }

    #[test]
    fn case_3() {
        assert_eq!(Euler::number_spiral_diagonals(7), 261);
    }

    #[test]
    fn case_4() {
        assert_eq!(Euler::number_spiral_diagonals(1001), 669_171_001);
    }
}
