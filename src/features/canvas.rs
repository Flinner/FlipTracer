use super::colors::Color;
use crate::features::colors;

#[derive(Debug)]
/// Canvas, a grid of pixels, with `height` and `width` (for faster access)
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<Color>>,
}

impl Canvas {
    /// Returns a new blank `Canvas` with `colors::BLACK`
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            width,
            height,
            grid: vec![vec![colors::BLACK; width]; height],
        }
    }

    /// Modify (`x`, `y`) to `color`. Mutates `self`
    pub fn write(self: &mut Self, x: usize, y: usize, color: Color) {
        //(&self.grid[width][height]) = &color;
        self.grid[x][y] = color;
    }
    /// Get `Color` at point (`x`,`y`). Equivalent to `self.grid[x][y]`
    pub fn get(self, x: usize, y: usize) -> Color {
        self.grid[x][y]
    }
}
