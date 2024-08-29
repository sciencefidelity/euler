use integer::Integer;
use std::{fs, path::Path};

pub struct Euler;

const BYTE_OFFSET: u8 = 64;

impl Euler {
    pub fn coded_triangle_numbers<P>(path: P) -> i32
    where
        P: AsRef<Path>,
    {
        let data = fs::read(path).expect("failed to open file");
        let (mut word_val, mut count) = (0, 0);
        for b in data {
            match b {
                b'"' | b',' if word_val == 0 => {}
                b'"' => {
                    if word_val.is_triangle() {
                        count += 1;
                    }
                    word_val = 0;
                }
                _ if b.is_ascii_alphabetic() => word_val += u32::from(b - BYTE_OFFSET),
                _ => {}
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PATH: &str = "data/pr_0042.txt";

    #[test]
    fn test_coded_triangle_numbers() {
        assert_eq!(Euler::coded_triangle_numbers(PATH), 162);
    }
}
