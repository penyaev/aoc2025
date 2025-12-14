use aoc_2025::utils::{input_file, read_lines};

fn main() {
    let input = read_lines(input_file(7, false)).expect("failed to read input");

    let mut result = 0u64;
    let mut prev: Option<String> = None;
    let mut rays: Vec<bool> = Vec::new();
    for line_result in input {
        let line = line_result.expect("failed to read line");

        if prev.is_none() {
            rays.resize(line.len(), false);
            prev = Some(line);
            continue;
        }

        for i in 0..rays.len() {
            let prev_byte = prev.as_ref().unwrap().as_bytes()[i];
            let cur_byte = line.as_bytes()[i];

            if prev_byte == 'S' as u8 {
                rays[i] = true;
            }

            if cur_byte == '.' as u8 {
                // nothing, ray stays unchanged
            } else if cur_byte == '^' as u8 && rays[i] {
                if i >= 1 {
                    rays[i-1] = true;
                }
                if i < rays.len()-1 {
                    rays[i+1] = true;
                }
                rays[i] = false;
                result += 1;
            }
        }
    }

    println!("{}", result)
}
