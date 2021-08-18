use std::f64::{INFINITY, NEG_INFINITY};

use raytracer::{
    math::{point::Point, ray::Ray, vector::Vector},
    objects::shape::{self, Shape, ShapeType},
    testing::Testing,
};

#[test]
fn ray_misses() {
    let test_cases = [
        (Point::new(1.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0)),
        (Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0)),
        (Point::new(0.0, 0.0, -5.0), Vector::new(1.0, 1.0, 1.0)),
    ];
    for (origin, direction) in test_cases {
        let ray = Ray::new(origin, direction);

        let s: Shape = shape::cylinder::default();
        let xs = s.intersects(&ray);
        assert_eq!(xs, None);
    }
}

#[test]
fn ray_hits() {
    let test_cases = [
        (
            Point::new(1.0, 0.0, -5.0),
            Vector::new(0.0, 0.0, 1.0),
            5.0,
            5.0,
        ),
        (
            Point::new(0.0, 0.0, -5.0),
            Vector::new(0.0, 0.0, 1.0),
            4.0,
            6.0,
        ),
        (
            Point::new(0.5, 0.0, -5.0),
            Vector::new(0.1, 1.0, 1.0),
            6.80798,
            7.08872,
        ),
    ];
    for (origin, direction, i1, i2) in test_cases {
        let ray = Ray::new(origin, direction.normalize());

        let s: Shape = shape::cylinder::default();
        let xs = s.intersects(&ray).unwrap();
        assert_eq!(xs.count(), 2);
        Testing::assert_nearly_eq(xs.get_intersection(0).unwrap(), i1);
        Testing::assert_nearly_eq(xs.get_intersection(1).unwrap(), i2);
        assert_eq!(xs.get_intersection(3), None);
    }
}

#[test]
fn normal_vector_on_cylinder() {
    let test_casts = [
        (Point::new(1.0, 0.0, 0.0), Vector::new(1.0, 0.0, 0.0)),
        (Point::new(0.0, 5.0, -1.0), Vector::new(0.0, 0.0, -1.0)),
        (Point::new(0.0, -2.0, 1.0), Vector::new(0.0, 0.0, 1.0)),
        (Point::new(-1.0, 1.0, 0.0), Vector::new(-1.0, 0.0, 0.0)),
    ];

    let c = shape::cylinder::default();

    for (i, (p, n)) in test_casts.iter().enumerate() {
        eprintln!("running test case: {}", i);
        let normal = c.normal_at(*p).unwrap();
        assert_eq!(&normal, n);
    }
}

#[test]
fn minimum_and_maximum_for_a_cylinder() {
    let cyl = shape::cylinder::default();
    if let ShapeType::Cylinder {
        min,
        max,
        closed: _,
    } = cyl.shape_type
    {
        assert_eq!(min, NEG_INFINITY);
        assert_eq!(max, INFINITY);
    } else {
        panic!("Not a Cylinder!");
    }
}

#[test]
fn intersescting_a_constrained_cylinder() {
    let test_cases: [(Point, Vector, usize); 6] = [
        (Point::new(0.0, 1.5, 0.0), Vector::new(0.1, 1.0, 0.0), 0),
        (Point::new(0.0, 3.0, -5.0), Vector::new(0.0, 0.0, 1.0), 0),
        (Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0), 0),
        (Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0), 0),
        (Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0), 0),
        (Point::new(0.0, 1.5, -2.0), Vector::new(0.0, 0.0, 1.0), 2),
    ];
    for (origin, direction, count) in test_cases {
        let ray = Ray::new(origin, direction.normalize());

        let s: Shape = shape::cylinder::semi_default(1.0, 2.0, false);
        let xs = s.intersects(&ray).unwrap();
        assert_eq!(xs.count(), count);
        // Testing::assert_nearly_eq(xs.get_intersection(0).unwrap(), i1);
        // Testing::assert_nearly_eq(xs.get_intersection(1).unwrap(), i2);
    }
}

#[test]
fn default_for_cylinder_is_closed() {
    let cyl = shape::cylinder::default();
    if let ShapeType::Cylinder {
        min: _,
        max: _,
        closed,
    } = cyl.shape_type
    {
        assert!(!closed)
    } else {
        panic!("Not a Cylinder!");
    }
}

#[test]
fn intersescting_caps_of_closed_cylinder() {
    let test_cases: [(Point, Vector, usize); 5] = [
        (Point::new(0.0, 3.0, 0.0), Vector::new(0.0, -1.0, 0.0), 2),
        (Point::new(0.0, 3.0, -2.0), Vector::new(0.0, -1.0, 2.0), 2),
        (Point::new(0.0, 4.0, -2.0), Vector::new(0.0, -1.0, 1.0), 2), // corner
        (Point::new(0.0, 0.0, -2.0), Vector::new(0.0, 1.0, 1.0), 2),
        (Point::new(0.0, -1.0, -2.0), Vector::new(0.0, 1.0, 1.0), 2), // corner
    ];
    for (i, (origin, direction, count)) in test_cases.iter().enumerate() {
        let ray = Ray::new(*origin, direction.normalize());

        let s: Shape = shape::cylinder::semi_default(1.0, 2.0, true);
        eprintln!("iter: {}", i);
        let xs = s.intersects(&ray).unwrap();
        assert_eq!(xs.count(), *count);
        // Testing::assert_nearly_eq(xs.get_intersection(0).unwrap(), i1);
        // Testing::assert_nearly_eq(xs.get_intersection(1).unwrap(), i2);
    }
}

#[test]
fn normal_vector_on_cylinder_end_caps() {
    let test_casts = [
        (Point::new(0.0, 1.0, 0.0), Vector::new(0.0, -1.0, 0.0)),
        (Point::new(0.5, 1.0, 0.0), Vector::new(0.0, -1.0, 0.0)),
        (Point::new(0.0, 1.0, 0.5), Vector::new(0.0, -1.0, 0.0)),
        (Point::new(0.5, 2.0, 0.0), Vector::new(0.0, 1.0, 0.0)),
        (Point::new(0.0, 2.0, 0.0), Vector::new(0.0, 1.0, 0.0)),
        (Point::new(0.0, 2.0, 0.5), Vector::new(0.0, 1.0, 0.0)),
    ];

    let c = shape::cylinder::semi_default(1.0, 2.0, true);

    for (i, (p, n)) in test_casts.iter().enumerate() {
        eprintln!("running test case: {}", i);
        let normal = c.normal_at(*p).unwrap();
        assert_eq!(&normal, n);
    }
}
