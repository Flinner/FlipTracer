use std::f64::consts;

use raytracer::{
    math::{
        point::Point,
        ray::Ray,
        transformations::Transformation,
        vector::{self, Vector},
    },
    objects::sphere::Sphere,
};

#[test]
fn intersect_two_points() {
    let origin = Point::new(0.0, 0.0, -5.0);
    let direction = Vector::new(0.0, 0.0, 1.0);

    let ray = Ray::new(origin, direction);

    let s = Sphere::default();
    let xs = s.intersects(ray).unwrap();
    assert_eq!(xs.count(), 2);
    assert_eq!(xs.get_intersection(0).unwrap(), 4.0);
    assert_eq!(xs.get_intersection(1).unwrap(), 6.0);
    assert_eq!(xs.get_intersection(3), None);
}

#[test]
fn intersecets_at_tanget() {
    let origin = Point::new(0.0, 1.0, -5.0);
    let direction = Vector::new(0.0, 0.0, 1.0);

    let ray = Ray::new(origin, direction);

    let s = Sphere::default();
    let xs = s.intersects(ray).unwrap();
    assert_eq!(xs.count(), 2);
    assert_eq!(xs.get_intersection(0).unwrap(), 5.0);
    assert_eq!(xs.get_intersection(1).unwrap(), 5.0);
    assert_eq!(xs.get_intersection(3), None);
}

#[test]
#[should_panic]
fn ray_misses() {
    let origin = Point::new(0.0, 2.0, -5.0);
    let direction = Vector::new(0.0, 0.0, 1.0);

    let ray = Ray::new(origin, direction);

    let s = Sphere::default();
    let _xs = s.intersects(ray).unwrap();
}

#[test]
fn ray_originates_in_sphere() {
    let origin = Point::new(0.0, 0.0, 0.0);
    let direction = Vector::new(0.0, 0.0, 1.0);

    let ray = Ray::new(origin, direction);

    let s = Sphere::default();
    let xs = s.intersects(ray).unwrap();
    assert_eq!(xs.count(), 2);
    assert_eq!(xs.get_intersection(0).unwrap(), -1.0);
    assert_eq!(xs.get_intersection(1).unwrap(), 1.0);
}

#[test]
fn sphere_behind_ray() {
    let origin = Point::new(0.0, 0.0, 5.0);
    let direction = Vector::new(0.0, 0.0, 1.0);

    let ray = Ray::new(origin, direction);

    let s = Sphere::default();
    let xs = s.intersects(ray).unwrap();
    assert_eq!(xs.count(), 2);
    assert_eq!(xs.get_intersection(0).unwrap(), -6.0);
    assert_eq!(xs.get_intersection(1).unwrap(), -4.0);
}

#[test]
fn sphere_default_transformation() {
    let transformation = Transformation::identity();

    let s = Sphere::default();
    assert_eq!(s.transformation, transformation)
}

#[test]
fn intersecting_scaled_sphere() {
    let origin = Point::new(0.0, 0.0, -5.0);
    let direction = Vector::new(0.0, 0.0, 1.0);

    let ray = Ray::new(origin, direction);

    let transformation = Transformation::scaling(2.0, 2.0, 2.0);
    let s = Sphere::new(transformation);

    let xs = s.intersects(ray).unwrap();

    assert_eq!(xs.count(), 2);
    println!("{:#?}", xs);
    assert_eq!(xs.get_intersection(0).unwrap(), 3.0);
    assert_eq!(xs.get_intersection(1).unwrap(), 7.0);
}

#[test]
#[should_panic]
fn intersecting_translated_sphere() {
    let origin = Point::new(0.0, 0.0, -5.0);
    let direction = Vector::new(0.0, 0.0, 1.0);

    let ray = Ray::new(origin, direction);

    let transformation = Transformation::scaling(5.0, 0.0, 0.0);
    let s = Sphere::new(transformation);

    let _xs = s.intersects(ray).unwrap();
}

#[test]
fn normal_of_sphere() {
    let sqrt3_by3 = 3.0_f64.sqrt() / 3.0;
    let tests = vec![
        (
            Sphere::default().normal_at(Point::new(1.0, 0.0, 0.0)),
            vector::UNIT_X,
        ),
        (
            Sphere::default().normal_at(Point::new(0.0, 1.0, 0.0)),
            vector::UNIT_Y,
        ),
        (
            Sphere::default().normal_at(Point::new(0.0, 0.0, 1.0)),
            vector::UNIT_Z,
        ),
        (
            Sphere::default().normal_at(Point::new(sqrt3_by3, sqrt3_by3, sqrt3_by3)),
            Vector::new(sqrt3_by3, sqrt3_by3, sqrt3_by3),
        ),
    ];

    for test in tests {
        assert_eq!(test.0.unwrap(), test.1);

        // normals are normalized
        assert_eq!(test.1, test.1.normalize())
    }
}

#[test]
fn normal_of_translated_sphere() {
    let sqrt2_by2 = 2.0_f64.sqrt() / 2.0;
    let translation = Transformation::translation(0.0, 1.0, 0.0);
    let scaled_rotated =
        Transformation::scaling(1.0, 0.5, 1.0) * Transformation::rotate_z(consts::PI / 5.0);

    let tests = vec![
        (
            Sphere::new(translation).normal_at(Point::new(
                0.0,
                1.70711,
                -consts::FRAC_1_SQRT_2, // -0.7011
            )),
            Vector::new(0.0, consts::FRAC_1_SQRT_2, -consts::FRAC_1_SQRT_2),
        ),
        (
            Sphere::new(scaled_rotated).normal_at(Point::new(0.0, sqrt2_by2, -sqrt2_by2)),
            Vector::new(0.0, 0.97014, -0.24254),
        ),
    ];

    for test in tests {
        let test0 = test.0.unwrap();
        let test1 = test.1;
        assert_nearly_eq(test0.x, test1.x);
        assert_nearly_eq(test0.y, test1.y);
        assert_nearly_eq(test0.z, test1.z);
    }
}

#[test]
fn sphere_default_material() {
    // Sphere::new()
}

fn assert_nearly_eq(a: f64, b: f64) {
    let assertion = (a - b).abs();
    println!("{},{},{}", a, b, assertion);
    assert!(assertion < 0.00001)
}
