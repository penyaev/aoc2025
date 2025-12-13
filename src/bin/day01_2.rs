use aoc_2025::{read_lines, input_file};

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


        let before = pos;
        pos += delta;
        result += (pos / 100 - before / 100).abs();

        if before >= 0 && pos < 0 {
            result += 1;
        }
        if before == 0 && pos < 0 {
            result -= 1;
        }
        if before > 0 && pos == 0 {
            result += 1;
        }


        while pos < 0 {
            pos += 100;
        }
        while pos > 99 {
            pos -= 100;
        }

        // println!("{}: {} -> {} = {}", line, before, pos, result);
    }

    println!("{}", result);
}