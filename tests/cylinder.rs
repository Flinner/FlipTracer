use raytracer::{
    math::{point::Point, ray::Ray, vector::Vector},
    objects::shape::{self, Shape},
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
