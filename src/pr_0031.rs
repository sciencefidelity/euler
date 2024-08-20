use std::cmp::Ordering;

pub struct Euler;

impl Euler {
    pub fn coin_sums(candidates: &[i32], target: i32) -> i32 {
        let mut coin_sums = CoinSums::new(candidates, target);
        coin_sums.backtrack(target, 0);
        coin_sums.count
    }
}

struct CoinSums<'a> {
    candidates: &'a [i32],
    target: i32,
    count: i32,
}

impl<'a> CoinSums<'a> {
    pub const fn new(candidates: &'a [i32], target: i32) -> Self {
        Self {
            candidates,
            target,
            count: 0,
        }
    }

    pub fn backtrack(&mut self, remain: i32, start: usize) {
        match remain.cmp(&0) {
            Ordering::Less => {}
            Ordering::Equal => self.count += 1,
            Ordering::Greater => {
                for (i, coin) in self.candidates.iter().enumerate().skip(start) {
                    self.backtrack(remain - coin, i);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const COINS: [i32; 8] = [1, 2, 5, 10, 20, 50, 100, 200];

    #[test]
    fn case_1() {
        assert_eq!(Euler::coin_sums(&COINS, 200), 73_682);
    }
}
