use crate::graphics::{color::Color, materials::Material};
use crate::math::point::Point;
use crate::math::ray::Ray;
use crate::math::transformations::Transformation;
use crate::math::vector::Vector;
use crate::objects::intersections::Intersections;
use std::time::{SystemTime, UNIX_EPOCH};

use super::groups::Group;
/// expose the shapes
pub use super::{cone, cube, cylinder, plane, sphere};

#[derive(PartialEq, Copy, Debug, Clone)]
pub struct Shape<'a> {
    /// Generated from current time, unique
    pub uid: u128,
    /// `Transformation::identity()` is "no transformation", using `Option` could be done in the future
    pub transformation: Transformation, // TODO: maybe using Option for Transformation could lead to better performance?
    /// `Material`, used for finding `Color`
    pub material: Material,
    /// Holds the Shape Type, the only difference between different Shape Types
    /// See `ShapeType`
    pub shape_type: ShapeType,
    /// Allows `Shape`s to be `Group`ed together
    pub parent: Option<&'a Group<'a>>,
}

#[derive(PartialEq, Copy, Debug, Clone)]
/// Holds the Shape Type
/// This is used to match against different functions for finding
/// 1. Normal
/// 2. Intersections
pub enum ShapeType {
    Sphere,
    Plane,
    Cube,
    /// `min` and `max` are the height to truncated at (y axis)
    /// `close`: whether the cylinder should have a cap or not
    Cylinder {
        min: f64,
        max: f64,
        closed: bool,
    },
    /// `min` and `max` are the height to truncated at (y axis)
    /// `close`: whether the Cone should have a cap or not
    Cone {
        min: f64,
        max: f64,
        closed: bool,
    },
}

/// All Functions here change the `Point`s and `Vector`s from *world-space* to *object-space*
/// This makes calculations easier
impl<'a> Shape<'a> {
    /// Returns `Some(Intersections)` if there are intersections and `None` if there are none.
    /// `None` can also be used when finding `Intersections` is impossible
    /// such as not being able to convert from *world-space* to *object-space*
    pub fn intersects(&'a self, ray: &Ray) -> Option<Intersections<'a>> {
        let transformation = self.transformation.inverse()?;
        let ray = ray.transform(transformation);

        use ShapeType::*;
        match self.shape_type {
            Sphere => sphere::local_intersects(self, ray),
            Plane => plane::local_intersects(self, ray),
            Cube => cube::local_intersects(self, ray),
            Cylinder { min, max, closed } => {
                cylinder::local_intersects(self, ray, min, max, closed)
            }
            Cone { min, max, closed } => cone::local_intersects(self, ray, min, max, closed),
        }
    }

    /// Finds the Normal at point, point must be in *world-coordinates*
    /// Returns `Some(Vector)` if there is a normal
    /// `None` is used when finding Normal is impossible
    /// such as not being able to convert from *world-space* to *object-space*
    pub fn normal_at(&self, world_point: Point) -> Option<Vector> {
        // converting to object space
        let object_point = self.transformation.inverse()? * world_point;

        use ShapeType::*;
        let object_normal: Vector = match self.shape_type {
            Sphere => sphere::object_normal_at(self, object_point)?,
            Plane => plane::object_normal_at(self, object_point)?,
            Cube => cube::object_normal_at(self, object_point)?,
            Cylinder {
                min,
                max,
                closed: _,
            } => cylinder::object_normal_at(self, object_point, min, max)?,
            Cone {
                min,
                max,
                closed: _,
            } => cone::object_normal_at(self, object_point, min, max)?,
        };

        // converting back to world space
        let world_normal = (self.transformation.inverse()?.transpose()) * object_normal;
        Some(world_normal.normalize())
    }

    /// Finds the `Color` at point caused by patterns, point must be in *world-coordinates*
    /// Returns `Some(Color)` if there is a color pattern at that `world_point`
    /// `None` is used when finding `Color` is impossible
    /// such as not being able to convert from *world-space* to *object-space*
    /// or no `Pattern` in `Material`
    pub fn pattern_at(&self, world_point: Point) -> Option<Color> {
        let object_point = self.transformation.inverse()? * world_point;
        let pattern_space = self.material.pattern?.transformation.inverse()? * object_point;

        Some(self.material.pattern?.at(pattern_space))
    }
}

impl<'a> Shape<'a> {
    /// As you would expect, returns a new `Shape`
    /// Everthing is set manually, except for the `uid`
    ///      `uid: new_shape_id()`
    pub fn new(
        transformation: Transformation,
        material: Material,
        shape_type: ShapeType,
        parent: Option<&'a Group>,
    ) -> Self {
        Self {
            uid: new_shape_id(),
            transformation,
            material,
            shape_type,
            parent,
        }
    }
}

impl Default for Shape<'_> {
    /// Default `Shape` has
    /// ```rs
    /// transformation: Transformation::identity(),
    /// material: Material::default(),
    /// shape_type: ShapeType::Sphere,
    /// ```
    fn default() -> Self {
        Self {
            uid: new_shape_id(),
            transformation: Transformation::identity(),
            material: Material::default(),
            shape_type: ShapeType::Sphere,
            parent: None,
        }
    }
}

fn new_shape_id() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time Moved /backwards/! ")
        .as_nanos()
}
