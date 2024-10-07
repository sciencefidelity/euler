#[must_use]
pub fn int_to_binary_string(n: i32) -> String {
    format!("{n:b}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_to_binary_string() {
        assert_eq!(int_to_binary_string(585), "1001001001".to_owned());
    }
}
