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

    #[test]
    fn point_around_y() {
        let p = Point::new(0.0, 0.0, 1.0);

        let half_quarter = Transformation::rotate_y(PI / 4.0);
        let full_quarter = Transformation::rotate_y(PI / 2.0);

        let e_p1 = Point::new(2.0_f64.sqrt() / 2.0, 0.0, 2.0_f64.sqrt() / 2.0);
        let e_p2 = Point::new(1.0, 0.0, 0.0);

        assert_eq!(half_quarter * p, e_p1);
        assert_eq!(full_quarter * p, e_p2);
    }

    #[test]
    fn point_around_z() {
        let p = Point::new(0.0, 1.0, 0.0);

        let half_quarter = Transformation::rotate_z(PI / 4.0);
        let full_quarter = Transformation::rotate_z(PI / 2.0);

        let e_p1 = Point::new(-2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0);
        let e_p2 = Point::new(-1.0, 0.0, 0.0);

        assert_eq!(half_quarter * p, e_p1);
        assert_eq!(full_quarter * p, e_p2);
    }
}

mod shearing {
    use super::*;

    #[test]
    fn x_moves_in_proportion_to_y() {
        let transform = Transformation::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);

        let expected = Point::new(5.0, 3.0, 4.0);

        assert_eq!(transform * p, expected);
    }

    #[test]
    fn x_moves_in_proportion_to_z() {
        let transform = Transformation::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);

        let expected = Point::new(6.0, 3.0, 4.0);

        assert_eq!(transform * p, expected);
    }

    #[test]
    fn y_moves_in_proportion_to_x() {
        let transform = Transformation::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);

        let expected = Point::new(2.0, 5.0, 4.0);

        assert_eq!(transform * p, expected);
    }

    #[test]
    fn y_moves_in_proportion_to_z() {
        let transform = Transformation::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);

        let expected = Point::new(2.0, 7.0, 4.0);

        assert_eq!(transform * p, expected);
    }

    #[test]
    fn z_moves_in_proportion_to_x() {
        let transform = Transformation::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Point::new(2.0, 3.0, 4.0);

        let expected = Point::new(2.0, 3.0, 6.0);

        assert_eq!(transform * p, expected);
    }

    #[test]
    fn z_moves_in_proportion_to_y() {
        let transform = Transformation::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Point::new(2.0, 3.0, 4.0);

        let expected = Point::new(2.0, 3.0, 7.0);

        assert_eq!(transform * p, expected);
    }
}

mod chainining {
    use super::*;

    #[test]
    fn apply_individually() {
        let p = Point::new(1.0, 0.0, 1.0);

        let rotation = Transformation::rotate_x(PI / 2.0);
        let scaling = Transformation::scaling(5.0, 5.0, 5.0);
        let translation = Transformation::translation(10.0, 5.0, 7.0);

        // rotation first
        let p2 = rotation.clone() * p;
        assert_eq!(p2, Point::new(1.0, -1.0, 0.0));

        // then scaling
        // this test has floating problems :(
        let p3 = scaling.clone() * p2;
        let e_p3 = Point::new(5.0, -5.0, 0.0);
        assert!((p3.x - e_p3.x).abs() < 0.0001);
        assert!((p3.y - e_p3.y).abs() < 0.0001);
        assert!((p3.z - e_p3.z).abs() < 0.0001);

        // then translation
        let p4 = translation.clone() * p3;
        assert_eq!(p4, Point::new(15.0, 0.0, 7.0));
    }

    #[test]
    fn chained_transformations() {
        let p = Point::new(1.0, 0.0, 1.0);

        let rotation = Transformation::rotate_x(PI / 2.0);
        let scaling = Transformation::scaling(5.0, 5.0, 5.0);
        let translation = Transformation::translation(10.0, 5.0, 7.0);

        let transformation = translation * scaling * rotation;

        assert_eq!(transformation * p, Point::new(15.0, 0.0, 7.0));
    }
}
