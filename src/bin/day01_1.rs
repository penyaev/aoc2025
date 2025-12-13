use aoc_2025::utils::{input_file, read_lines};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

fn main() {
    let day = 1;
    let input = read_lines(input_file(day, false)).expect(&format!("failed to read input for day {}", day));
    let mut pos = 50;
    let mut result = 0;

    for line_result in input {
        let line = line_result.expect("failed to read line from input");

        let direction = match &line[0..1] {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("invalid direction"),
        };

        let steps = line[1..].parse::<i32>().expect("failed to parse steps");

        let delta = match direction {
            Direction::Left => steps * -1,
            Direction::Right => steps,
        };

        pos += delta;
        while pos < 0 {
            pos += 100;
        }
        while pos > 99 {
            pos -= 100;
        }

        if pos == 0 {
            result += 1;
        }
    }

    println!("{}", result);
}