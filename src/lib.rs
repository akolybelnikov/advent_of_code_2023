use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

// read .txt file and return the lines as a vector of strings
pub fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buf = BufReader::new(file);
    buf.lines().collect()
}