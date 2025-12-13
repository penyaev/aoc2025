use aoc_2025::{input_file, num_len, read_split};

struct InvalidIDIterator {
    to: u64,
    current: u64,
}

impl Iterator for InvalidIDIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.current > self.to {
                return None;
            }

            let len = num_len(self.current);
            if len % 2 == 1 {
                self.current = 10u64.pow(len as u32);
                continue;
            }

            let half_pow = 10u64.pow((len as u32)/2);
            let half = self.current / half_pow;
            let num = half * half_pow + half;

            if num < self.current {
                self.current = (half + 1) * half_pow;
                continue;
            }
            if num > self.to {
                return None;
            }

            self.current = num + 1;
            return Some(num);
        }
    }
}
fn invalid_ids(from: u64, to: u64) -> InvalidIDIterator {
    InvalidIDIterator {
        to,
        current: from,
    }
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
            //println!("[{}; {}] {}", from, to, invalid_id);
            result += invalid_id;
        }
    }

    println!("{}", result)
}
