use crate::{
    constants,
    math::{point::Point, ray::Ray, vector::Vector},
};

use super::shape::Shape;

#[derive(Debug, PartialEq, Clone)]
/// Returns list of intersections, and the id of object that the ray intersected with
pub struct Intersection<'a> {
    pub intersects_at: f64,
    pub object: Shape<'a>,
}

#[derive(Debug, PartialEq, Clone)]
/// Returns list of intersections, and the id of object that the ray intersected with
pub struct Intersections<'a> {
    pub list: Vec<Intersection<'a>>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct PreComputed<'a> {
    pub intersects_at: f64,
    pub object: Shape<'a>,
    pub inside: bool,
    pub point: Point,
    pub eyev: Vector,
    pub normalv: Vector,
    /// Reflected Ray Direction (assuming there is reflection!)
    pub reflectv: Vector,
    /// Offsets towards the normal, prevents shadow 'acne'
    pub over_point: Point,
    /// Offsets below the normal, refactored ray originate here
    pub under_point: Point,
    /// refractive index of the material being exited (n1)
    pub refractive_exited: f64,
    /// refractive index of the material being entered (n2)
    pub refractive_entered: f64,
}

impl<'a> Intersections<'a> {
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
    /// `None` if there is no Shape (index out of bounds)
    pub fn get_intersection(&self, index: usize) -> Option<f64> {
        self.get(index)
            .map(|intersection| intersection.intersects_at)
    }

    /// Returns `Some(Shape)` at `index`
    /// `None` if there is no Shape (index out of bounds)
    pub fn get_object(&self, index: usize) -> Option<Shape> {
        self.get(index).map(|intersection| intersection.object)
    }
    pub fn hit(&self) -> Option<&Intersection> {
        self.list.iter().find(|&x| x.intersects_at >= 0.0)
    }

    /// Used to Chain `.agregate` calls
    pub fn agregate(self, rhs: Intersection<'a>) -> Intersections {
        let mut list = self.list;
        // TODO: sort on hit?
        list.push(rhs);
        list// keep it sorted
            .sort_unstable_by(|a, b| a.intersects_at.partial_cmp(&b.intersects_at).unwrap());
        Intersections { list }
    }
}

impl<'a> Intersection<'a> {
    pub fn new(intersects_at: f64, object: Shape<'a>) -> Self {
        Intersection {
            intersects_at,
            object,
        }
    }

    /// Returns `Intersections`
    pub fn agregate(self, rhs: Self) -> Intersections<'a> {
        Intersections {
            list: vec![self, rhs],
        }
    }

    /// Precomputes the point in world space where the intersection occurred.
    /// `xs` is the list of all intersections, used for computing refractive index
    /// for transperant objects. If there are no intersections just plug in `None`.
    pub fn prepare_computations(
        &self,
        ray: Ray,
        xs: Option<&Intersections>,
    ) -> Option<PreComputed> {
        let intersects_at = self.intersects_at;
        let point = ray.position(intersects_at);
        let object = self.object;
        let eyev = -ray.direction;
        let mut normalv = self.object.normal_at(point)?;
        let inside: bool;

        // TODO: remove the unwrap
        let (refractive_exited, refractive_entered) = refractive_index(self, xs.unwrap());

        if normalv.dot_product(&eyev) < 0.0 {
            inside = true;
            normalv = -normalv
        } else {
            inside = false
        }
        let reflectv = ray.direction.reflect(normalv);
        let over_point = point + normalv * constants::EPSILON;
        let under_point = point - (normalv * constants::EPSILON);

        Some(PreComputed {
            intersects_at,
            object,
            inside,
            point,
            eyev,
            normalv,
            reflectv,
            over_point,
            under_point,
            refractive_exited,
            refractive_entered,
        })
    }
}

impl PreComputed<'_> {
    /// Finds Fresnel Effect
    /// returns reflectance
    pub fn schlick(&self) -> f64 {
        // cos of the angle between eye and normal vectors
        let mut cos = self.eyev.dot_product(&self.normalv);
        let n1 = self.refractive_exited;
        let n2 = self.refractive_entered;

        // total internal reflection can only accour if n1 > n2
        if n1 > n2 {
            let n = n1 / n2;
            let sin2_t = n.powi(2) * (1.0 - cos.powi(2));

            if sin2_t > 1.0 {
                return 1.0;
            }
            // when n1 > n2, use cos(theta_t) instead
            let cos_t = (1.0 - sin2_t).sqrt();
            cos = cos_t;
        }
        let r0 = ((n1 - n2) / (n1 + n2)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}

fn refractive_index(hit: &Intersection, xs: &Intersections) -> (f64, f64) {
    let mut refractive_exited: f64 = 1.0;
    let mut refractive_entered: f64 = 1.0;

    // record which objects have been encoutered (for refraction)
    let mut containers: Vec<Shape> = Vec::with_capacity(xs.list.len());

    for i in &xs.list {
        let is_hit = hit.object.uid == i.object.uid
            && (hit.intersects_at - i.intersects_at).abs() < constants::EPSILON;

        if is_hit {
            if let Some(last) = containers.last() {
                refractive_exited = last.material.refractive_index;
            } else {
                refractive_exited = 1.0
            }
        }
        if let Some(index) = containers.iter().position(|x| x.uid == i.object.uid) {
            // intersection is exiting object
            // since it already entered and is present in the container
            // TODO: use swap_remove
            containers.remove(index);
        } else {
            // intersection is entering object
            // push it, next time it is encoutered, this branch won't run
            containers.push(i.object);
        }
        if is_hit {
            if let Some(last) = containers.last() {
                refractive_entered = last.material.refractive_index;
            } else {
                refractive_entered = 1.0;
            }
            break;
        }
    }
    (refractive_exited, refractive_entered)
}
