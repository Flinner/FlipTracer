use raytracer::math::{point::Point, ray::Ray, vector::Vector};

#[test]
fn create_ray() {
    let origin = Point::new(1.0, 2.0, 3.0);
    let direction = Vector::new(4.0, 5.0, 6.0);

    let ray = Ray::new(origin.clone(), direction.clone());

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
