use raytracer::math::{point::Point, transformations::Transformation, vector::Vector};

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

#[test]
fn multiplying_vector_by_translation_matrix() {
    let transform = Transformation::translation(5.0, -3.0, 2.0);
    let vector = Vector::new(-3.0, 4.0, 5.0);

    assert_eq!(transform * vector, vector);
}

#[test]
fn multiplying_point_by_scaling_matrix() {
    let transform = Transformation::scaling(2.0, 3.0, 4.0);
    let point = Point::new(-4.0, 6.0, 8.0);

    let expected = Point::new(-8.0, 18.0, 32.0);

    assert_eq!(transform * point, expected)
}

#[test]
fn multiplying_vector_by_scaling_matrix() {
    let transform = Transformation::scaling(2.0, 3.0, 4.0);
    let vector = Vector::new(-4.0, 6.0, 8.0);

    let expected = Vector::new(-8.0, 18.0, 32.0);
    assert_eq!(transform * vector, expected)
}

#[test]
fn multiplying_vector_by_inverse_of_scaling_matrix() {
    let transform = Transformation::scaling(2.0, 3.0, 4.0);
    let inv = transform.inverse();
    let vector = Vector::new(-4.0, 6.0, 8.0);

    let expected = Vector::new(-2.0, 2.0, 2.0);

    assert_eq!(inv.unwrap() * vector, expected);
}
