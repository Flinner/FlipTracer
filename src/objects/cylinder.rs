use crate::{
    constants,
    graphics::materials::Material,
    math::{point::Point, ray::Ray, transformations::Transformation, vector::Vector},
};

use super::{
    intersections::{Intersection, Intersections},
    shape::{Shape, ShapeType},
};

pub(super) fn local_intersects(cylinder: &Shape, ray: &Ray) -> Option<Intersections> {
    let a = ray.direction.x.powi(2) + ray.direction.z.powi(2);

    // ray parrallel to y axis
    // aprox zero
    if a.abs() < constants::EPSILON {
        return None;
    }

    let b: f64 = 2.0 * ray.origin.x * ray.direction.x + //.
	/*    .*/2.0 * ray.origin.z * ray.direction.z;
    let c: f64 = ray.origin.x.powi(2) + ray.origin.z.powi(2) - 1.0;
    let disc = b.powi(2) - 4.0 * a * c;

    // ray doesn't intersect cylinder
    if disc < 0.0 {
        return None;
    }

    let i1 = (-b - disc.sqrt()) / (2.0 * a);
    let i2 = (-b + disc.sqrt()) / (2.0 * a);

    let cylinder = *cylinder;
    Some(Intersections {
        list: vec![
            Intersection {
                intersects_at: i1,
                object: cylinder,
            },
            Intersection {
                intersects_at: i2,
                object: cylinder,
            },
        ],
    })
}

pub(super) fn object_normal_at(_cube: &Shape, _object_point: Point) -> Option<Vector> {
    todo!()
}
/// Returns a `Shape` with `shape_type` `cylinder`
/// Equivelent to `Shape::new(transformation, material, ShapeType::Cylinder)`
pub fn new(transformation: Transformation, material: Material) -> Shape {
    Shape::new(transformation, material, ShapeType::Cylinder)
}

/// Returns a `Shape` with
/// `shape_type` `ShapeType::Cylinder`
/// `Material`: `Material::default()`
/// `Transformation`: `Transformation::default()`
pub fn default() -> Shape {
    Shape {
        shape_type: ShapeType::Cylinder,
        ..Default::default()
    }
}
