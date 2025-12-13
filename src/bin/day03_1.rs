use aoc_2025::utils::{input_file, read_lines};

fn main() {
    let input = read_lines(input_file(3, false)).expect("failed to read input");

    let mut result = 0;

    for line_result in input {
        let line = line_result.expect("failed to read line");

        let mut max1: Option<u8> = None;
        let mut max_index = 0;
        for (i, c) in line[..line.len()-1].bytes().enumerate() {
             if max1 == None || c > max1.unwrap() {
                 max1 = Some(c);
                 max_index = i;
             }
        }

        let mut max2: Option<u8> = None;
        for (_, c) in line[max_index+1..].bytes().enumerate() {
            if max2 == None || c > max2.unwrap() {
                max2 = Some(c);
            }
        }

        let jolts = ((max1.unwrap() - 48) as i32) * 10 + ((max2.unwrap() - 48) as i32);
        println!("{}: {}", line, jolts);

        result += jolts;
    }

    println!("{}", result)
}
