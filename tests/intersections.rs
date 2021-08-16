use raytracer::{
    constants,
    math::{point::Point, ray::Ray, transformations::Transformation, vector::Vector},
    objects::{
        intersections::{Intersection, Intersections},
        shape::{self, Shape},
        sphere,
    },
    testing::Testing,
};
use std::f64::consts::SQRT_2;

#[test]
fn intersection_encapsulate_object() {
    let origin = Point::new(0.0, 0.0, 5.0);
    let direction = Vector::new(0.0, 0.0, 1.0);

    let ray = Ray::new(origin, direction);

    let s: Shape = shape::sphere::default();
    let xs = s.intersects(&ray).unwrap();
    assert_eq!(xs.get_object(0), Some(s));
}

#[test]
fn agregate_intersections() {
    let s: Shape = shape::sphere::default();
    let i1 = Intersection::new(1.0, s);
    let i2 = Intersection::new(2.0, s);

    let xs = i1.agregate(i2);

    assert_eq!(xs.count(), 2);
    assert_eq!(xs.get_intersection(0), Some(1.0));
    assert_eq!(xs.get_intersection(1), Some(2.0));
}

#[test]
fn hit_when_all_intersections_positive() {
    let s: Shape = shape::sphere::default();
    let i1 = Intersection::new(1.0, s);
    let i2 = Intersection::new(2.0, s);

    let xs = i1.clone().agregate(i2);

    assert_eq!(xs.hit(), Some(&i1));
}

#[test]
fn hit_when_some_intersections_negative() {
    let s: Shape = shape::sphere::default();
    let i1 = Intersection::new(-1.0, s);
    let i2 = Intersection::new(1.0, s);

    let xs = i1.agregate(i2.clone());

    assert_eq!(xs.hit(), Some(&i2));
}

#[test]
fn hit_when_all_intersections_negative() {
    let s: Shape = shape::sphere::default();
    let i1 = Intersection::new(-1.0, s);
    let i2 = Intersection::new(-2.0, s);

    let xs = i1.agregate(i2);

    assert_eq!(xs.hit(), None);
}

#[test]
fn hit_is_lowest_non_negative() {
    let s: Shape = shape::sphere::default();
    let i1 = Intersection::new(5.0, s);
    let i2 = Intersection::new(7.0, s);
    let i3 = Intersection::new(-3.0, s);
    let i4 = Intersection::new(2.0, s);

    let xs = i1.agregate(i2).agregate(i3).agregate(i4.clone());
    assert_eq!(xs.hit(), Some(&i4));
}

#[test]
fn hit_when_intersection_is_outside() {
    let origin = Point::new(0.0, 0.0, -5.0);
    let direction = Vector::new(0.0, 0.0, 1.0);

    let ray = Ray::new(origin, direction);
    let s: Shape = shape::sphere::default();
    let i = Intersection::new(4.0, s);

    let xs = Intersections {
        list: vec![i.to_owned()],
    };

    let comps = i.prepare_computations(ray, Some(&xs)).unwrap();
    assert!(!comps.inside);
}

#[test]
fn hit_when_intersection_is_inside() {
    let origin = Point::new(0.0, 0.0, 0.0);
    let direction = Vector::new(0.0, 0.0, 1.0);

    let ray = Ray::new(origin, direction);
    let s: Shape = shape::sphere::default();
    let i = Intersection::new(1.0, s);

    let xs = Intersections {
        list: vec![i.to_owned()],
    };

    let comps = i.prepare_computations(ray, Some(&xs)).unwrap();
    assert!(comps.inside);
    assert_eq!(comps.point, Point::new(0.0, 0.0, 1.0));
    assert_eq!(comps.eyev, Vector::new(0.0, 0.0, -1.0));
    assert_eq!(comps.normalv, Vector::new(0.0, 0.0, -1.0)); // inverted
}
#[test]
fn hit_should_offset_the_point() {
    let origin = Point::new(0.0, 0.0, -5.0);
    let direction = Vector::new(0.0, 0.0, 1.0);

    let ray = Ray::new(origin, direction);
    let mut s: Shape = shape::sphere::default();
    s.transformation = Transformation::translation(0.0, 0.0, 1.0);
    let i = Intersection::new(5.0, s);

    let xs = Intersections {
        list: vec![i.to_owned()],
    };

    let comps = i.prepare_computations(ray, Some(&xs)).unwrap();
    println!("comps.over_point.z {}", comps.over_point.z);
    assert!(comps.over_point.z < -constants::EPSILON / 2.0);
    assert!(comps.point.z > comps.over_point.z)
}

#[test]
fn finding_refractive_indices_of_inner_and_outer_surface() {
    // n1 is the material being exited
    // n2 is the material being entered
    // A is the bigger circle
    // B is a smaller circle inside A
    // C is a smaller circle inside A
    // B and C intersect by overlapping each other slightly (like a ven diagram)

    let mut a = sphere::glass();
    a.transformation = Transformation::scaling(2.0, 2.0, 2.0);
    a.material.refractive_index = 1.5;
    let mut b = sphere::glass();
    b.transformation = Transformation::translation(0.0, 0.0, -0.25);
    b.material.refractive_index = 2.0;
    let mut c = sphere::glass();
    c.transformation = Transformation::translation(0.0, 0.0, 0.25);
    c.material.refractive_index = 2.5;

    let origin = Point::new(0.0, 0.0, -4.0);
    let direction = Vector::new(0.0, 0.0, 1.0);

    let ray = Ray::new(origin, direction);

    let i1 = Intersection::new(2.0, a);
    let i2 = Intersection::new(2.75, b);
    let i3 = Intersection::new(3.25, c);
    let i4 = Intersection::new(4.75, b);
    let i5 = Intersection::new(5.25, c);
    let i6 = Intersection::new(6.0, a);
    // let xs = i1
    //     .agregate(i2)
    //     .agregate(i3)
    //     .agregate(i4)
    //     .agregate(i5)
    //     .agregate(i6);

    let xs: Intersections = Intersections {
        list: vec![i1, i2, i3, i4, i5, i6],
    };

    // index, refractive_exited, refractive_entered
    let test_cases: [(usize, f64, f64); 6] = [
        (0, 1.0, 1.5),
        (1, 1.5, 2.0),
        (2, 2.0, 2.5),
        (3, 2.5, 2.5),
        (4, 2.5, 1.5),
        (5, 1.5, 1.0),
    ];
    for (i, refractive_exited, refractive_entered) in test_cases {
        println!("======{}", i);
        let comps = xs
            .get(i)
            .unwrap()
            .prepare_computations(ray, Some(&xs))
            .unwrap();
        assert_eq!(comps.refractive_exited, refractive_exited);
        assert_eq!(comps.refractive_entered, refractive_entered);
    }
}

#[test]
fn under_point_is_offset_below_surface() {
    let origin = Point::new(0.0, 0.0, -5.0);
    let direction = Vector::new(0.0, 0.0, 1.0);

    let ray = Ray::new(origin, direction);
    let mut s: Shape = shape::sphere::glass();
    s.transformation = Transformation::translation(0.0, 0.0, 1.0);
    let i = Intersection::new(5.0, s);

    let xs = Intersections {
        list: vec![i.to_owned()],
    };

    let comps = i.prepare_computations(ray, Some(&xs)).unwrap();
    println!("comps.over_point.z {}", comps.under_point.z);
    assert!(comps.under_point.z > constants::EPSILON / 2.0);
    assert!(comps.point.z < comps.under_point.z)
}

#[test]
fn schilck_approx_under_total_internal_reflection() {
    let shape = sphere::glass();
    let ray = Ray::new(
        Point::new(0.0, 0.0, SQRT_2 / 2.0),
        Vector::new(0.0, 1.0, 0.0),
    );
    let i1 = Intersection::new(-SQRT_2 / 2.0, shape);
    let i2 = Intersection::new(SQRT_2 / 2.0, shape);
    let xs = i1.agregate(i2);

    let comps = xs
        .get(1)
        .unwrap()
        .prepare_computations(ray, Some(&xs))
        .unwrap();
    let reflectance = comps.schlick();

    assert_eq!(reflectance, 1.0)
}

#[test]
fn schilck_approx_with_perpenducual_viewing_angle() {
    let shape = sphere::glass();
    let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0));
    let i1 = Intersection::new(-1.0, shape);
    let i2 = Intersection::new(01.0, shape);
    let xs = i1.agregate(i2);

    let comps = xs
        .get(1)
        .unwrap()
        .prepare_computations(ray, Some(&xs))
        .unwrap();
    let reflectance = comps.schlick();

    Testing::assert_nearly_eq(reflectance, 0.04)
}

#[test]
fn schilck_approx_with_with_small_angle_entered_greater_than_exited() {
    let shape = sphere::glass();
    let ray = Ray::new(Point::new(0.0, 0.99, -2.0), Vector::new(0.0, 0.0, 1.0));
    let i1 = Intersection::new(1.8589, shape);
    // let i2 = Intersection::new(01.0, shape);
    let xs = Intersections { list: vec![i1] };

    let comps = xs
        .get(0)
        .unwrap()
        .prepare_computations(ray, Some(&xs))
        .unwrap();
    let reflectance = comps.schlick();

    Testing::assert_nearly_eq(reflectance, 0.48873)
}
