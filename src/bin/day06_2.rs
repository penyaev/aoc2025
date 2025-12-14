use aoc_2025::utils::input_file;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

fn get_newline_offsets(file_name: &String) -> Vec<u64> {
    let mut file = File::open(file_name).expect("failed to open file");

    let mut buf = [0u8; 100];
    let mut result = Vec::new();

    let mut offset = 0usize;

    loop {
        let n = file.read(&mut buf).unwrap();
        if n == 0 {
            break;
        }

        for i in 0..n {
            if buf[i] == b'\n' {
                result.push((i+offset) as u64);
            }
        }

        offset += n;
    }

    result
}

struct ReaderIter<R: Read + Seek> {
    file: R,
    offsets: Vec<u64>,
    pos: u64,
}

impl<R: Read + Seek> ReaderIter<R> {
    fn new(file: R, offsets: Vec<u64>) -> Self {
        Self {
            file,
            offsets,
            pos: 0,
        }
    }
}

impl<R: Read + Seek> Iterator for ReaderIter<R> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = [0u8; 1];
        let mut bytes = Vec::new();
        for offset in &self.offsets {
            self.file.seek(SeekFrom::Start(*offset + self.pos)).unwrap();

            let n = self.file.read(&mut buf).expect("failed to read line");
            if n == 0 {
                return None;
            }

            bytes.push(buf[0]);
        }
        self.pos += 1;

        Some(String::from_utf8(bytes).unwrap())
    }
}

fn main() {
    let file_name = input_file(6, false);
    let mut offsets: Vec<u64> = get_newline_offsets(&file_name).iter().map(|o| o + 1).collect();
    offsets.insert(0, 0);
    // println!("offsets: {:?}", offsets);

    let iter = ReaderIter::new(File::open(&file_name).unwrap(), offsets);

    let mut current_op = String::from("");
    let mut current_nums: Vec<u64> = Vec::new();
    let mut result = 0u64;
    for (i, s) in iter.enumerate() {
        // println!("line {}: [{}]", i, s);
        if s.trim().is_empty() {
            // println!("{:?} op:{}", current_nums, current_op);
            if !current_op.is_empty() {
                result += match current_op.as_str() {
                    "+" => current_nums.iter().sum::<u64>(),
                    "*" => current_nums.iter().product::<u64>(),
                    _ => panic!("unknown op: {}", current_op),
                }
            }
            current_nums.clear();
            current_op.clear();
            continue;
        }
        let num: u64 = s[..s.len() - 1].trim().parse().expect(&format!("failed to parse string into number {} at pos {}", s, i));
        current_nums.push(num);

        let op = s[s.len() - 1..].trim().to_string();
        if !op.is_empty() {
            current_op = op;
        }
    }

    result += match current_op.as_str() {
        "+" => current_nums.iter().sum::<u64>(),
        "*" => current_nums.iter().product::<u64>(),
        _ => panic!("unknown op: {}", current_op),
    };

    println!("{}", result);
}
