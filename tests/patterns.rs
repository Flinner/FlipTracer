use color::{BLACK, WHITE};
use raytracer::graphics::{color, patterns::Pattern};
use raytracer::{
    graphics::color::Color,
    graphics::patterns::PatternType,
    math::{point::Point, transformations::Transformation},
    objects::shape::{self, Shape},
};

mod stripe {
    use super::*;
    #[test]
    fn creating_pattern() {
        let stripe_pattern =
            Pattern::stripped(color::WHITE, color::BLACK, Transformation::identity());
        let stripe_pattern_test = Pattern {
            a: WHITE,
            b: BLACK,
            transformation: Transformation::identity(),
            pattern_type: PatternType::Stripped,
        };
        assert_eq!(stripe_pattern, stripe_pattern_test)
    }

    #[test]
    fn constant_in_y() {
        let stripe_pattern =
            Pattern::stripped(color::WHITE, color::BLACK, Transformation::identity());
        assert_eq!(stripe_pattern.at(Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(stripe_pattern.at(Point::new(0.0, 1.0, 0.0)), WHITE);
        assert_eq!(stripe_pattern.at(Point::new(0.0, 2.0, 0.0)), WHITE);
    }
    #[test]
    fn constant_in_z() {
        let stripe_pattern =
            Pattern::stripped(color::WHITE, color::BLACK, Transformation::identity());
        assert_eq!(stripe_pattern.at(Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(stripe_pattern.at(Point::new(0.0, 0.0, 1.0)), WHITE);
        assert_eq!(stripe_pattern.at(Point::new(0.0, 0.0, 2.0)), WHITE);
    }
    #[test]
    fn pattern_alternates_in_x() {
        let stripe_pattern =
            Pattern::stripped(color::WHITE, color::BLACK, Transformation::identity());
        assert_eq!(stripe_pattern.at(Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(stripe_pattern.at(Point::new(0.9, 0.0, 0.0)), WHITE);
        assert_eq!(stripe_pattern.at(Point::new(1.0, 0.0, 0.0)), BLACK);
        assert_eq!(stripe_pattern.at(Point::new(-0.1, 0.0, 0.0)), BLACK);
        assert_eq!(stripe_pattern.at(Point::new(-1.0, 0.0, 0.0)), BLACK);
        assert_eq!(stripe_pattern.at(Point::new(-1.1, 0.0, 0.0)), WHITE);
    }
    #[test]
    fn pattern_with_object_transformation() {
        let mut sphere = shape::default::sphere();
        sphere.transformation = Transformation::scaling(2.0, 2.0, 2.0);
        sphere.material.pattern = Some(Pattern::stripped(WHITE, BLACK, Transformation::identity()));
        let object: Shape = sphere;
        let color = object.pattern_at(Point::new(1.5, 0.0, 0.0)).unwrap();

        assert_eq!(color, WHITE);
    }

    #[test]
    fn pattern_with_pattern_transformation() {
        let mut sphere = shape::default::sphere();
        sphere.material.pattern = Some(Pattern::stripped(
            WHITE,
            BLACK,
            Transformation::scaling(2.0, 2.0, 2.0),
        ));
        let object: Shape = sphere;
        let color = object.pattern_at(Point::new(1.5, 0.0, 0.0)).unwrap();

        assert_eq!(color, WHITE);
    }
    #[test]
    fn pattern_with_object_transformation_and_pattern_transformation() {
        let mut sphere = shape::default::sphere();
        sphere.transformation = Transformation::scaling(2.0, 2.0, 2.0);
        sphere.material.pattern = Some(Pattern::stripped(
            WHITE,
            BLACK,
            Transformation::translation(0.5, 0.0, 0.0),
        ));
        let object: Shape = sphere;
        let color = object.pattern_at(Point::new(2.5, 0.0, 0.0)).unwrap();

        assert_eq!(color, WHITE);
    }
}

mod gradient {

    use super::*;
    #[test]
    fn gradient_linearly_interpolates_between_colors() {
        let pattern = Pattern::gradient(WHITE, BLACK, Transformation::identity());
        assert_eq!(pattern.at(Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(
            pattern.at(Point::new(0.25, 0.0, 0.0)),
            Color::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            pattern.at(Point::new(0.5, 0.0, 0.0)),
            Color::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            pattern.at(Point::new(0.75, 0.0, 0.0)),
            Color::new(0.25, 0.25, 0.25)
        );
    }
}
