use crate::math::{point::Point, transformations::Transformation};

use super::color::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pattern {
    pub a: Color,
    pub b: Color,
    pub transformation: Transformation,
}

impl Pattern {
    pub fn new(a: Color, b: Color, transformation: Transformation) -> Self {
        Self {
            a,
            b,
            transformation,
        }
    }

    // returns the stripe pattern at point
    /// uses pattern space point, not World-Space, or Object-Space!
    pub fn at(&self, pattern_point: Point) -> Color {
        if pattern_point.x.floor() % 2.0 == 0.0 {
            self.a
        } else {
            self.b
        }
    }
}
