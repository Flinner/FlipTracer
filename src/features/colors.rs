use core::f64;
use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, Debug)]
/// RGB color triplet, values idly be between
/// `0` and `1`, but not restriced.
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

/// Red Color, Equivelent of `Color::new(1.0, 0.0, 0.0)`
pub const RED: Color = Color {
    red: 1.0,
    blue: 0.0,
    green: 0.0,
};
/// Green Color, Equivelent of `Color::new(0.0, 1.0, 0.0)`
pub const GREEN: Color = Color {
    red: 0.0,
    green: 1.0,
    blue: 0.0,
};
/// Blue Color, Equivelent of `Color::new(0.0, 0.0, 1.0)`
pub const BLUE: Color = Color {
    red: 0.0,
    green: 0.0,
    blue: 1.0,
};
/// Black Color, Equivelent of `Color::new(0.0, 0.0, 0.0)`
pub const BLACK: Color = Color {
    red: 0.0,
    green: 0.0,
    blue: 0.0,
};
/// White Color, Equivelent of `Color::new(1.0, 1.0, 1.0)`
pub const WHITE: Color = Color {
    red: 1.0,
    green: 1.0,
    blue: 1.0,
};

impl Color {
    /// Generate a new `Color`.
    pub fn new(red: f64, green: f64, blue: f64) -> Color {
        Color { red, green, blue }
    }
}

impl Add for Color {
    type Output = Self;

    /// Add two `Color`s to each other
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl Sub for Color {
    type Output = Self;
    /// Subtract two `Color`s from each other
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red - rhs.red,
            green: self.green - rhs.green,
            blue: self.blue - rhs.blue,
        }
    }
}

impl Mul<Color> for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        use std::f64::EPSILON;
        (self.red - other.red).abs() < EPSILON
            && (self.green - other.green).abs() < EPSILON
            && (self.blue - other.blue).abs() < EPSILON
    }
}
