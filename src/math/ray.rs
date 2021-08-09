use super::{point::Point, transformations::Transformation, vector::Vector};

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    /// Returns a new `Ray` struct
    pub fn new(origin: Point, direction: Vector) -> Ray {
        Ray { origin, direction }
    }

    /// Returns the `Point` in  `distance` from the `origin: Point` in `direction: Vector`
    pub fn position(&self, distance: f64) -> Point {
        self.origin + self.direction * distance
    }

    pub fn transform(&self, transformation: Transformation) -> Self {
        Ray {
            origin: transformation * self.origin,
            direction: transformation * self.direction,
        }
    }
}
