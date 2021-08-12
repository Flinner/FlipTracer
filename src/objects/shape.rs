use super::sphere::Sphere;
use crate::graphics::materials::Material;
use crate::math::point::Point;
use crate::math::ray::Ray;
use crate::math::transformations::Transformation;
use crate::math::vector::Vector;
use crate::objects::intersections::Intersections;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(PartialEq, Copy, Debug, Clone)]
pub struct Plane {}

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
            Shape::Sphere(s) => s.intersects(ray),
            Shape::Plane(_) => todo!(),
        }
    }
    pub fn normal_at(&self, world_point: Point) -> Option<Vector> {
        // converting to object space
        let object_point = self.transformation().inverse()? * world_point;

        let object_normal: Vector = match self {
            Shape::Sphere(s) => s.object_normal_at(object_point)?,
            Shape::Plane(_) => todo!(),
        };

        // converting to back to world space
        let world_normal = (self.transformation().inverse()?.transpose()) * object_normal;
        Some(world_normal.normalize())
    }
    pub fn transformation(&self) -> Transformation {
        match self {
            Shape::Sphere(s) => s.transformation,
            Shape::Plane(_) => todo!(),
        }
    }

    pub fn material(&self) -> Material {
        match self {
            Shape::Sphere(s) => s.material,
            Shape::Plane(_) => todo!(),
        }
    }

    pub fn uid(&self) -> u128 {
        match self {
            Shape::Sphere(s) => s.uid,
            Shape::Plane(_) => todo!(),
        }
    }

    pub fn shape_id() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time Moved /backwards/! ")
            .as_nanos()
    }
}
