use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// # Panics
///
/// Will panic if file is not found.
pub fn read_lines<P>(file: P) -> io::Lines<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(file).expect("failed to open file");
    io::BufReader::new(file).lines()
}
