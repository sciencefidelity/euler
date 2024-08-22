pub use math;

pub struct Euler;

impl Euler {
    pub fn smallest_multiple(n: u64) -> u64 {
        (1..=n).reduce(math::lcm).unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(2520, Euler::smallest_multiple(10));
    }

    #[test]
    fn case_2() {
        assert_eq!(232_792_560, Euler::smallest_multiple(20));
    }
}
