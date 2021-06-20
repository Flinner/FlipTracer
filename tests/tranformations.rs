use raytracer::math::{
    point::Point,
    transformations::{self, Transformation},
};

#[test]
fn multiplying_point_by_translation_matrix() {
    let transform = Transformation::translation(5.0, -3.0, 2.0);
    let point = Point::new(-3.0, 4.0, 5.0);

    let expected = Point::new(2.0, 1.0, 7.0);

    assert_eq!(transform * point, expected)
}

#[test]
fn multiplying_point_by_inverse_of_translation_matrix() {
    let transform = Transformation::translation(5.0, -3.0, 2.0);
    let inv = transform.inverse();
    let point = Point::new(-3.0, 4.0, 5.0);

    let expected = Point::new(-8.0, 7.0, 3.0);

    assert_eq!(inv.clone().unwrap() * point, expected);
}
