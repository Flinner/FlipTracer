use raytracer::graphics::{color::Color, materials::Material};
use raytracer::math::point;
use raytracer::{
    graphics::lights::PointLight,
    math::{point::Point, vector::Vector},
};
#[test]
fn new_material_light() {
    let color = Color::new(1.0, 1.0, 1.0);

    let material = Material::new(color, 0.1, 0.9, 0.3, 200.0);

    assert_eq!(material.ambient, 0.1);
    assert_eq!(material.diffuse, 0.9);
    assert_eq!(material.specular, 0.3);
    assert_eq!(material.shininess, 200.0);
}
#[test]
fn lighting_with_eye_between_the_light_and_surface() {
    let position = point::ORIGIN;
    let material = Material::default();

    let eyev = Vector::new(0.0, 0.0, -1.0);
    let normal = Vector::new(0.0, 0.0, -1.0);
    let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

    let result = material.lighting(light, position, eyev, normal);
    assert_eq!(result, Color::new(1.9, 1.9, 1.9));
}

#[test]
fn lighting_with_eye_between_the_light_and_surface_eye_offset_45() {
    let sqrt2_by2 = f64::sqrt(2.0) / 2.0;

    let position = point::ORIGIN;
    let material = Material::default();

    let eyev = Vector::new(0.0, sqrt2_by2, -sqrt2_by2);
    let normal = Vector::new(0.0, 0.0, -1.0);
    let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

    let result = material.lighting(light, position, eyev, normal);
    assert_eq!(result, Color::new(1.0, 1.0, 1.0));
}

#[test]
fn lighting_with_eye_between_the_light_and_surface_light_offset_45() {
    let sqrt2_by2 = f64::sqrt(2.0) / 2.0;

    let position = point::ORIGIN;
    let material = Material::default();

    let eyev = Vector::new(0.0, 0.0, -1.0);
    let normal = Vector::new(0.0, 0.0, -1.0);
    let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

    let result = material.lighting(light, position, eyev, normal);

    let expected = 0.1 + 0.9 * sqrt2_by2 + 0.0;
    assert_eq!(result, Color::new(expected, expected, expected));
}

#[test]
fn lighting_with_eye_opposite_surface_light_offset_45() {
    let sqrt2_by2 = f64::sqrt(2.0) / 2.0;

    let position = point::ORIGIN;
    let material = Material::default();

    let eyev = Vector::new(0.0, -sqrt2_by2, -sqrt2_by2);
    let normal = Vector::new(0.0, 0.0, -1.0);
    let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

    let result = material.lighting(light, position, eyev, normal);

    let expected = 0.1 + 0.9 * sqrt2_by2 + 0.9;
    assert_eq!(result, Color::new(expected, expected, expected));
}

#[test]
fn lighting_with_light_behind_surface() {
    let position = point::ORIGIN;
    let material = Material::default();

    let eyev = Vector::new(0.0, 0.0, -1.0);
    let normal = Vector::new(0.0, 0.0, -1.0);
    let light = PointLight::new(Point::new(0.0, 10.0, 10.0), Color::new(1.0, 1.0, 1.0));

    let result = material.lighting(light, position, eyev, normal);

    let expected = 0.1;
    assert_eq!(result, Color::new(expected, expected, expected));
}
