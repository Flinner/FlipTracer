use raytracer::graphics::{color, patterns::StripePattern};

mod stripe {
    use super::*;
    use color::{BLACK, WHITE};
    use raytracer::math::point::Point;
    #[test]
    fn creating_pattern() {
        let stripe_pattern = StripePattern::new(WHITE, BLACK);
        let stripe_pattern_test = StripePattern { a: WHITE, b: BLACK };
        assert_eq!(stripe_pattern, stripe_pattern_test)
    }

    #[test]
    fn constant_in_y() {
        let stripe_pattern = StripePattern::new(WHITE, BLACK);
        assert_eq!(stripe_pattern.at(Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(stripe_pattern.at(Point::new(0.0, 1.0, 0.0)), WHITE);
        assert_eq!(stripe_pattern.at(Point::new(0.0, 2.0, 0.0)), WHITE);
    }
    #[test]
    fn constant_in_z() {
        let stripe_pattern = StripePattern::new(WHITE, BLACK);
        assert_eq!(stripe_pattern.at(Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(stripe_pattern.at(Point::new(0.0, 0.0, 1.0)), WHITE);
        assert_eq!(stripe_pattern.at(Point::new(0.0, 0.0, 2.0)), WHITE);
    }
    #[test]
    fn pattern_alternates_in_x() {
        let stripe_pattern = StripePattern::new(WHITE, BLACK);
        assert_eq!(stripe_pattern.at(Point::new(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(stripe_pattern.at(Point::new(0.9, 0.0, 0.0)), WHITE);
        assert_eq!(stripe_pattern.at(Point::new(1.0, 0.0, 0.0)), BLACK);
        assert_eq!(stripe_pattern.at(Point::new(-0.1, 0.0, 0.0)), BLACK);
        assert_eq!(stripe_pattern.at(Point::new(-1.0, 0.0, 0.0)), BLACK);
        assert_eq!(stripe_pattern.at(Point::new(-1.1, 0.0, 0.0)), WHITE);
    }
}
