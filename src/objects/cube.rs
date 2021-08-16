use crate::{
    constants,
    graphics::materials::Material,
    math::{point::Point, ray::Ray, transformations::Transformation, vector::Vector},
};

use super::{
    intersections::{Intersection, Intersections},
    shape::{Shape, ShapeType},
};

pub(super) fn local_intersects(cube: &Shape, ray: &Ray) -> Option<Intersections> {
    let (xtmin, xtmax) = check_axis(ray.origin.x, ray.direction.x);
    let (ytmin, ytmax) = check_axis(ray.origin.y, ray.direction.y);
    let (ztmin, ztmax) = check_axis(ray.origin.z, ray.direction.z);

    let tmin = xtmin.max(ytmin).max(ztmin);
    let tmax = xtmax.min(ytmax).min(ztmax);

    if tmin > tmax {
        None
    } else {
        let cube = *cube;

        Some(Intersections {
            list: vec![
                Intersection {
                    intersects_at: tmin,
                    object: cube,
                },
                Intersection {
                    intersects_at: tmax,
                    object: cube,
                },
            ],
        })
    }
}

pub(super) fn object_normal_at(_cube: &Shape, object_point: Point) -> Option<Vector> {
    let x = object_point.x;
    let y = object_point.y;
    let z = object_point.z;

    let absx = x.abs();
    let absy = y.abs();
    let absz = z.abs();
    let maxc = absx.max(absy).max(absz);

    Some(if (maxc - absx).abs() < constants::EPSILON {
        Vector::new(x, 0.0, 0.0)
    } else if (maxc - absy).abs() < constants::EPSILON {
        Vector::new(0.0, y, 0.0)
    } else {
        Vector::new(0.0, 0.0, z)
    })
}

fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
    let tmin_numerator = -1.0 - origin;
    let tmax_numerator = 1.0 - origin;

    let tmin = tmin_numerator / direction;
    let tmax = tmax_numerator / direction;

    // if direction.abs() >= EPSILON {
    //     tmin = tmin_numerator / direction;
    //     tmax = tmax_numerator / direction;
    // } else {
    //     tmin = std::f64::NAN;
    //     tmax = std::f64::NAN;
    // }

    // tmin , tmax
    (tmax.min(tmin), tmax.max(tmin))
}

/// Returns a `Shape` with `shape_type` `Cube`
/// Equivelent to `Shape::new(transformation, material, ShapeType::Sphere)`
pub fn new(transformation: Transformation, material: Material) -> Shape {
    Shape::new(transformation, material, ShapeType::Cube)
}

/// Returns a `Shape` with
/// `shape_type` `ShapeType::Cube`
/// `Material`: `Material::default()`
/// `Transformation`: `Transformation::default()`
pub fn default() -> Shape {
    Shape {
        shape_type: ShapeType::Cube,
        ..Default::default()
    }
}
