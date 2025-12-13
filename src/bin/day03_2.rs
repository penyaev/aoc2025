use aoc_2025::{input_file, prepend_digit, read_lines};


const MAX_LEN: usize = 12;
fn solve(digits: Vec<u8>) -> u64 {
    let mut dp = vec![vec![0; digits.len() + 1]; MAX_LEN + 1];

    for l in 1..=MAX_LEN {
        for i in 1..=digits.len() {
            let digit = digits[digits.len()-i];

            let prev = dp[l][i-1];
            let added = if i == 1 || l == 1 { digit as u64 } else { prepend_digit(dp[l-1][i-1], digit) };

            dp[l][i] = if added > prev { added } else { prev };
        }
    }

    dp[MAX_LEN][digits.len()]
}

fn main() {
    let input = read_lines(input_file(3, false)).expect("failed to read input");

    let mut result = 0u64;

    for line_result in input {
        let line = line_result.expect("failed to read line");

        let digits: Vec<u8> = line.chars()
            .map(|c| c.to_digit(10).expect("not a digit") as u8)
            .collect();

        let jolts = solve(digits);

        // println!("{}: {}", line, jolts);

        result += jolts;
    }

    println!("{}", result)
}
