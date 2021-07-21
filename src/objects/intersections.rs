use crate::math::{point::Point, ray::Ray, vector::Vector};

use super::sphere::Sphere;

#[derive(Debug, PartialEq, Clone)]
/// Returns list of intersections, and the id of object that the ray intersected with
pub struct Intersection {
    pub intersects_at: f64,
    pub object: Sphere,
}

#[derive(Debug, PartialEq, Clone)]
/// Returns list of intersections, and the id of object that the ray intersected with
pub struct Intersections {
    pub list: Vec<Intersection>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PreComputed {
    pub intersects_at: f64,
    pub object: Sphere,
    pub inside: bool,
    pub point: Point,
    pub eyev: Vector,
    pub normalv: Vector,
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
            let object = &self.list[index].object;
            let intersection = Intersection {
                intersects_at,
                object: object.clone(),
            };
            Some(intersection)
        }
    }
    /// Returns `intersection_at: f64`
    pub fn get_intersection(&self, index: usize) -> Option<f64> {
        match self.get(index) {
            None => None,
            Some(intersection) => Some(intersection.intersects_at),
        }
    }

    /// Returns `intersection_at: f64`
    pub fn get_object(&self, index: usize) -> Option<Sphere> {
        match self.get(index) {
            None => None,
            Some(intersection) => Some(intersection.object),
        }
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
    pub fn new(intersects_at: f64, object: &Sphere) -> Self {
        Intersection {
            intersects_at,
            object: object.clone(),
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
        let point = ray.position(self.intersects_at);
        let intersects_at = self.intersects_at;
        let object = self.object.clone();
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

        Some(PreComputed {
            intersects_at,
            object,
            inside,
            point,
            eyev,
            normalv,
        })
    }
}
