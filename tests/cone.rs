use raytracer::{
    math::{point::Point, ray::Ray, vector::Vector},
    objects::shape::{self, Shape},
    testing::Testing,
};
#[test]
fn ray_hits() {
    let test_cases = [
        (
            Point::new(0.0, 0.0, -5.0),
            Vector::new(0.0, 0.0, 1.0),
            5.0,
            5.0,
        ),
        (
            Point::new(0.0, 0.0, -5.0),
            Vector::new(1.0, 1.0, 1.0),
            8.66025,
            8.66025,
        ),
        (
            Point::new(1.0, 1.0, -5.0),
            Vector::new(-0.5, -1.0, 1.0),
            4.55006,
            49.44994,
        ),
    ];
    for (origin, direction, i1, i2) in test_cases {
        let ray = Ray::new(origin, direction.normalize());

        let s: Shape = shape::cone::default();
        let xs = s.intersects(&ray).unwrap();
        println!("ahhh {:?}", origin);
        assert_eq!(xs.count(), 2);
        Testing::assert_nearly_eq(xs.get_intersection(0).unwrap(), i1);
        Testing::assert_nearly_eq(xs.get_intersection(1).unwrap(), i2);
        assert_eq!(xs.get_intersection(3), None);
    }
}

#[test]
fn ray_parallel_to_one_of_its_halves() {
    let origin = Point::new(0.0, 0.0, -1.0);
    let direction = Vector::new(0.0, 1.0, 1.0);

    let ray = Ray::new(origin, direction.normalize());

    let s: Shape = shape::cone::default();
    let xs = s.intersects(&ray).unwrap();
    println!("ahhh {:?}", origin);
    assert_eq!(xs.count(), 1);
    Testing::assert_nearly_eq(xs.get_intersection(0).unwrap(), 0.35355);
}

#[test]
fn intersescting_caps_of_closed_cone() {
    let test_cases: [(Point, Vector, usize); 3] = [
        (Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0), 0),
        (Point::new(0.0, 0.0, -0.25), Vector::new(0.0, 1.0, 1.0), 2),
        (Point::new(0.0, 0.0, -0.25), Vector::new(0.0, 1.0, 0.0), 4),
    ];
    for (i, (origin, direction, count)) in test_cases.iter().enumerate() {
        let ray = Ray::new(*origin, direction.normalize());

        let s: Shape = shape::cone::semi_default(-0.5, 0.5, true);
        eprintln!("iter: {}", i);
        let xs = s.intersects(&ray).unwrap();
        assert_eq!(&xs.count(), count);
    }
}
