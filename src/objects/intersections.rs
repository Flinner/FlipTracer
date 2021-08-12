use crate::{
    graphics::color::Color,
    math::{point::Point, ray::Ray, vector::Vector},
};

use super::{shape::Shape, world::World};

#[derive(Debug, PartialEq, Clone)]
/// Returns list of intersections, and the id of object that the ray intersected with
pub struct Intersection {
    pub intersects_at: f64,
    pub object: Shape,
}

#[derive(Debug, PartialEq, Clone)]
/// Returns list of intersections, and the id of object that the ray intersected with
pub struct Intersections {
    pub list: Vec<Intersection>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct PreComputed {
    pub intersects_at: f64,
    pub object: Shape,
    pub inside: bool,
    pub point: Point,
    pub eyev: Vector,
    pub normalv: Vector,
    /// Offsets towards the normal, prevents shadow 'acne'
    pub over_point: Point,
}

impl Intersections {
    /// count of intersections, eq to `self.list.len()`
    pub fn count(&self) -> usize {
        self.list.len()
    }
    /// Returns Intersection
    pub fn get(&self, index: usize) -> Option<Intersection> {
        if index >= self.list.len() {
            None
        } else {
            let intersects_at = self.list[index].intersects_at;
            let intersection = Intersection {
                intersects_at,
                object: self.list[index].object,
            };
            Some(intersection)
        }
    }
    /// Returns `intersection_at: f64`
    pub fn get_intersection(&self, index: usize) -> Option<f64> {
        self.get(index)
            .map(|intersection| intersection.intersects_at)
    }

    /// Returns `intersection_at: f64`
    pub fn get_object(&self, index: usize) -> Option<Shape> {
        self.get(index).map(|intersection| intersection.object)
    }
    pub fn hit(&self) -> Option<&Intersection> {
        self.list.iter().find(|&x| x.intersects_at >= 0.0)
    }

    /// Used to Chain `.agregate` calls
    pub fn agregate(mut self, rhs: &Intersection) -> Intersections {
        self.list.push(rhs.clone());
        self.list// keep it sorted
            .sort_unstable_by(|a, b| a.intersects_at.partial_cmp(&b.intersects_at).unwrap());
        self
    }
}

impl Intersection {
    pub fn new(intersects_at: f64, object: Shape) -> Self {
        Intersection {
            intersects_at,
            object,
        }
    }

    /// Returns `Intersections`
    pub fn agregate(&self, rhs: &Self) -> Intersections {
        Intersections {
            list: vec![self.clone(), rhs.clone()],
        }
    }

    /// Precomputes the point in world space where the intersection occurred
    pub fn prepare_computations(&self, ray: Ray) -> Option<PreComputed> {
        let intersects_at = self.intersects_at;
        let point = ray.position(intersects_at);
        let object = self.object;
        let eyev = -ray.direction;
        let mut normalv = self.object.normal_at(point)?;
        let inside: bool;

        // when the intersection is inside the object, invert the normal
        if normalv.dot_product(&eyev) < 0.0 {
            inside = true;
            normalv = -normalv
        } else {
            inside = false
        }
        let over_point = point + normalv * 0.0000001; // EPSILON
                                                      // TODO: move EPSILON to Shapes

        Some(PreComputed {
            intersects_at,
            object,
            inside,
            point,
            eyev,
            normalv,
            over_point,
        })
    }
}

impl PreComputed {
    pub fn shade_hit(&self, w: &World) -> Color {
        let shadowed = w.is_shadowed(self.over_point);
        self.object.material().lighting(
            self.object,
            w.light.expect("no light!"),
            self.over_point,
            self.eyev,
            self.normalv,
            shadowed,
        )
    }
}
