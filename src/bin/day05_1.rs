use aoc_2025::utils::{input_file, read_lines, RangeSet};

fn main() {
    let input = read_lines(input_file(5, false)).expect("failed to read input");

    let mut result = 0u64;
    let mut ranges = true;
    let mut rs = RangeSet::new();
    for line_result in input {
        let line = line_result.expect("failed to parse line");

        if line.is_empty() {
            ranges = false;
            continue;
        }

        if ranges {
            let parts: Vec<u64> = line
                .split('-')
                .map(|x| x.parse().expect("failed to parse number"))
                .collect();
            assert_eq!(parts.len(), 2);

            let (from, to) = (parts[0], parts[1]+1); // +1 to make the interval inclusive on both ends

            let _ = rs.insert(from, to); // ignore duplicate range error
        } else {
            let value: u64 = line.parse().expect("failed to parse number");

            if rs.contains(value) {
                result += 1;
            }

        }

    }

    println!("{}", result)
}
