use std::time::{SystemTime, UNIX_EPOCH};

use crate::math::{point, ray::Ray, transformations::Transformation};

use super::intersections::{Intersection, Intersections};

#[derive(Clone, PartialEq, Debug)]
pub struct Sphere {
    pub uid: u128,
    pub transformation: Transformation,
}

impl Sphere {
    /// Creates a new `Sphere`.
    pub fn new() -> Self {
        Self {
            uid: time_now(),
            transformation: Transformation::identity(),
        }
    }

    pub fn intersects(&self, ray: Ray) -> Option<Intersections> {
        let sphere_to_ray = ray.origin - point::ORIGIN;

        let a = ray.direction.dot_product(&ray.direction);
        let b = 2.0 * ray.direction.dot_product(&sphere_to_ray);
        let c = sphere_to_ray.dot_product(&sphere_to_ray) - 1.0;

        let discriminant = b.powf(2.0) - 4.0 * a * c;

        if discriminant < 0.0 {
            None
        } else {
            let t1: f64 = (-b - discriminant.sqrt()) / (2.0_f64 * a);
            let t2: f64 = (-b + discriminant.sqrt()) / (2.0_f64 * a);
            let object = self;

            let i1 = Intersection::new(t1, object);
            let i2 = Intersection::new(t2, object);

            Some(i1.agregate(&i2))
        }
    }
}

fn time_now() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time Moved /backwards/! ")
        .as_nanos()
}
