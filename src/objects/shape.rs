use super::{plane::Plane, sphere::Sphere};
use crate::graphics::{color::Color, materials::Material};
use crate::math::point::Point;
use crate::math::ray::Ray;
use crate::math::transformations::Transformation;
use crate::math::vector::Vector;
use crate::objects::intersections::Intersections;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(PartialEq, Copy, Debug, Clone)]
pub enum Shape {
    Sphere(Sphere),
    Plane(Plane),
}

impl Shape {
    pub fn intersects(&self, ray: &Ray) -> Option<Intersections> {
        let transformation = self.transformation().inverse()?;
        let ray = &ray.transform(transformation);
        match self {
            Shape::Sphere(s) => s.local_intersects(ray),
            Shape::Plane(s) => s.local_intersects(ray),
        }
    }

    pub fn normal_at(&self, world_point: Point) -> Option<Vector> {
        // converting to object space
        let object_point = self.transformation().inverse()? * world_point;

        let object_normal: Vector = match self {
            Shape::Sphere(s) => s.object_normal_at(object_point)?,
            Shape::Plane(s) => s.object_normal_at(object_point)?,
        };

        // converting to back to world space
        let world_normal = (self.transformation().inverse()?.transpose()) * object_normal;
        Some(world_normal.normalize())
    }

    pub fn pattern_at(&self, world_point: Point) -> Option<Color> {
        let object_point = self.transformation().inverse()? * world_point;
        let pattern_space = self.material().pattern?.transformation.inverse()? * object_point;

        Some(self.material().pattern?.at(pattern_space))
    }

    pub fn transformation(&self) -> Transformation {
        match self {
            Shape::Sphere(s) => s.transformation,
            Shape::Plane(s) => s.transformation,
        }
    }

    pub fn material(&self) -> Material {
        match self {
            Shape::Sphere(s) => s.material,
            Shape::Plane(s) => s.material,
        }
    }

    pub fn uid(&self) -> u128 {
        match self {
            Shape::Sphere(s) => s.uid,
            Shape::Plane(s) => s.uid,
        }
    }

    pub fn new_shape_id() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time Moved /backwards/! ")
            .as_nanos()
    }
}

pub(in crate::objects) trait ShapeInterface {
    /// As you would expect, returns a new object
    fn new(transformation: Transformation) -> Self;

    /// all points are relative to the object i.e, object space
    /// Should NEVER be called directly!
    /// call `Shape.intersects()`
    fn local_intersects(&self, ray: &Ray) -> Option<Intersections>;

    /// Returns normal (perpendicular to surface) at `point`
    /// all points are relative to the object i.e, object space
    /// Should NEVER be called directly!
    /// call `Shape.normal_at()`
    fn object_normal_at(&self, object_point: Point) -> Option<Vector>;
}
