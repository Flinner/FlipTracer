use super::colors::Color;
use crate::features::colors;

#[derive(Debug)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            width,
            height,
            grid: vec![vec![colors::BLACK; width]; height],
        }
    }

    pub fn write(self: &mut Self, x: usize, y: usize, color: Color) {
        //(&self.grid[width][height]) = &color;
        self.grid[x][y] = color;
    }

    pub fn get(self, x: usize, y: usize) -> Color {
        self.grid[x][y]
    }
}
