use std::f64::{INFINITY, NEG_INFINITY};

use crate::{
    constants,
    graphics::materials::Material,
    math::{point::Point, ray::Ray, transformations::Transformation, vector::Vector},
};

use super::{
    intersections::{Intersection, Intersections},
    shape::{Shape, ShapeType},
};

pub(super) fn local_intersects(
    cylinder: &Shape,
    ray: &Ray,
    min: f64,
    max: f64,
    closed: bool,
) -> Option<Intersections> {
    let a = ray.direction.x.powi(2) + ray.direction.z.powi(2);

    let mut xs = Intersections { list: vec![] };
    let cylinder = *cylinder;

    // ray parrallel to y axis
    // aprox zero
    if a.abs() < constants::EPSILON {
        // but could intersect with caps
        if closed {
            intersect_caps(cylinder, min, max, ray, &mut xs)
        };
        return if xs.list.is_empty() { None } else { Some(xs) };
    } else {
        let b: f64 = 2.0 * ray.origin.x * ray.direction.x + //.
	/*         */2.0 * ray.origin.z * ray.direction.z;
        let c: f64 = ray.origin.x.powi(2) + ray.origin.z.powi(2) - 1.0;
        let disc = b.powi(2) - 4.0 * a * c;

        // ray doesn't intersect cylinder
        if disc < 0.0 {
            return None;
        }

        let i1 = (-b - disc.sqrt()) / (2.0 * a);
        let i2 = (-b + disc.sqrt()) / (2.0 * a);

        let y1 = ray.origin.y + i1 * ray.direction.y;
        let y2 = ray.origin.y + i2 * ray.direction.y;

        if min < y1 && y1 < max {
            xs.list.push(Intersection::new(i1, cylinder))
        }
        if min < y2 && y2 < max {
            xs.list.push(Intersection::new(i2, cylinder))
        }
        if closed {
            intersect_caps(cylinder, min, max, ray, &mut xs);
        }
    }
    Some(xs)
}

pub(super) fn object_normal_at(
    _cube: &Shape,
    object_point: Point,
    min: f64,
    max: f64,
) -> Option<Vector> {
    let p = object_point;
    let dist = p.x.powi(2) + p.z.powi(2);
    if dist < 1.0 && p.y >= max - constants::EPSILON {
        Some(Vector::new(0.0, 1.0, 0.0))
    } else if dist < 1.0 && p.y <= min + constants::EPSILON {
        Some(Vector::new(0.0, -1.0, 0.0))
    } else {
        Some(Vector::new(p.x, 0.0, p.z))
    }
}

/// Returns a `Shape` with `shape_type` `cylinder`
/// Equivelent to `Shape::new(transformation, material, ShapeType::Cylinder)`
pub fn new(
    transformation: Transformation,
    material: Material,
    min: f64,
    max: f64,
    closed: bool,
) -> Shape {
    Shape::new(
        transformation,
        material,
        ShapeType::Cylinder { min, max, closed },
    )
}

/// Returns a `Shape` with
/// `shape_type` `ShapeType::Cylinder`
/// `Material`: `Material::default()`
/// `Transformation`: `Transformation::default()`
pub fn default() -> Shape {
    Shape {
        shape_type: ShapeType::Cylinder {
            min: NEG_INFINITY,
            max: INFINITY,
            closed: false,
        },
        ..Default::default()
    }
}

/// Just like default, but gives you access to min, max and closed
/// i.e, truncated cylinder
pub fn semi_default(min: f64, max: f64, closed: bool) -> Shape {
    Shape {
        shape_type: ShapeType::Cylinder { min, max, closed },
        ..Default::default()
    }
}

/// a helper function
/// checks to see if the `intersection` (`intersects_at`) is within the radius of 1
/// (radius of the cylinder) from the y axis
fn check_cap(Ray { origin, direction }: &Ray, intersection: f64) -> bool {
    let x = origin.x + intersection * direction.x;
    let z = origin.z + intersection * direction.z;

    (x.powi(2) + z.powi(2)) <= 1.0
}

/// mutates xs and adds the intersections if any.
fn intersect_caps(object: Shape, min: f64, max: f64, ray: &Ray, xs: &mut Intersections) {
    //    check for an intersection with the LOWER end cap by intersecing
    //    the ray with the plane at  y=cylinder.min
    let intersection = (min - ray.origin.y) / ray.direction.y;

    if check_cap(ray, intersection) {
        xs.list.push(Intersection::new(intersection, object))
    }
    //    check for an intersection with the UPPER end cap by intersecing
    //    the ray with the plane at  y=cylinder.max
    let intersection = (max - ray.origin.y) / ray.direction.y;
    if check_cap(ray, intersection) {
        xs.list.push(Intersection::new(intersection, object))
    }
}
