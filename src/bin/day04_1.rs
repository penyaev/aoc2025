use std::fmt;
use std::fmt::Formatter;
use aoc_2025::utils::{Grid, input_file, read_lines};

#[derive(Clone)]
enum Cell {
    PaperRoll,
    Empty,
}

impl Cell {
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        matches!(self, Cell::Empty)
    }

    pub fn is_paper_roll(&self) -> bool {
        matches!(self, Cell::PaperRoll)
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell::Empty
    }
}

impl TryFrom<char> for Cell {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '@' => Ok(Cell::PaperRoll),
            '.' => Ok(Cell::Empty),
            _ => Err(format!("Invalid character for Cell: {}", c)),
        }
    }
}


impl fmt::Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::PaperRoll => write!(f, "@"),
        }
    }
}

fn main() {
    let input = read_lines(input_file(4, false)).expect("failed to read input");

    let mut grid = Grid::<Cell>::new(0, 0);
    for line_result in input {
        let line = line_result.expect("failed to read line");

        grid.set_width(line.len());
        grid.add_row();

        for (i, c) in line.chars().enumerate() {
            grid.set(
                i,
                grid.height() - 1,
                Cell::try_from(c).expect("invalid cell character"),
            )
            .expect("failed to set cell")
        }
    }

    let mut result = 0;
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            if grid.get(x, y).unwrap().is_paper_roll() && (grid.neighbours(x, y).iter().filter(|(_, _, cell)| cell.is_paper_roll()).collect::<Vec<_>>()).len() < 4 {
                result += 1;
            }
        }
    }

    println!("{}", result)
}
