use std::fmt::{Display, Formatter};

pub struct Grid<Cell> {
    width: usize,
    height: usize,
    cells: Vec<Vec<Cell>>,
}

impl<Cell> Grid<Cell>
where
    Cell: Default + Clone
{
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![vec![Cell::default(); width]; height],
        }
    }

    pub fn set_height(&mut self, height: usize) {
        self.cells.resize(height, vec![Cell::default(); self.width]);
        self.height = height;
    }
    pub fn set_width(&mut self, width: usize) {
        for row in &mut self.cells {
            row.resize(width, Cell::default());
        }
        self.width = width;
    }

    pub fn add_row(&mut self) {
        self.set_height(self.height + 1);
    }

    pub fn add_column(&mut self) {
        self.set_width(self.width + 1);
    }

    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(&self.cells[y][x])
        }
    }

    pub fn neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize, &Cell)> {
        let mut neighbours = Vec::new();
        for dx  in -1isize..=1 {
            for dy in -1isize..=1 {
                if dx == 0 && dy == 0 { continue; }
                if dx + (x as isize) < 0 { continue; }
                if dy + (y as isize) < 0 { continue; }

                let xx = ((x as isize) + dx) as usize;
                let yy = ((y as isize) + dy) as usize;
                if let Some(cell) = self.get(xx, yy) {
                    neighbours.push((xx, yy, cell));
                }
            }
        }

        neighbours
    }

    pub fn set(&mut self, x: usize, y: usize, value: Cell) -> Result<(), String> {
        if x >= self.width || y >= self.height {
            return Err(format!("Coordinates ({}, {}) are out of bounds", x, y));
        }
        self.cells[y][x] = value;
        Ok(())
    }
}

impl<Cell> Display for Grid<Cell>
where
    Cell: Display + Clone + Default
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.get(x, y).unwrap())?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}