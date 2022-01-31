use raytracer::math::{point::Point, ray::Ray, transformations::Transformation, vector::Vector};

#[test]
fn create_ray() {
    let origin = Point::new(1.0, 2.0, 3.0);
    let direction = Vector::new(4.0, 5.0, 6.0);

    let ray = Ray::new(origin, direction);

    assert_eq!(ray.origin, origin);
    assert_eq!(ray.direction, direction);
}

#[test]
fn position_from_distance() {
    let origin = Point::new(2.0, 3.0, 4.0);
    let direction = Vector::new(1.0, 0.0, 0.0);

    let ray = Ray::new(origin, direction);

    let tests = vec![
        (ray.position(0.0), Point::new(2.0, 3.0, 4.0)),
        (ray.position(1.0), Point::new(3.0, 3.0, 4.0)),
        (ray.position(-1.0), Point::new(1.0, 3.0, 4.0)),
        (ray.position(2.5), Point::new(4.5, 3.0, 4.0)),
    ];

    for test in tests {
        assert_eq!(test.0, test.1);
    }
}

#[test]
fn translate_a_ray() {
    let origin = Point::new(1.0, 2.0, 3.0);
    let direction = Vector::new(0.0, 1.0, 0.0);

    let r1 = Ray::new(origin, direction);
    let m = Transformation::translation(3.0, 4.0, 5.0);

    let r2 = r1.transform(m);
    assert_eq!(r2.origin, Point::new(4.0, 6.0, 8.0));
    assert_eq!(r2.direction, Vector::new(0.0, 1.0, 0.0));
}

#[test]
fn scale_a_ray() {
    let origin = Point::new(1.0, 2.0, 3.0);
    let direction = Vector::new(0.0, 1.0, 0.0);

    let r1 = Ray::new(origin, direction);
    let m = Transformation::scaling(2.0, 3.0, 4.0);

    let r2 = r1.transform(m);
    assert_eq!(r2.origin, Point::new(2.0, 6.0, 12.0));
    assert_eq!(r2.direction, Vector::new(0.0, 3.0, 0.0));
}
