use aoc_2025::{input_file, read_lines};

fn num_len(x: u64) -> u8 {
    let mut l: u8 = 1;
    while x >= 10u64.pow(l as u32) {
        l += 1;
    }

    l
}


fn prepend_digit(num: u64, digit: u8) -> u64 {
    let num_len = num_len(num);

    let pow = 10u64.pow((num_len) as u32);

    (digit as u64) * pow + num
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prepend_digit() {
        assert_eq!(prepend_digit(123, 9), 9123);
        assert_eq!(prepend_digit(0, 1), 10);
        assert_eq!(prepend_digit(5, 1), 15);
        assert_eq!(prepend_digit(1000, 5), 51000);
        assert_eq!(prepend_digit(98765, 1), 198765);
    }
}