use raytracer::{
    math::{point::Point, ray::Ray, vector::Vector},
    objects::sphere::Sphere,
};

#[test]
fn intersect_two_points() {
    let origin = Point::new(0.0, 0.0, -5.0);
    let direction = Vector::new(0.0, 0.0, 1.0);

    let ray = Ray::new(origin, direction);

    let s = Sphere::new();
    let xs = s.intersects(ray).unwrap();
    assert_eq!(xs.count(), 2);
    assert_eq!(xs.get(0).unwrap(), 4.0);
    assert_eq!(xs.get(1).unwrap(), 6.0);
    assert_eq!(xs.get(3), None);
}

#[test]
fn intersecets_at_tanget() {
    let origin = Point::new(0.0, 1.0, -5.0);
    let direction = Vector::new(0.0, 0.0, 1.0);

    let ray = Ray::new(origin, direction);

    let s = Sphere::new();
    let xs = s.intersects(ray).unwrap();
    assert_eq!(xs.count(), 2);
    assert_eq!(xs.get(0).unwrap(), 5.0);
    assert_eq!(xs.get(1).unwrap(), 5.0);
    assert_eq!(xs.get(3), None);
}

#[test]
#[should_panic]
fn ray_misses() {
    let origin = Point::new(0.0, 2.0, -5.0);
    let direction = Vector::new(0.0, 0.0, 1.0);

    let ray = Ray::new(origin, direction);

    let s = Sphere::new();
    let _xs = s.intersects(ray).unwrap();
}

#[test]
fn ray_originates_in_sphere() {
    let origin = Point::new(0.0, 0.0, 0.0);
    let direction = Vector::new(0.0, 0.0, 1.0);

    let ray = Ray::new(origin, direction);

    let s = Sphere::new();
    let xs = s.intersects(ray).unwrap();
    assert_eq!(xs.count(), 2);
    assert_eq!(xs.get(0).unwrap(), -1.0);
    assert_eq!(xs.get(1).unwrap(), 1.0);
}

#[test]
fn sphere_behind_ray() {
    let origin = Point::new(0.0, 0.0, 5.0);
    let direction = Vector::new(0.0, 0.0, 1.0);

    let ray = Ray::new(origin, direction);

    let s = Sphere::new();
    let xs = s.intersects(ray).unwrap();
    assert_eq!(xs.count(), 2);
    assert_eq!(xs.get(0).unwrap(), -6.0);
    assert_eq!(xs.get(1).unwrap(), -4.0);
}

#[test]
fn intersection_encapsulate_object_id() {
    let origin = Point::new(0.0, 0.0, 5.0);
    let direction = Vector::new(0.0, 0.0, 1.0);

    let ray = Ray::new(origin, direction);

    let s = Sphere::new();
    let xs = s.intersects(ray).unwrap();
    assert_eq!(xs.object, s.uid);
}
