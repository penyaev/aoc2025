use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn input_file(day: u8, small: bool) -> String {
    format!("inputs/day{:02}{}.txt", day, if small { "_small" } else { "" })
}
pub fn read_lines(file_name: String) -> std::io::Result<std::io::Lines<BufReader<std::fs::File>>> {
    let file = File::open(file_name)?;

    Ok(BufReader::new(file).lines())
}

pub fn read_split(file_name: String, separator: u8) -> std::io::Result<std::io::Split<BufReader<std::fs::File>>> {
    let file = File::open(file_name)?;

    Ok(BufReader::new(file).split(separator))
}
