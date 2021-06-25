use raytracer::{
    math::{point::Point, ray::Ray, vector::Vector},
    objects::{intersections::Intersection, sphere::Sphere},
};

#[test]
fn intersection_encapsulate_object_id() {
    let origin = Point::new(0.0, 0.0, 5.0);
    let direction = Vector::new(0.0, 0.0, 1.0);

    let ray = Ray::new(origin, direction);

    let s = Sphere::new();
    let xs = s.intersects(ray).unwrap();
    assert_eq!(xs.get_object(0), Some(s));
}

#[test]
fn agregate_intersections() {
    let s = Sphere::new();
    let i1 = Intersection::new(1.0, s);
    let i2 = Intersection::new(1.0, s);

    let xs = i1.agregate(i2);

    assert_eq!(xs.count(), 2);
    assert_eq!(xs.get_intersection(0), Some(1.0))
}
