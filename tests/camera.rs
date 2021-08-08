use std::{f64::consts, f64::consts::FRAC_PI_2};

use raytracer::{
    graphics::{camera::Camera, color::Color},
    math::{point::Point, transformations::Transformation, vector::Vector},
    objects::world::World,
    testing::Testing,
};

#[test]
fn constructing_a_camera() {
    let hsize = 160;
    let vsize = 120;
    let field_of_view = consts::FRAC_PI_2;

    let c = Camera::new(hsize, vsize, field_of_view);

    assert_eq!(c.hsize, 160);
    assert_eq!(c.vsize, 120);
    assert_eq!(c.fov, consts::PI / 2.0);
    assert_eq!(c.transform, Transformation::identity());
}

#[test]
fn pixel_size_of_horizontal_canvas() {
    let c = Camera::new(200, 125, consts::FRAC_PI_2);
    Testing::assert_nearly_eq(c.pixel_size, 0.01)
}
#[test]
fn pixel_size_of_vertical_canvas() {
    let c = Camera::new(125, 200, consts::FRAC_PI_2);
    Testing::assert_nearly_eq(c.pixel_size, 0.01)
}

#[test]
fn rendering_a_world() {
    let w = World::default();
    let mut c = Camera::new(11, 11, FRAC_PI_2);

    let from = Point::new(0.0, 0.0, -5.0);
    let to = Point::new(0.0, 0.0, 0.0);
    let up = Vector::new(0.0, 1.0, 0.0);
    c.transform = Transformation::view(from, to, up);
    println!("transform: {:?}", c.transform);

    let image = c.render(w);

    Testing::assert_nearly_eq(image.get(5, 5), Color::new(0.38066, 0.47583, 0.2855))
}
