use aoc_2025::utils::{input_file, read_lines};

fn main() {
    let input = read_lines(input_file(6, false)).expect("failed to read input");

    let mut numbers: Vec<Vec<u64>> = Vec::new();
    let mut result = 0u64;
    for line_result in input {
        let line = line_result.expect("failed to parse line");

        if line.trim().starts_with(['*', '+']) {
            result = line
                .trim()
                .split_ascii_whitespace()
                .enumerate()
                .map(|(i, x)| match x {
                    "+" => numbers.iter().map(|y| y[i]).sum(),
                    "*" => numbers.iter().map(|y| y[i]).product(),
                    _ => panic!("unexpected operation"),
                })
                .reduce(|a, b| a + b)
                .unwrap();
        } else {
            numbers.push(
                line.trim()
                    .split_ascii_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect(),
            );
        }
    }

    println!("{}", result)
}
