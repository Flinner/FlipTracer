use raytracer::{
    graphics::{color::Color, lights::PointLight},
    math::point::Point,
};

#[test]
fn new_point_light() {
    let point = Point::new(0.0, 0.0, 0.0);
    let color = Color::new(1.0, 1.0, 1.0);

    let light = PointLight::new(point, color);
    assert_eq!(light.color, color);
    assert_eq!(light.position, point);
}
