use raytracer::math::{point::Point, transformations::Transformation, vector::Vector};
use std::f64::consts::PI;

mod translation {
    use super::*;
    #[test]
    fn multiply_point() {
        let transform = Transformation::translation(5.0, -3.0, 2.0);
        let point = Point::new(-3.0, 4.0, 5.0);

        let expected = Point::new(2.0, 1.0, 7.0);

        assert_eq!(transform * point, expected)
    }

    #[test]
    fn inverse_multiply_point() {
        let transform = Transformation::translation(5.0, -3.0, 2.0);
        let inv = transform.inverse();
        let point = Point::new(-3.0, 4.0, 5.0);

        let expected = Point::new(-8.0, 7.0, 3.0);

        assert_eq!(inv.clone().unwrap() * point, expected);
    }

    #[test]
    fn multiply_vector() {
        let transform = Transformation::translation(5.0, -3.0, 2.0);
        let vector = Vector::new(-3.0, 4.0, 5.0);

        assert_eq!(transform * vector, vector);
    }
}

mod scaling {
    use super::*;

    #[test]
    fn multiply_point() {
        let transform = Transformation::scaling(2.0, 3.0, 4.0);
        let point = Point::new(-4.0, 6.0, 8.0);

        let expected = Point::new(-8.0, 18.0, 32.0);

        assert_eq!(transform * point, expected)
    }

    #[test]
    fn multiply_vector() {
        let transform = Transformation::scaling(2.0, 3.0, 4.0);
        let vector = Vector::new(-4.0, 6.0, 8.0);

        let expected = Vector::new(-8.0, 18.0, 32.0);
        assert_eq!(transform * vector, expected)
    }

    #[test]
    fn inverse_multiply_vector() {
        let transform = Transformation::scaling(2.0, 3.0, 4.0);
        let inv = transform.inverse();
        let vector = Vector::new(-4.0, 6.0, 8.0);

        let expected = Vector::new(-2.0, 2.0, 2.0);

        assert_eq!(inv.unwrap() * vector, expected);
    }
}

mod rotation {

    use super::*;
    #[test]
    fn point_around_x() {
        let p = Point::new(0.0, 1.0, 0.0);

        let half_quarter = Transformation::rotate_x(PI / 4.0);
        let full_quarter = Transformation::rotate_x(PI / 2.0);

        let e_p1 = Point::new(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0);
        let e_p2 = Point::new(0.0, 0.0, 1.0);

        assert_eq!(half_quarter * p, e_p1);
        assert_eq!(full_quarter * p, e_p2);
    }

    #[test]
    fn inverse_point_around_x() {
        let p = Point::new(0.0, 1.0, 0.0);

        let half_quarter = Transformation::rotate_x(PI / 4.0);

        let e_p1 = Point::new(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);

        assert_eq!(half_quarter.inverse().unwrap() * p, e_p1);
    }
}
