use crate::{
    graphics::materials::Material,
    math::{point::Point, ray::Ray, transformations::Transformation, vector::Vector},
    objects::intersections::Intersection,
};

use super::{
    intersections::Intersections,
    shape::{Shape, ShapeInterface},
};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Plane {
    pub uid: u128,
    pub transformation: Transformation,
    pub material: Material,
}

impl Default for Plane {
    fn default() -> Self {
        Plane::new(Transformation::identity())
    }
}

impl Plane {
    pub fn new(transformation: Transformation) -> Self {
        ShapeInterface::new(transformation)
    }
}

impl ShapeInterface for Plane {
    fn new(transformation: Transformation) -> Self {
        Plane {
            uid: Shape::new_shape_id(),
            transformation,
            material: Material::default(),
        }
    }

    /// Four Cases
    /// 1. Ray Parallel to plane, no intersections
    /// 1. Ray is *coplaner* to plane, infinite intersections on infinitly thin plane.\
    /// consider no intersections and ray misses
    /// 1. Ray origin is above the plane
    /// 1. Ray origin is below the plane
    fn local_intersects(&self, ray: &Ray) -> Option<Intersections> {
        if ray.direction.y.abs() < 0.00001
        // TODO: use  EPSILON
        // case 1 and 2
        {
            None
        } else {
            // case 3 and 4
            let intersects_at = -ray.origin.y / ray.direction.y;
            let object = (*self).into();
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
    fn object_normal_at(&self, _object_point: Point) -> Option<Vector> {
        Some(Vector::new(0.0, 1.0, 0.0))
    }
}

impl From<Plane> for Shape {
    fn from(s: Plane) -> Self {
        Shape::Plane(s)
    }
}
