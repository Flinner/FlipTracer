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

pub(super) fn local_intersects<'a>(
    cone: &'a Shape,
    ray: Ray,
    min: f64,
    max: f64,
    closed: bool,
) -> Option<Intersections<'a>> {
    // x^2 - y^2 + z^2
    let a = ray.direction.x.powi(2) - ray.direction.y.powi(2) + ray.direction.z.powi(2);

    let b: f64 = 2.0 * ray.origin.x * ray.direction.x -
	/*.    */2.0 * ray.origin.y * ray.direction.y +
	/*.    */2.0 * ray.origin.z * ray.direction.z;

    let c: f64 = ray.origin.x.powi(2) -
	/*.    */ray.origin.y.powi(2) +
	/*.    */ray.origin.z.powi(2);

    let mut xs = Intersections { list: vec![] };
    let cone = *cone;

    let a_iszero = a.abs() < constants::EPSILON;
    let b_iszero = b.abs() < constants::EPSILON;

    // no intersection
    if a_iszero && b_iszero {
        // return if xs.list.is_empty() { None } else { Some(xs) };
        return None;
    // single intersection
    } else if a_iszero && !b_iszero {
        let i = -c / (2.0 * b);
        xs.list.push(Intersection::new(i, cone))
    } else {
        let disc = b.powi(2) - 4.0 * a * c;

        // ray doesn't intersect cone
        if disc < 0.0 {
            return None;
        }

        let i1 = (-b - disc.sqrt()) / (2.0 * a);
        let i2 = (-b + disc.sqrt()) / (2.0 * a);

        let y1 = ray.origin.y + i1 * ray.direction.y;
        let y2 = ray.origin.y + i2 * ray.direction.y;

        if min < y1 && y1 < max {
            xs.list.push(Intersection::new(i1, cone))
        }
        if min < y2 && y2 < max {
            xs.list.push(Intersection::new(i2, cone))
        }
    }
    if closed {
        intersect_caps(cone, min, max, &ray, &mut xs);
    }

    Some(xs)
}
pub(super) fn object_normal_at(
    _cone: &Shape,
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
        let mut y = (p.x.powi(2) + p.z.powi(2)).sqrt();
        if p.y > 0.0 {
            y = -y
        };
        Some(Vector::new(p.x, y, p.z))
    }
}

/// Returns a `Shape` with `shape_type` `cone`
/// Equivelent to `Shape::new(transformation, material, ShapeType::cone, None)`
pub fn new<'a>(
    transformation: Transformation,
    material: Material,
    min: f64,
    max: f64,
    closed: bool,
) -> Shape<'a> {
    Shape::new(
        transformation,
        material,
        ShapeType::Cone { min, max, closed },
        // no parent
        None,
    )
}

/// Returns a `Shape` with
/// `shape_type` `ShapeType::Cone`
/// `Material`: `Material::default()`
/// `Transformation`: `Transformation::default()`
pub fn default<'a>() -> Shape<'a> {
    Shape {
        shape_type: ShapeType::Cone {
            min: NEG_INFINITY,
            max: INFINITY,
            closed: false,
        },
        ..Default::default()
    }
}

/// Just like default, but gives you access to min, max and closed
/// i.e, truncated Cone
pub fn semi_default<'a>(min: f64, max: f64, closed: bool) -> Shape<'a> {
    Shape {
        shape_type: ShapeType::Cone { min, max, closed },
        ..Default::default()
    }
}

/// a helper function
/// checks to see if the `intersection` (`intersects_at`) is within the radius of 1
/// (radius of the Cone) from the y axis
fn check_cap(Ray { origin, direction }: &Ray, intersection: f64, radius: f64) -> bool {
    let x = origin.x + intersection * direction.x;
    let z = origin.z + intersection * direction.z;

    (x.powi(2) + z.powi(2)) <= radius.abs()
}

/// mutates xs and adds the intersections if any.
fn intersect_caps<'a>(
    object: Shape<'a>,
    min: f64,
    max: f64,
    ray: &Ray,
    xs: &mut Intersections<'a>,
) {
    //    check for an intersection with the LOWER end cap by intersecing
    //    the ray with the plane at  y=Cone.min
    let intersection = (min - ray.origin.y) / ray.direction.y;

    if check_cap(ray, intersection, min) {
        xs.list.push(Intersection::new(intersection, object))
    }
    //    check for an intersection with the UPPER end cap by intersecing
    //    the ray with the plane at  y=Cone.max
    let intersection = (max - ray.origin.y) / ray.direction.y;
    if check_cap(ray, intersection, max) {
        xs.list.push(Intersection::new(intersection, object))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::SQRT_2;

    #[test]
    fn normal_vector_on_cone() {
        let test_casts = [
            (Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 0.0)),
            (Point::new(1.0, 1.0, 1.0), Vector::new(1.0, -SQRT_2, 1.0)),
            (Point::new(-1.0, -1.0, 0.0), Vector::new(-1.0, 1.0, 0.0)),
        ];

        let c = default();

        for (i, (p, n)) in test_casts.iter().enumerate() {
            eprintln!("running test case: {}", i);
            let normal = object_normal_at(&c, *p, NEG_INFINITY, INFINITY).unwrap();
            eprintln!("normal: {:#?}", normal);
            assert_eq!(&normal, n);
        }
    }
}
