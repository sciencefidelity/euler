pub use math;

pub fn smallest_multiple(n: u64) -> u64 {
    (1..=n).reduce(math::lcm).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smallest_multiple() {
        assert_eq!(232_792_560, smallest_multiple(20));
    }
}
