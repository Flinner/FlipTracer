use super::{point::Point, vector::Vector};

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    /// Returns a new `Ray` struct
    pub fn new(origin: Point, direction: Vector) -> Ray {
        Ray { origin, direction }
    }
}
