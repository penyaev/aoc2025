use aoc_2025::utils::{input_file, read_lines};
use std::cmp::max;

fn main() {
    let input = read_lines(input_file(9, false)).expect("failed to read input");

    let mut points: Vec<(u64, u64)> = Vec::new();
    let mut best = 0u64;
    for line_result in input {
        let line = line_result.expect("failed to parse line");

        let nums: Vec<u64> = line
            .split(',')
            .map(|x| x.parse().expect("failed to parse number"))
            .collect();

        assert_eq!(nums.len(), 2);
        let (x, y) = (nums[0], nums[1]);

        for point in &points {
            best = max(best, (point.0.abs_diff(x) + 1) * (point.1.abs_diff(y) + 1));
        }

        points.push((x, y));
    }

    println!("{}", best);
}
