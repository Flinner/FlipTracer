use raytracer::features::point::Point;
use raytracer::features::vector::Vector;
use std::any::Any;
use std::any::TypeId;

#[test]
fn points_have_point_type() {
    let point = Point {
        x: 4.3,
        y: -4.2,
        z: 3.1,
    };
    assert_eq!(point.x, 4.3);
    assert_eq!(point.y, -4.2);
    assert_eq!(point.z, 3.1);
    assert_eq!(point.type_id(), TypeId::of::<Point>());
    assert_ne!(point.type_id(), TypeId::of::<Vector>());
}

#[test]
fn new_creates_points() {
    let point = Point::new(4.0, -4.0, 3.0);
    assert_eq!(
        point,
        Point {
            x: 4.0,
            y: -4.0,
            z: 3.0
        }
    )
}

#[test]
fn adding_point_and_vector_ie_displacment() {
    let point = Point::new(3.0, -2.0, 5.0);
    let vector = Vector::new(-2.0, 3.0, 1.0);

    let new_point = point.displacment(&vector);
    let expected_point = Point::new(1.0, 1.0, 6.0);

    assert_eq!(new_point, expected_point);
}

#[test]
fn subtracting_point_and_vector_ie_negative_displacment() {
    let point = Point::new(3.0, 2.0, 1.0);
    let vector = Vector::new(5.0, 6.0, 7.0);

    let new_point = point.negative_displacment(&vector);
    let expected_point = Point::new(-2.0, -4.0, -6.0);

    assert_eq!(new_point, expected_point);
}

#[test]
fn subtracting_points_give_vector() {
    let point1 = Point::new(3.0, 2.0, 1.0);
    let point2 = Point::new(5.0, 6.0, 7.0);

    let new_vector = point1.subtract(&point2);
    let expected_vector = Vector::new(-2.0, -4.0, -6.0);
    assert_eq!(new_vector, expected_vector);
}

#[test]
fn negate_points() {
    let point = Point::new(3.0, -2.0, 5.0);
    let expected_point = Point::new(-3.0, 2.0, -5.0);
    assert_eq!(-point, expected_point)
}
