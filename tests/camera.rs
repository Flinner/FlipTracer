use std::f64::consts;

use raytracer::{graphics::camera::Camera, math::transformations::Transformation};

#[test]
fn constructing_a_camera() {
    let hsize = 160.0;
    let vsize = 120.0;
    let field_of_view = consts::FRAC_PI_2;

    let c = Camera::new(hsize, vsize, field_of_view);

    assert_eq!(c.hsize, 160.0);
    assert_eq!(c.vsize, 120.0);
    assert_eq!(c.fov, consts::PI / 2.0);
    assert_eq!(c.transform, Transformation::identity());
}

#[test]
fn pixel_size_of_horizontal_canvas() {
    let c = Camera::new(200.0, 125.0, consts::FRAC_PI_2);
    assert_nearly_eq(c.pixel_size, 0.01)
}

fn assert_nearly_eq(a: f64, b: f64) {
    let assertion = (a - b).abs();
    println!("{},{},{}", a, b, assertion);
    assert!(assertion < 0.00001)
}
