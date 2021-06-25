use std::time::{SystemTime, UNIX_EPOCH};

use crate::math::{point, ray::Ray};

use super::intersection::Intersection;

pub struct Sphere {
    pub uid: u128,
}

impl Sphere {
    /// Creates a new `Sphere`.
    pub fn new() -> Self {
        Self { uid: time_now() }
    }

    pub fn intersects(&self, ray: Ray) -> Option<Intersection> {
        let sphere_to_ray = ray.origin - point::ORIGIN;

        let a = ray.direction.dot_product(&ray.direction);
        let b = 2.0 * ray.direction.dot_product(&sphere_to_ray);
        let c = sphere_to_ray.dot_product(&sphere_to_ray) - 1.0;

        let discriminant = b.powf(2.0) - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        } else {
            let t1: f64 = (-b - discriminant.sqrt()) / (2.0_f64 * a);
            let t2: f64 = (-b + discriminant.sqrt()) / (2.0_f64 * a);
            let list = vec![t1, t2];
            return Some(Intersection { list });
        }
    }
}

fn time_now() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time Moved /backwards/! ")
        .as_nanos()
}
