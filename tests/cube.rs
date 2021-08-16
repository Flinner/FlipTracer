use raytracer::{
    math::{point::Point, ray::Ray, vector::Vector},
    objects::shape::{self, Shape},
};

#[test]
fn intersects_ray() {
    let test_cases = [
        (
            Point::new(5.0, 0.5, 0.0),
            Vector::new(-1.0, 0.0, 0.0),
            4.0,
            6.0,
        ),
        (
            Point::new(-5.0, 0.5, 0.0),
            Vector::new(1.0, 0.0, 0.0),
            4.0,
            6.0,
        ),
        (
            Point::new(0.5, 5.0, 0.0),
            Vector::new(0.0, -1.0, 0.0),
            4.0,
            6.0,
        ),
        (
            Point::new(0.5, -5.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
            4.0,
            6.0,
        ),
        (
            Point::new(0.5, 0.0, 5.0),
            Vector::new(0.0, 0.0, -1.0),
            4.0,
            6.0,
        ),
        (
            Point::new(0.5, 0.0, -5.0),
            Vector::new(0.0, 0.0, 1.0),
            4.0,
            6.0,
        ),
        (
            Point::new(0.0, 0.5, 0.0),
            Vector::new(0.0, 0.0, 1.0),
            -1.0,
            1.0,
        ),
    ];
    for (origin, direction, i1, i2) in test_cases {
        let ray = Ray::new(origin, direction);

        let s: Shape = shape::cube::default();
        let xs = s.intersects(&ray).unwrap();
        assert_eq!(xs.count(), 2);
        assert_eq!(xs.get_intersection(0).unwrap(), i1);
        assert_eq!(xs.get_intersection(1).unwrap(), i2);
        assert_eq!(xs.get_intersection(3), None);
    }
}

#[test]
fn ray_misses() {
    let test_cases = [
        (
            Point::new(-2.0, 0.0, 0.0),
            Vector::new(0.2673, 0.5345, 0.8018),
        ),
        (
            Point::new(0.0, -2.0, 0.0),
            Vector::new(0.8018, 0.2673, 0.5345),
        ),
        (
            Point::new(0.0, 0.0, -2.0),
            Vector::new(0.5345, 0.8018, 0.2673),
        ),
        (Point::new(2.0, 0.0, 2.0), Vector::new(0.0, 0.0, -1.0)),
        (Point::new(0.0, 2.0, 2.0), Vector::new(0.0, -1.0, 0.0)),
        (Point::new(2.0, 2.0, 0.0), Vector::new(-1.0, 0.0, 0.0)),
    ];
    for (origin, direction) in test_cases {
        let ray = Ray::new(origin, direction);

        let s: Shape = shape::cube::default();
        let xs = s.intersects(&ray);
        assert_eq!(xs, None);
    }
}

#[test]
fn normal_on_surface_of_a_cube() {
    let test_casts = [
        (Point::new(1.0, 0.5, -0.8), Vector::new(1.0, 0.0, 0.0)),
        (Point::new(-1.0, -0.2, 0.9), Vector::new(-1.0, 0.0, 0.0)),
        (Point::new(-0.4, 1.0, -0.1), Vector::new(0.0, 1.0, 0.0)),
        (Point::new(0.3, -1.0, -0.7), Vector::new(0.0, -1.0, 0.0)),
        (Point::new(-0.6, 0.3, 1.0), Vector::new(0.0, 0.0, 1.0)),
        (Point::new(0.4, 0.4, -1.0), Vector::new(0.0, 0.0, -1.0)),
        (Point::new(1.0, 1.0, 1.0), Vector::new(1.0, 0.0, 0.0)),
        (Point::new(-1.0, -1.0, -1.0), Vector::new(-1.0, 0.0, 0.0)),
    ];

    let c = shape::cube::default();

    for (i, (p, n)) in test_casts.iter().enumerate() {
        eprintln!("running test case: {}", i);
        let normal = c.normal_at(*p).unwrap();
        assert_eq!(&normal, n);
    }
}
