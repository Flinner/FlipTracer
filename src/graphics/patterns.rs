use crate::math::point::Point;

use super::color::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StripePattern {
    pub a: Color,
    pub b: Color,
}

impl StripePattern {
    pub fn new(a: Color, b: Color) -> Self {
        Self { a, b }
    }

    // returns the stripe pattern at point
    pub fn at(&self, point: Point) -> Color {
        eprintln!("yad yad {}", point.x.floor() % 2.0);
        if point.x.floor() % 2.0 == 0.0 {
            self.a
        } else {
            self.b
        }
    }
}
