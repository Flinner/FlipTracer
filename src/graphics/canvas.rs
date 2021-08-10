use super::color::Color;
use crate::graphics::color;

#[derive(Debug)]
/// Canvas, a grid of pixels, with `height` and `width` (for faster access)
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Color>,
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
            grid: vec![color; height * width],
        }
    }

    /// Modify (`x`, `y`) to `color`. Mutates `self`
    pub fn write(&mut self, x: usize, y: usize, color: Color) {
        // make sure it is not out of bounds
        // if self.height > y && self.width > x {
        self.grid[y * self.width + x] = color;
        // }
    }
    /// similar to `write` but it avoids calculating `x` and `y` values
    pub fn write_i(&mut self, i: usize, color: Color) {
        // make sure it is not out of bounds
        self.grid[i] = color;
    }
    /// Get `Color` at point (`x`,`y`). Equivalent to `self.grid[x][y]`
    pub fn get(self, x: usize, y: usize) -> Color {
        self.grid[y * self.width + x]
    }
    /// similar to `get` but it avoids calculating `x` and `y` values
    pub fn get_i(self, i: usize) -> Color {
        self.grid[i]
    }
}
