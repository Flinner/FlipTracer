use std::time::{SystemTime, UNIX_EPOCH};

use crate::math::ray::Ray;

use super::intersection::Intersection;

pub struct Sphere {
    pub uid: u128,
}

impl Sphere {
    /// Creates a new `Sphere`.
    pub fn new() -> Self {
        Self { uid: time_now() }
    }

    pub fn intersects(&self, _ray: Ray) -> Intersection {
        todo!()
    }
}

fn time_now() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time Moved /backwards/! ")
        .as_nanos()
}
