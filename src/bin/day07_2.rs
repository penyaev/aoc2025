use aoc_2025::utils::{input_file, read_lines};

fn main() {
    let input = read_lines(input_file(7, false)).expect("failed to read input");

    let mut prev: Option<String> = None;
    let mut rays: Vec<u64> = Vec::new();
    for line_result in input {
        let line = line_result.expect("failed to read line");

        if prev.is_none() {
            rays.resize(line.len(), 0);
            prev = Some(line);
            continue;
        }

        let mut new_rays = vec![0; rays.len()];

        for i in 0..rays.len() {
            let prev_byte = prev.as_ref().unwrap().as_bytes()[i];
            let cur_byte = line.as_bytes()[i];

            if prev_byte == 'S' as u8 {
                rays[i] = 1;
            }

            if cur_byte == '.' as u8 {
                new_rays[i] += rays[i];
            } else if cur_byte == '^' as u8 {
                if i >= 1 {
                    new_rays[i-1] += rays[i];
                }
                if i < rays.len()-1 {
                    new_rays[i+1] += rays[i];
                }
                new_rays[i] = 0;
            }
        }

        rays = new_rays;

        // println!("{:?}", rays);
        prev = Some(line);
    }



    println!("{}", rays.iter().sum::<u64>())
}
