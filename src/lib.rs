use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_and_parse<T, P>(filename: P) -> Vec<T>
where
    P: AsRef<Path>,
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    read_lines(filename)
        .unwrap()
        .into_iter()
        .map(|depth| depth.unwrap().parse::<T>().expect("parse line"))
        .collect::<Vec<T>>()
}
