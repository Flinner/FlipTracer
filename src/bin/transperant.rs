#![allow(clippy::field_reassign_with_default)]

use raytracer::{
    graphics::{
        camera::Camera,
        color::{self, Color},
        lights::PointLight,
        patterns::{Pattern, PatternType},
        ppm,
    },
    math::{point::Point, transformations::Transformation, vector::Vector},
    objects::{shape, world::World},
};
use std::f64::consts::{FRAC_PI_2, FRAC_PI_3};

fn main() {
    // extreemly flattend floor with mattee textrue
    // let mut floor = Sphere::default();
    // floor.transformation = Transformation::scaling(10.0, 0.01, 10.0);
    // floor.material.color = Color::new(1.0, 0.9, 0.9);
    // floor.material.specular = 0.0;

    let mut floor = shape::plane::default();
    floor.material.pattern = Some(Pattern::new(
        color::WHITE,
        color::BLACK,
        Transformation::identity(),
        PatternType::Checker,
    ));
    floor.transformation = Transformation::rotate_x(FRAC_PI_2) //.
	* Transformation::translation(0.0, 0.0, 10.0)
	* Transformation::scaling(0.2, 0.2, 0.2);
    floor.material.ambient = 0.7;
    floor.material.diffuse = 0.2;
    floor.material.specular = 0.0;

    let mut glass = shape::sphere::default();
    // glass_sphere.transformation = Transformation::translation(0.0, 1.0, 0.0);
    glass.material.diffuse = 0.0;
    glass.material.specular = 0.9;
    glass.material.shininess = 300.0;
    glass.material.reflective = 0.9;
    glass.material.transparency = 0.9;
    glass.material.refractive_index = 1.5;
    glass.material.color = Color::new(0.0, 0.0, 0.0);

    let mut air = shape::sphere::default();
    air.transformation = Transformation::scaling(0.5, 0.5, 0.5);
    air.material.ambient = 0.0;
    air.material.diffuse = 0.0;
    air.material.specular = 0.9;
    air.material.shininess = 300.0;
    air.material.reflective = 0.9;
    air.material.transparency = 0.9;
    air.material.refractive_index = 1.0000034;
    air.material.color = Color::new(1.0, 1.0, 1.0);

    let mut world = World::new();
    world.light = Some(PointLight::new(
        Point::new(2.0, 10.0, -5.0),
        Color::new(0.9, 0.9, 0.9),
    ));
    world.objects = vec![
        floor, // left_wall, right_wall,
        glass, air,
    ];

    // let mut camera = Camera::new(1920, 1080, FRAC_PI_3);
    // let mut camera = Camera::new(3840, 2160, FRAC_PI_3);
    let mut camera = Camera::new(300, 300, 0.45);
    camera.transform = Transformation::view(
        Point::new(0.0, 0.0, -5.0),
        Point::new(0.0, 0.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(world);
    let ppm = ppm::new(canvas);
    println!("{}", ppm);
}
