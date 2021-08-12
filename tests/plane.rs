use raytracer::{
    math::{point::Point, vector},
    objects::{plane::Plane, shape::Shape},
};

#[test]
fn normal_of_planes() {
    let tests = vec![
        (
            Shape::Plane(Plane::default()).normal_at(Point::new(1.0, 0.0, 0.0)),
            vector::UNIT_Y,
        ),
        (
            Shape::Plane(Plane::default()).normal_at(Point::new(0.0, 1.0, 0.0)),
            vector::UNIT_Y,
        ),
        (
            Shape::Plane(Plane::default()).normal_at(Point::new(1.0, 0.0, 1.0)),
            vector::UNIT_Y,
        ),
    ];

    for test in tests {
        assert_eq!(test.0.unwrap(), test.1);

        // normals are normalized
        assert_eq!(test.1, test.1.normalize())
    }
}
