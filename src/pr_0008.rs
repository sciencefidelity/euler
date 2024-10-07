use std::fs;

pub fn largest_product_in_a_series(path: &str, n: usize) -> i64 {
    let file = fs::read(path).unwrap();
    let digits = file.trim_ascii_end().to_vec();
    let mut digits: Vec<i64> = digits.iter().map(|&x| i64::from(x)).collect();
    for num in &mut digits {
        *num -= 48;
    }
    let mut max_product: i64 = 0;
    for subarray in digits.windows(n) {
        let product = subarray.iter().product::<i64>();
        max_product = max_product.max(product);
    }
    max_product
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_product_in_a_series() {
        assert_eq!(
            23_514_624_000,
            largest_product_in_a_series("data/pr_0008.txt", 13)
        );
    }
}
