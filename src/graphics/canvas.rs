use super::color::Color;
use crate::graphics::color;

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
        Canvas::new_color(width, height, color::BLACK)
    }

    /// Returns a new blank `Canvas` with a specified `Color`
    pub fn new_color(width: usize, height: usize, color: Color) -> Self {
        Canvas {
            width,
            height,
            grid: vec![vec![color; width]; height],
        }
    }

    /// Modify (`x`, `y`) to `color`. Mutates `self`
    pub fn write(&mut self, x: usize, y: usize, color: Color) {
        // make sure it is not out of bounds
        if self.height > y && self.width > x {
            self.grid[y][x] = color;
        }
    }
    /// Get `Color` at point (`x`,`y`). Equivalent to `self.grid[x][y]`
    pub fn get(self, x: usize, y: usize) -> Color {
        self.grid[y][x]
    }
}
