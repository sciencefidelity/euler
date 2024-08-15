use std::fs;

pub struct Euler;

impl Euler {
    pub fn names_scores(path: &str) -> usize {
        let mut file = fs::read_to_string(path).unwrap();
        file = file.trim()[1..file.len() - 2].to_string();
        let mut names: Vec<&str> = file.split("\",\"").collect();
        names.sort_unstable();
        names
            .into_iter()
            .enumerate()
            .map(|(i, name)| {
                let mut sum = 0;
                name.bytes().for_each(|b| {
                    sum += usize::from(b - 64);
                });
                (i + 1) * sum
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(871_198_282, Euler::names_scores("src/pr_0022.txt"));
    }
}
