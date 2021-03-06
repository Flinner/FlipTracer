use raytracer::graphics::color;
use raytracer::graphics::patterns::Pattern;
use raytracer::graphics::{color::Color, materials::Material};
use raytracer::math::point;
use raytracer::math::transformations::Transformation;
use raytracer::objects::shape::{self, Shape};
use raytracer::{
    graphics::lights::PointLight,
    math::{point::Point, vector::Vector},
};

#[test]
fn new_material_light() {
    let color = Color::new(1.0, 1.0, 1.0);

    let material = Material::new(color, 0.1, 0.9, 0.3, 200.0, 0.5, 0.4, 0.9, None);

    assert_eq!(material.ambient, 0.1);
    assert_eq!(material.diffuse, 0.9);
    assert_eq!(material.specular, 0.3);
    assert_eq!(material.shininess, 200.0);
    assert_eq!(material.reflective, 0.5);
    assert_eq!(material.transparency, 0.4);
    assert_eq!(material.refractive_index, 0.9);
}
#[test]
fn lighting_with_eye_between_the_light_and_surface() {
    let position = point::ORIGIN;
    let material = Material::default();

    let eyev = Vector::new(0.0, 0.0, -1.0);
    let normal = Vector::new(0.0, 0.0, -1.0);
    let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

    let s: Shape = shape::sphere::default();

    let result = material.lighting(s, light, position, eyev, normal, false);
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

    let s: Shape = shape::sphere::default();

    let result = material.lighting(s, light, position, eyev, normal, false);
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

    let s: Shape = shape::sphere::default();

    let result = material.lighting(s, light, position, eyev, normal, false);

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

    let s: Shape = shape::sphere::default();

    let result = material.lighting(s, light, position, eyev, normal, false);

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

    let s: Shape = shape::sphere::default();

    let result = material.lighting(s, light, position, eyev, normal, false);

    let expected = 0.1;
    assert_eq!(result, Color::new(expected, expected, expected));
}

#[test]
fn lighting_with_the_surface_in_shadow() {
    let position = point::ORIGIN;
    let material = Material::default();

    let eyev = Vector::new(0.0, 0.0, -1.0);
    let normal = Vector::new(0.0, 0.0, -1.0);
    let light = PointLight::new(Point::new(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));
    let in_shadow = true;

    let s: Shape = shape::sphere::default();

    let result = material.lighting(s, light, position, eyev, normal, in_shadow);

    let expected = 0.1;
    assert_eq!(result, Color::new(expected, expected, expected));
}

#[test]
fn new_material_with_pattern_applied() {
    let color = Color::new(1.0, 1.0, 1.0);

    let stripe_pattern = Pattern::stripped(color::WHITE, color::BLACK, Transformation::identity());
    let material = Material::new(
        color,
        1.0,
        0.0,
        0.0,
        200.0,
        0.0,
        0.0,
        1.0,
        Some(stripe_pattern),
    );

    let eyev = Vector::new(0.0, 0.0, -1.0);
    let normal = Vector::new(0.0, 0.0, -1.0);
    let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
    let in_shadow = false;

    let s = Shape {
        material,
        ..Default::default()
    }; // useless placeholder

    let c1 = material.lighting(s, light, Point::new(0.9, 0.0, 0.0), eyev, normal, in_shadow);
    let c2 = material.lighting(s, light, Point::new(1.1, 0.0, 0.0), eyev, normal, in_shadow);

    assert_eq!(c1, color::WHITE);
    assert_eq!(c2, color::BLACK);
}

#[test]
fn default_material_reflective() {
    let m = Material::default();
    assert_eq!(m.reflective, 0.0);
}

#[test]
fn transperancy_and_refractive_index_for_default_material() {
    let m = Material::default();
    assert_eq!(m.refractive_index, 1.0);
    assert_eq!(m.transparency, 0.0);
}
