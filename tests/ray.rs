use raytracer::math::{point::Point, ray::Ray, vector::Vector};

#[test]
fn create_ray() {
    let origin = Point::new(1.0, 2.0, 3.0);
    let direction = Vector::new(4.0, 5.0, 6.0);

    let ray = Ray::new(origin.clone(), direction.clone());

    assert_eq!(ray.origin, origin);
    assert_eq!(ray.direction, direction);
}
