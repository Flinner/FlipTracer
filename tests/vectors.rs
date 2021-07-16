use raytracer::math::point::Point;
use raytracer::math::vector;
use raytracer::math::vector::Vector;
use std::any::Any;
use std::any::TypeId;

#[test]
fn vectors_have_vector_type() {
    let vector = Vector {
        x: 4.3,
        y: -4.2,
        z: 3.1,
    };
    assert_eq!(vector.x, 4.3);
    assert_eq!(vector.y, -4.2);
    assert_eq!(vector.z, 3.1);
    assert_eq!(vector.type_id(), TypeId::of::<Vector>());
    assert_ne!(vector.type_id(), TypeId::of::<Point>());
}

#[test]
fn new_creates_vectors() {
    let vector = Vector::new(4.0, -4.0, 3.0);
    assert_eq!(
        vector,
        Vector {
            x: 4.0,
            y: -4.0,
            z: 3.0
        }
    )
}

#[test]
fn adding_vector_and_vector() {
    let vector1 = Vector::new(3.0, -2.0, 5.0);
    let vector2 = Vector::new(-2.0, 3.0, 1.0);

    let new_vector = vector1.add(&vector2);
    let expected_point = Vector::new(1.0, 1.0, 6.0);

    assert_eq!(new_vector, expected_point);
}

#[test]
fn negate_vectors() {
    let vector = Vector::new(3.0, -2.0, 5.0);
    let expected_vector = Vector::new(-3.0, 2.0, -5.0);
    assert_eq!(-vector, expected_vector)
}

#[test]
fn scalar_multiplication() {
    let vector = Vector::new(3.0, -2.0, 5.0);
    let expected_vector = Vector::new(-6.0, 4.0, -10.0);
    assert_eq!(vector * -2.0, expected_vector)
}

#[test]
fn scalar_division() {
    let vector = Vector::new(3.0, -2.0, 5.0);
    let expected_vector = Vector::new(-1.5, 1.0, -2.5);
    assert_eq!(vector / -2.0, expected_vector)
}

#[test]
fn magnitude_of_unit_vectors() {
    let vectors: Vec<Vector> = vec![
        Vector::new(1.0, 0.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 0.0, 1.0),
        vector::UNIT_X,
        vector::UNIT_Y,
        vector::UNIT_Z,
    ];
    for vector in vectors {
        assert_eq!(vector.magnitude(), 1.0);
    }
}

#[test]
fn test_unit_vectors() {
    assert_eq!(Vector::new(1.0, 0.0, 0.0), vector::UNIT_X);
    assert_eq!(Vector::new(0.0, 1.0, 0.0), vector::UNIT_Y);
    assert_eq!(Vector::new(0.0, 0.0, 1.0), vector::UNIT_Z);
}

#[test]
fn magnitude_of_positive_vectors() {
    let vector = Vector::new(1.0, 2.0, 3.0);
    assert_eq!(vector.magnitude(), f64::sqrt(14.0));
}
#[test]
fn magnitude_of_negative_vectors() {
    let vector = Vector::new(-1.0, -2.0, -3.0);
    assert_eq!(vector.magnitude(), f64::sqrt(14.0));
}

#[test]
fn normalizing_vectors() {
    let sqrt14 = f64::sqrt(14.0);

    // (Vector, NormalizedVector)
    let vectors: Vec<(Vector, Vector)> = vec![
        (Vector::new(4.0, 0.0, 0.0), Vector::new(1.0, 0.0, 0.0)),
        (
            Vector::new(1.0, 2.0, 3.0),
            Vector::new(1.0 / sqrt14, 2.0 / sqrt14, 3.0 / sqrt14),
        ),
    ];
    for vector in vectors {
        assert_eq!(vector.0.normalize(), vector.1);
        assert_eq!(vector.0.normalize().magnitude(), 1.0);
    }
}

#[test]
fn dot_product() {
    let vector1 = Vector::new(1.0, 2.0, 3.0);
    let vector2 = Vector::new(2.0, 3.0, 4.0);

    let dot_product = vector1.dot_product(&vector2);

    assert_eq!(dot_product, 20.0);
}

#[test]
fn cross_product() {
    let vector1 = Vector::new(1.0, 2.0, 3.0);
    let vector2 = Vector::new(2.0, 3.0, 4.0);

    let cross_product = vector1.cross_product(&vector2);
    let expected_vector = Vector::new(-1.0, 2.0, -1.0);
    assert_eq!(cross_product, expected_vector);

    let cross_product_reverse = vector2.cross_product(&vector1);
    let expected_vector_reverse = Vector::new(1.0, -2.0, 1.0);
    assert_eq!(cross_product_reverse, expected_vector_reverse);
}

#[test]
fn reflecting_ray() {
    let sqrt2_by2 = f64::sqrt(2.0) / 2.0;

    let ray1 = Vector::new(1.0, -1.0, 0.0);
    let normal1 = Vector::new(0.0, -1.0, 0.0);
    let reflect1 = Vector::new(1.0, 1.0, 0.0);

    // slanted surface
    let ray2 = Vector::new(0.0, -1.0, 0.0);
    let normal2 = Vector::new(sqrt2_by2, sqrt2_by2, 0.0);
    let reflect2 = Vector::new(1.0, 0.0, 0.0);

    assert_nearly_eq(ray1.reflect(normal1), reflect1);
    assert_nearly_eq(ray2.reflect(normal2), reflect2);
}

fn assert_nearly_eq(a: Vector, b: Vector) {
    let assertion = (a.x - b.x).abs();
    println!("{},{},{}", a.x, b.x, assertion);
    assert!(assertion < 0.00001);

    let assertion = (a.y - b.y).abs();
    println!("{},{},{}", a.y, b.y, assertion);
    assert!(assertion < 0.00001);

    let assertion = (a.z - b.z).abs();
    println!("{},{},{}", a.z, b.z, assertion);
    assert!(assertion < 0.00001);
}
