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
use std::f64::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4};

fn main() {
    // extreemly flattend floor with mattee textrue
    // let mut floor = Sphere::default();
    // floor.transformation = Transformation::scaling(10.0, 0.01, 10.0);
    // floor.material.color = Color::new(1.0, 0.9, 0.9);
    // floor.material.specular = 0.0;

    let mut floor = shape::default::plane();
    floor.transformation = Transformation::scaling(10.0, 0.01, 10.0);
    floor.material.pattern = Some(Pattern::new(
        color::WHITE,
        color::BLACK,
        Transformation::scaling(0.1, 0.1, 0.1) * Transformation::rotate_y(FRAC_PI_3),
        PatternType::Stripped,
    ));
    floor.material.color = Color::new(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;

    let mut left_wall = shape::default::sphere();
    left_wall.transformation = Transformation::translation(0.0, 0.0, 5.0)
        * Transformation::rotate_y(-FRAC_PI_4)
        * Transformation::rotate_x(FRAC_PI_2)
        * Transformation::scaling(10.0, 0.01, 10.0);
    left_wall.material = floor.material;

    let mut right_wall = shape::default::sphere();
    right_wall.transformation = Transformation::translation(0.0, 0.0, 5.0)
        * Transformation::rotate_y(FRAC_PI_4)
        * Transformation::rotate_x(FRAC_PI_2)
        * Transformation::scaling(10.0, 0.01, 10.0);
    right_wall.material = floor.material;

    let mut middle = shape::default::sphere();
    middle.transformation = Transformation::translation(-0.5, 1.0, 0.5);
    middle.material.color = Color::new(0.1, 1.0, 0.5); // green
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;
    middle.material.pattern = Some(Pattern::new(
        color::RED,
        color::GREEN,
        Transformation::scaling(0.3, 0.3, 0.3) * Transformation::rotate_y(1.0),
        PatternType::Stripped,
    ));

    let mut right = shape::default::sphere();
    right.transformation =
        Transformation::translation(1.5, 0.5, -0.5) * Transformation::scaling(0.5, 0.5, 0.5);
    right.material.color = Color::new(0.5, 1.0, 0.1); // green
    right.material.diffuse = 1.0;
    right.material.specular = 0.3;
    right.material.pattern = Some(Pattern::new(
        color::BLUE,
        color::WHITE,
        Transformation::scaling(0.3, 0.3, 0.3) * Transformation::rotate_z(1.0),
        PatternType::Stripped,
    ));

    let mut left = shape::default::sphere();
    left.transformation =
        Transformation::translation(-1.5, 0.33, -0.75) * Transformation::scaling(0.33, 0.33, 0.33);
    left.material.color = Color::new(1.0, 0.8, 0.1); // yellow
    left.material.diffuse = 0.7;
    left.material.specular = 0.1;

    let mut world = World::new();
    world.light = Some(PointLight::new(
        Point::new(-10.0, 10.0, -10.0),
        Color::new(1.0, 1.0, 1.0),
    ));
    world.objects = vec![
        floor, // left_wall,
        // right_wall,
        middle, right, left,
    ];

    let mut camera = Camera::new(1920, 1080, FRAC_PI_3);
    // let mut camera = Camera::new(300, 300, FRAC_PI_3);
    camera.transform = Transformation::view(
        Point::new(0.0, 1.5, -5.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(world);
    let ppm = ppm::new(canvas);
    println!("{}", ppm);
}
