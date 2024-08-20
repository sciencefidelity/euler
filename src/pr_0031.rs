use std::cmp::Ordering;

pub struct Euler;

const COINS: [i32; 8] = [1, 2, 5, 10, 20, 50, 100, 200];

impl Euler {
    pub fn coin_sums(target: i32) -> i32 {
        let mut coin_sums = CoinSums::new(target);
        coin_sums.backtrack(target, 0);
        coin_sums.count
    }
}

struct CoinSums {
    target: i32,
    count: i32,
}

impl CoinSums {
    pub const fn new(target: i32) -> Self {
        Self { target, count: 0 }
    }

    pub fn backtrack(&mut self, remain: i32, start: usize) {
        match remain.cmp(&0) {
            Ordering::Less => {}
            Ordering::Equal => self.count += 1,
            Ordering::Greater => {
                for (i, coin) in COINS.iter().enumerate().skip(start) {
                    self.backtrack(remain - coin, i);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Euler::coin_sums(200), 73682);
    }
}
