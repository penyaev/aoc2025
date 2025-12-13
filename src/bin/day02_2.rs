use std::collections::HashSet;
use aoc_2025::utils::{input_file, num_len, read_split};

fn invalid_ids(from: u64, to: u64) -> Vec<u64> {
    let mut result: Vec<u64> = Vec::new();
    for len in num_len(from)..=num_len(to) {
        let mut seen: HashSet<u64> = HashSet::new();
        for prefix_len in 1..=len/2 {
            if len % prefix_len != 0 {
                continue;
            }
            let repeat = len / prefix_len;

            for prefix in 10u64.pow((prefix_len-1) as u32)..10u64.pow(prefix_len as u32) {
                let mut num = 0u64;
                for _ in 0..repeat {
                    num *= 10u64.pow(prefix_len as u32);
                    num += prefix;
                }

                if num >= from && num <= to && seen.insert(num) {
                    result.push(num);
                }
            }
        }
    }

    result
}

fn main() {
    let input = read_split(input_file(2, false), b',').expect("failed to read input");

    let mut result: u64 = 0;
    for chunk_result in input {
        let chunk = chunk_result.expect("failed to read chunk from input");
        let s = String::from_utf8(chunk).expect("failed to parse chunk");

        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 2 {
            panic!("invalid input");
        }
        let from: u64 = parts[0].parse().expect("failed to parse from");
        let to: u64 = parts[1].parse().expect("failed to parse to");

        for invalid_id in invalid_ids(from, to) {
            // println!("[{}; {}] {}", from, to, invalid_id);
            result += invalid_id;
        }
    }

    println!("{}", result)
}
