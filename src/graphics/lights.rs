use crate::math::point::Point;

use super::color::Color;

/// Point of light in 3d space, with no size,
/// intensisty is defined by `Color`
pub struct Light {
    pub position: Point,
    pub color: Color,
}

impl Light {
    pub fn new(position: Point, color: Color) -> Self {
        Self { position, color }
    }
}
