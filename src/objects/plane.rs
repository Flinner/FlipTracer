use crate::{
    constants,
    graphics::materials::Material,
    math::{point::Point, ray::Ray, transformations::Transformation, vector::Vector},
    objects::intersections::Intersection,
};

use super::{
    intersections::Intersections,
    shape::{Shape, ShapeType},
};

/// Four Cases
/// 1. Ray Parallel to plane, no intersections
/// 1. Ray is *coplaner* to plane, infinite intersections on infinitly thin plane.\
/// consider no intersections and ray misses
/// 1. Ray origin is above the plane
/// 1. Ray origin is below the plane
pub(super) fn local_intersects<'a>(shape: &'a Shape, ray: Ray) -> Option<Intersections<'a>> {
    if ray.direction.y.abs() < constants::EPSILON
    // case 1 and 2
    {
        None
    } else {
        // case 3 and 4
        let intersects_at = -ray.origin.y / ray.direction.y;
        let object = *shape;
        let intersection = Intersection {
            intersects_at,
            object,
        };

        Some(Intersections {
            list: vec![intersection],
        })
    }
}

/// planes always have a constant normal
pub(super) fn object_normal_at(_plane: &Shape, _object_point: Point) -> Option<Vector> {
    Some(Vector::new(0.0, 1.0, 0.0))
}

/// Returns a `Shape` with `shape_type` `Plane`
/// Equivelent to `Shape::new(transformation, material, ShapeType::Sphere)`
pub fn new<'a>(transformation: Transformation, material: Material) -> Shape<'a> {
    Shape::new(transformation, material, ShapeType::Plane, None)
}

/// Returns a `Shape` with
/// `shape_type` `ShapeType::Plane`
/// `Material`: `Material::default()`
/// `Transformation`: `Transformation::default()`
pub fn default<'a>() -> Shape<'a> {
    Shape {
        shape_type: ShapeType::Plane,
        ..Default::default()
    }
}
