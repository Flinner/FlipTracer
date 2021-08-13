use crate::math::{point::Point, transformations::Transformation};

use super::color::{self, Color};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pattern {
    pub a: Color,
    pub b: Color,
    pub transformation: Transformation,
    pub pattern_type: PatternType,
}

impl Default for Pattern {
    fn default() -> Self {
        Self::new(
            color::WHITE,
            color::BLACK,
            Transformation::identity(),
            PatternType::Stripped,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PatternType {
    /// Zebra like stripes
    Stripped,
    /// Gradient with two colors
    Gradient,
    /// Circlur Stripes
    Ring,
    /// Checkers, like Chess, but in 3D space too
    Checker,
}

impl Pattern {
    /// new `Pattern`
    pub fn new(
        a: Color,
        b: Color,
        transformation: Transformation,
        pattern_type: PatternType,
    ) -> Self {
        Self {
            a,
            b,
            transformation,
            pattern_type,
        }
    }

    /// new `Stripped` Pattern
    pub fn stripped(a: Color, b: Color, transformation: Transformation) -> Self {
        Self::new(a, b, transformation, PatternType::Stripped)
    }
    /// new `Gradient` Pattern
    pub fn gradient(a: Color, b: Color, transformation: Transformation) -> Self {
        Self::new(a, b, transformation, PatternType::Gradient)
    }
    /// new `Ring` Pattern
    pub fn ring(a: Color, b: Color, transformation: Transformation) -> Self {
        Self::new(a, b, transformation, PatternType::Ring)
    }

    // returns the stripe pattern at point
    /// uses pattern space point, not World-Space, or Object-Space!
    pub fn at(&self, pattern_point: Point) -> Color {
        match self.pattern_type {
            PatternType::Stripped => color_at::stripped(self, pattern_point),
            PatternType::Gradient => color_at::gradient(self, pattern_point),
            PatternType::Ring => color_at::ring(self, pattern_point),
            PatternType::Checker => color_at::checker(self, pattern_point),
        }
    }
}

/// uses pattern space point, not World-Space, or Object-Space!
mod color_at {
    use super::*;

    /// Returns the color caused by `Stripped` Pattern
    pub(super) fn stripped(pattern: &Pattern, pattern_point: Point) -> Color {
        if pattern_point.x.floor() % 2.0 == 0.0 {
            pattern.a
        } else {
            pattern.b
        }
    }

    /// Returns the color caused by `Gradient` Pattern
    pub(super) fn gradient(pattern: &Pattern, pattern_point: Point) -> Color {
        let distance = pattern.b - pattern.a;
        let fraction = pattern_point.x - pattern_point.x.floor();
        pattern.a + distance * fraction
    }

    /// Returns the color caused by `Ring` Pattern
    pub(super) fn ring(pattern: &Pattern, pattern_point: Point) -> Color {
        let p = pattern_point;

        // distance from radius
        let radius = (p.x.powf(2.0) + p.z.powf(2.0)).sqrt();

        if radius.floor() % 2.0 == 0.0 {
            pattern.a
        } else {
            pattern.b
        }
    }

    /// Returns the color caused by `Checker` Pattern
    pub(super) fn checker(_pattern: &Pattern, _pattern_pointt: Point) -> Color {
        todo!()
    }
}
