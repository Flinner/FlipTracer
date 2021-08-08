use crate::{
    graphics::color::Color,
    math::{point::Point, vector::Vector},
};

pub trait Testing {
    fn assert_nearly_eq(a: Self, b: Self);
}

impl Testing for f64 {
    fn assert_nearly_eq(a: Self, b: Self) {
        let assertion = (a - b).abs();
        println!("{},{},{}", a, b, assertion);
        assert!(assertion < 0.00001);
    }
}

impl Testing for f32 {
    fn assert_nearly_eq(a: Self, b: Self) {
        let assertion = (a - b).abs();
        println!("{},{},{}", a, b, assertion);
        assert!(assertion < 0.00001);
    }
}

impl Testing for Color {
    fn assert_nearly_eq(a: Self, b: Self) {
        f64::assert_nearly_eq(a.red, b.red);
        f64::assert_nearly_eq(a.blue, b.blue);
        f64::assert_nearly_eq(a.green, b.green);
    }
}

impl Testing for Point {
    fn assert_nearly_eq(a: Self, b: Self) {
        f64::assert_nearly_eq(a.x, b.x);
        f64::assert_nearly_eq(a.y, b.y);
        f64::assert_nearly_eq(a.z, b.z);
    }
}

impl Testing for Vector {
    fn assert_nearly_eq(a: Self, b: Self) {
        f64::assert_nearly_eq(a.x, b.x);
        f64::assert_nearly_eq(a.y, b.y);
        f64::assert_nearly_eq(a.z, b.z);
    }
}
