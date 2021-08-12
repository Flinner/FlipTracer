use crate::{
    graphics::materials::Material,
    math::{
        point::{self, Point},
        ray::Ray,
        transformations::Transformation,
        vector::Vector,
    },
};

use super::{
    intersections::{Intersection, Intersections},
    shape::Shape,
};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Sphere {
    pub uid: u128,
    pub transformation: Transformation,
    pub material: Material,
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere::new(Transformation::identity())
    }
}

impl Sphere {
    pub fn new(transformation: Transformation) -> Self {
        Self {
            uid: Shape::shape_id(),
            transformation,
            material: Material::default(),
        }
    }

    /// all points are relative to the object i.e, object space
    /// Should NEVER be called directly!
    /// call `Shape.intersects()`
    pub(in crate::objects) fn intersects(&self, ray: &Ray) -> Option<Intersections> {
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
            let object = Shape::Sphere(*self);

            let i1 = Intersection::new(t1, object);
            let i2 = Intersection::new(t2, object);

            Some(i1.agregate(&i2))
        }
    }

    /// Returns normal (perpendicular to surface) at `point`
    /// all points are relative to the object i.e, object space
    /// Should NEVER be called directly!
    /// call `Shape.normal_at()`
    pub(in crate::objects) fn object_normal_at(&self, object_point: Point) -> Option<Vector> {
        let object_normal = object_point - point::ORIGIN;
        Some(object_normal)
    }
}

impl From<Sphere> for Shape {
    fn from(s: Sphere) -> Self {
        Shape::Sphere(s)
    }
}
