use super::color::Color;
use crate::math::point::Point;

/// Point of light in 3d space, with no size,
/// intensisty is defined by `Color`
pub struct PointLight {
    pub position: Point,
    /// color is also intensisty
    pub color: Color,
}

impl PointLight {
    pub fn new(position: Point, color: Color) -> Self {
        Self { position, color }
    }
}
