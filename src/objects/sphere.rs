use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    graphics::materials::Material,
    math::{
        point::{self, Point},
        ray::Ray,
        transformations::Transformation,
        vector::Vector,
    },
};

use super::intersections::{Intersection, Intersections};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Sphere {
    pub uid: u128,
    pub transformation: Transformation,
    pub material: Material,
}

impl Sphere {
    /// Creates a new `Sphere`.
    pub fn default() -> Self {
        Self {
            uid: time_now(),
            transformation: Transformation::identity(),
            material: Material::default(),
        }
    }

    /// Creates a new `Sphere` with `Transformation`.
    pub fn new(transformation: Transformation) -> Self {
        Self {
            uid: time_now(),
            transformation,
            material: Material::default(),
        }
    }

    /// Intersections with a Ray
    pub fn intersects(&self, ray: &Ray) -> Option<Intersections> {
        let transformation = match self.transformation.inverse() {
            None => return None,
            Some(t) => t,
        };
        let ray = ray.transform(transformation);

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

            let i1 = Intersection::new(t1, *object);
            let i2 = Intersection::new(t2, *object);

            Some(i1.agregate(&i2))
        }
    }

    /// Returns normal (perpendicular to surface) at `point`
    pub fn normal_at(&self, world_point: Point) -> Option<Vector> {
        // converting from world space to object space
        let object_point = self.transformation.inverse()? * world_point;
        let object_normal = object_point - point::ORIGIN;

        let world_normal = (self.transformation.inverse()?.transpose()) * object_normal;
        Some(world_normal.normalize())
    }
}

fn time_now() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time Moved /backwards/! ")
        .as_nanos()
}
