use raytracer::features::point::Point;
use raytracer::features::vector::Vector;
use std::any::Any;
use std::any::TypeId;

mod points {
    use super::*;
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
}

mod vectors {
    use super::*;

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
        ];
        for vector in vectors {
            assert_eq!(vector.magnitude(), 1.0);
        }
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
}
