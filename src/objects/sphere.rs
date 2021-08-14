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
    shape::{Shape, ShapeType},
};

/// Check `Shapeinterface.intersects()`
pub(super) fn local_intersects(sphere: &Shape, ray: &Ray) -> Option<Intersections> {
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
        let object = *sphere;

        let i1 = Intersection::new(t1, object);
        let i2 = Intersection::new(t2, object);

        Some(i1.agregate(&i2))
    }
}

/// Check `Shapeinterface.normal_at()`
pub(super) fn object_normal_at(_sphere: &Shape, object_point: Point) -> Option<Vector> {
    let object_normal = object_point - point::ORIGIN;
    Some(object_normal)
}

/// Returns a `Shape` with `shape_type` `Sphere`
/// Equivelent to `Shape::new(transformation, material, ShapeType::Sphere)`
pub fn new(transformation: Transformation, material: Material) -> Shape {
    Shape::new(transformation, material, ShapeType::Sphere)
}

/// Returns a `Shape` with
/// `shape_type` `ShapeType::Sphere`
/// `Material`: `Material::default()`
/// `Transformation`: `Transformation::default()`
pub fn default() -> Shape {
    Shape {
        shape_type: ShapeType::Sphere,
        ..Default::default()
    }
}
