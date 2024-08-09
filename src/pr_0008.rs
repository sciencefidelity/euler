use std::fs;

const ASCII_OFFSET: i32 = 48;

pub struct Euler;

impl Euler {
    pub fn largest_product_in_a_series(n: usize) -> i64 {
        let file = fs::read("src/pr_0008.txt").unwrap();
        let digits: Vec<u8> = file.trim_ascii_end().to_vec();
        let mut digits: Vec<i64> = digits.iter().map(|&x| i64::from(x)).collect();
        for num in digits.iter_mut() {
            *num -= 48;
        }
        let mut max_product: i64 = 0;
        for subarray in digits.windows(n) {
            let product = subarray.iter().product::<i64>();
            max_product = max_product.max(product);
        }
        max_product
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(5832, Euler::largest_product_in_a_series(4));
    }

    #[test]
    fn case_2() {
        assert_eq!(23514624000, Euler::largest_product_in_a_series(13));
    }
}
