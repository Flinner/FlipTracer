use crate::{
    graphics::materials::Material,
    math::{point::Point, ray::Ray, transformations::Transformation, vector::Vector},
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

    fn local_intersects(&self, ray: &Ray) -> Option<Intersections> {
        todo!()
    }

    /// planes always have a constant normal
    fn object_normal_at(&self, _object_point: Point) -> Option<Vector> {
        Some(Vector::new(0.0, 1.0, 0.0))
    }
}
