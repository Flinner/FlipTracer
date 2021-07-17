use raytracer::{
    math::{point::Point, ray::Ray, vector::Vector},
    objects::{intersections::Intersection, sphere::Sphere},
};

#[test]
fn intersection_encapsulate_object() {
    let origin = Point::new(0.0, 0.0, 5.0);
    let direction = Vector::new(0.0, 0.0, 1.0);

    let ray = Ray::new(origin, direction);

    let s = Sphere::default();
    let xs = s.intersects(ray).unwrap();
    assert_eq!(xs.get_object(0), Some(s));
}

#[test]
fn agregate_intersections() {
    let s = Sphere::default();
    let i1 = Intersection::new(1.0, &s);
    let i2 = Intersection::new(2.0, &s);

    let xs = i1.agregate(&i2);

    assert_eq!(xs.count(), 2);
    assert_eq!(xs.get_intersection(0), Some(1.0));
    assert_eq!(xs.get_intersection(1), Some(2.0));
}

#[test]
fn hit_when_all_intersections_positive() {
    let s = Sphere::default();
    let i1 = Intersection::new(1.0, &s);
    let i2 = Intersection::new(2.0, &s);

    let xs = i1.agregate(&i2);

    assert_eq!(xs.hit(), Some(&i1));
}

#[test]
fn hit_when_some_intersections_negative() {
    let s = Sphere::default();
    let i1 = Intersection::new(-1.0, &s);
    let i2 = Intersection::new(1.0, &s);

    let xs = i1.agregate(&i2);

    assert_eq!(xs.hit(), Some(&i2));
}

#[test]
fn hit_when_all_intersections_negative() {
    let s = Sphere::default();
    let i1 = Intersection::new(-1.0, &s);
    let i2 = Intersection::new(-2.0, &s);

    let xs = i1.agregate(&i2);

    assert_eq!(xs.hit(), None);
}

#[test]
fn hit_is_lowest_non_negative() {
    let s = Sphere::default();
    let i1 = Intersection::new(5.0, &s);
    let i2 = Intersection::new(7.0, &s);
    let i3 = Intersection::new(-3.0, &s);
    let i4 = Intersection::new(2.0, &s);

    let xs = i1.agregate(&i2).agregate(&i3).agregate(&i4);
    assert_eq!(xs.hit(), Some(&i4));
}