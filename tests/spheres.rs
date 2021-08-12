use std::convert::Into;
use std::f64::consts;

use raytracer::{
    graphics::materials::Material,
    math::{
        point::Point,
        ray::Ray,
        transformations::Transformation,
        vector::{self, Vector},
    },
    objects::{shape::Shape, sphere::Sphere},
    testing::Testing,
};

#[test]
fn intersect_two_points() {
    let origin = Point::new(0.0, 0.0, -5.0);
    let direction = Vector::new(0.0, 0.0, 1.0);

    let ray = Ray::new(origin, direction);

    let s: Shape = Sphere::default().into();
    let xs = s.intersects(&ray).unwrap();
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

    let s: Shape = Sphere::default().into();
    let xs = s.intersects(&ray).unwrap();
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

    let s: Shape = Sphere::default().into();
    let _xs = s.intersects(&ray).unwrap();
}

#[test]
fn ray_originates_in_sphere() {
    let origin = Point::new(0.0, 0.0, 0.0);
    let direction = Vector::new(0.0, 0.0, 1.0);

    let ray = Ray::new(origin, direction);

    let s: Shape = Sphere::default().into();
    let xs = s.intersects(&ray).unwrap();
    assert_eq!(xs.count(), 2);
    assert_eq!(xs.get_intersection(0).unwrap(), -1.0);
    assert_eq!(xs.get_intersection(1).unwrap(), 1.0);
}

#[test]
fn sphere_behind_ray() {
    let origin = Point::new(0.0, 0.0, 5.0);
    let direction = Vector::new(0.0, 0.0, 1.0);

    let ray = Ray::new(origin, direction);

    let s: Shape = Sphere::default().into();
    let xs = s.intersects(&ray).unwrap();
    assert_eq!(xs.count(), 2);
    assert_eq!(xs.get_intersection(0).unwrap(), -6.0);
    assert_eq!(xs.get_intersection(1).unwrap(), -4.0);
}

#[test]
fn sphere_default_transformation() {
    let transformation = Transformation::identity();

    let s: Shape = Sphere::default().into();
    assert_eq!(s.transformation(), transformation)
}

#[test]
fn intersecting_scaled_sphere() {
    let origin = Point::new(0.0, 0.0, -5.0);
    let direction = Vector::new(0.0, 0.0, 1.0);

    let ray = Ray::new(origin, direction);

    let transformation = Transformation::scaling(2.0, 2.0, 2.0);
    let s: Shape = Sphere::new(transformation).into();

    let xs = s.intersects(&ray).unwrap();

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
    let s: Shape = Sphere::new(transformation).into();

    let _xs = s.intersects(&ray).unwrap();
}

#[test]
fn normal_of_sphere() {
    let sqrt3_by3 = 3.0_f64.sqrt() / 3.0;
    let tests = vec![
        (
            Shape::Sphere(Sphere::default()).normal_at(Point::new(1.0, 0.0, 0.0)),
            vector::UNIT_X,
        ),
        (
            Shape::Sphere(Sphere::default()).normal_at(Point::new(0.0, 1.0, 0.0)),
            vector::UNIT_Y,
        ),
        (
            Shape::Sphere(Sphere::default()).normal_at(Point::new(0.0, 0.0, 1.0)),
            vector::UNIT_Z,
        ),
        (
            Shape::Sphere(Sphere::default()).normal_at(Point::new(sqrt3_by3, sqrt3_by3, sqrt3_by3)),
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
            Shape::Sphere(Sphere::new(translation)).normal_at(Point::new(
                0.0,
                1.70711,
                -consts::FRAC_1_SQRT_2, // -0.7011
            )),
            Vector::new(0.0, consts::FRAC_1_SQRT_2, -consts::FRAC_1_SQRT_2),
        ),
        (
            Shape::Sphere(Sphere::new(scaled_rotated))
                .normal_at(Point::new(0.0, sqrt2_by2, -sqrt2_by2)),
            Vector::new(0.0, 0.97014, -0.24254),
        ),
    ];

    for test in tests {
        let test0 = test.0.unwrap();
        let test1 = test.1;
        Testing::assert_nearly_eq(test0, test1);
    }
}

#[test]
fn sphere_default_material() {
    assert_eq!(Sphere::default().material, Material::default())
}

#[test]
fn sphere_given_material() {
    let mut s = Sphere::default();
    let mut m = Material::default();
    m.ambient = 1.0;
    s.material = m;
    assert_eq!(s.material, m)
}
