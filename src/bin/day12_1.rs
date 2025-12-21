use aoc_2025::utils::{Grid, input_file, read_lines};
use std::collections::{HashMap, VecDeque};
use std::fmt::Display;

enum ReadingMode {
    Header,
    ShapeBody(usize),
}

#[derive(Clone, Copy, Default)]
struct Cell(bool);

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if self.0 { '#' } else { '.' })
    }
}
struct Shape {
    grid: Grid<Cell>,
    filled_count: usize,
}

impl Shape {
    fn new() -> Self {
        Self {
            grid: Grid::new(0, 0),
            filled_count: 0,
        }
    }

    fn add_line(&mut self, line: &str) {
        self.grid.set_width(line.len());
        self.grid.add_row();
        for (x, c) in line.chars().enumerate() {
            let value = c == '#';
            self.grid.set(x, self.grid.height() - 1, Cell(value)).unwrap();
            if value {
                self.filled_count += 1;
            }
        }
    }
}

fn main() {
    let input = read_lines(input_file(12, false)).expect("failed to read input");

    let mut mode = ReadingMode::Header;
    let mut next_shape_index = 0usize;
    let mut current_shape = Shape::new();
    let mut shapes: Vec<Shape> = Vec::new();
    let mut result = 0usize;
    for line_result in input {
        let line = line_result.expect("failed to parse line");

        match mode {
            ReadingMode::Header => {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() == 1 {
                    mode = ReadingMode::ShapeBody(next_shape_index);
                } else {
                    assert_eq!(parts.len(), shapes.len() + 1);
                    let dimensions: Vec<usize> = parts[0].trim_matches(':').split('x').map(|x| x.parse().unwrap()).collect();
                    assert_eq!(dimensions.len(), 2);
                    let shapes_counts: Vec<usize> = parts[1..].iter().map(|s| s.parse().unwrap()).collect();
                    // println!("{:?} {:?}", dimensions, shapes_counts);


                    let (width, height) = (dimensions[0], dimensions[1]);
                    let full_3x3_squares = (3 * (width / 3)) * (3 * (height / 3)) / 9;
                    let shapes_required = shapes_counts.iter().sum();
                    if full_3x3_squares >= shapes_required {
                        result += 1;
                    } else {
                        let cells_required = shapes_counts.iter().enumerate().map(|(i, c)| shapes[i].filled_count * c).sum();
                        let cells_available = width * height;
                        if cells_available >= cells_required {
                            panic!("looks like we actually have to have an algorithm to solve this");
                        }
                    }
                }
            }
            ReadingMode::ShapeBody(cur_shape_index) => {
                if line.is_empty() {
                    assert_eq!(current_shape.grid.width(), 3);
                    assert_eq!(current_shape.grid.height(), 3);
                    shapes.push(current_shape);
                    current_shape = Shape::new();

                    mode = ReadingMode::Header;
                    next_shape_index += 1;
                } else {
                    current_shape.add_line(&line);
                }
            }
        }
    }

    println!("{}", result);
}
