#![allow(clippy::field_reassign_with_default)]

use raytracer::{
    graphics::{
        camera::Camera,
        color::{self, Color},
        lights::PointLight,
        materials::Material,
        ppm,
    },
    math::{point::Point, transformations::Transformation, vector::Vector},
    objects::{shape, world::World},
};
use std::f64::consts::FRAC_PI_2;

fn main() {
    // let mut camera = Camera::new(3840, 2160, FRAC_PI_3);
    let mut camera = Camera::new(1920, 1080, FRAC_PI_2 * 1.0 / 1.25);
    // let mut camera = Camera::new(100, 100, 0.785);
    camera.transform = Transformation::view(
        Point::new(-6.0, 6.0, -10.0),
        Point::new(6.0, 0.0, 6.0),
        Vector::new(-0.45, 1.0, 0.0),
    );
    let mut world = World::new();

    world.light = Some(PointLight::new(
        Point::new(50.0, 100.0, -50.0),
        Color::new(0.9, 0.9, 0.9),
    ));

    // =======================================
    // define constants to avoid duplication
    // =======================================

    let mut white_material = Material::default();
    white_material.color = color::WHITE;
    white_material.diffuse = 0.7;
    white_material.ambient = 0.1;
    white_material.specular = 0.0;
    white_material.reflective = 0.1;

    let mut blue_material = white_material;
    blue_material.color = Color::new(0.537, 0.831, 0.914);

    let mut red_material = white_material;
    red_material.color = Color::new(0.941, 0.322, 0.388);

    let mut purple_material = white_material;
    purple_material.color = Color::new(0.373, 0.404, 0.550);

    let standard_transform =
        Transformation::translation(1.0, -1.0, 1.0) * Transformation::scaling(0.5, 0.5, 0.5);

    let large_object = standard_transform * Transformation::scaling(3.5, 3.5, 3.5);
    let medium_object = standard_transform * Transformation::scaling(3.0, 3.0, 3.0);
    let small_object = standard_transform * Transformation::scaling(2.0, 2.0, 2.0);

    // =======================================
    // white backdrop for the scene
    // =======================================

    let mut plane = shape::plane::default();
    plane.material.color = color::WHITE;
    plane.material.ambient = 1.0;
    plane.material.diffuse = 0.0;
    plane.material.specular = 0.0;
    plane.transformation =
        Transformation::translation(0.0, 0.0, 500.0) * Transformation::rotate_x(FRAC_PI_2);

    // =======================================
    // elements of the scene
    // =======================================

    let mut sphere = shape::sphere::default();
    sphere.material.color = Color::new(0.373, 0.404, 0.550);
    sphere.material.diffuse = 0.2;
    sphere.material.ambient = 0.0;
    sphere.material.specular = 1.0;
    sphere.material.shininess = 200.0;
    sphere.material.reflective = 0.7;
    sphere.material.transparency = 0.7;
    sphere.material.refractive_index = 1.5;
    sphere.transformation = large_object;

    let mut cube1 = shape::cube::default();
    cube1.material = white_material;
    cube1.transformation = Transformation::translation(4.0, 0.0, 0.0) * medium_object;

    let mut cube2 = shape::cube::default();
    cube2.material = blue_material;
    cube2.transformation = Transformation::translation(8.5, 1.5, -0.5) * medium_object;

    let mut cube3 = shape::cube::default();
    cube3.material = red_material;
    cube3.transformation = Transformation::translation(0.0, 0.0, 4.0) * medium_object;

    let mut cube4 = shape::cube::default();
    cube4.material = white_material;
    cube4.transformation = Transformation::translation(4.0, 0.0, 4.0) * small_object;

    let mut cube5 = shape::cube::default();
    cube5.material = purple_material;
    cube5.transformation = Transformation::translation(7.5, 0.5, 4.0) * medium_object;

    let mut cube6 = shape::cube::default();
    cube6.material = white_material;
    cube6.transformation = Transformation::translation(-0.25, 0.25, 8.0) * medium_object;
    //.

    let mut cube7 = shape::cube::default();
    cube7.material = blue_material;
    cube7.transformation = Transformation::translation(4.0, 1.0, 7.5) * large_object;

    let mut cube8 = shape::cube::default();
    cube8.material = red_material;
    cube8.transformation = Transformation::translation(10.0, 2.0, 7.5) * medium_object;

    let mut cube9 = shape::cube::default();
    cube9.material = white_material;
    cube9.transformation = Transformation::translation(8.0, 2.0, 12.0) * small_object;

    // next page
    let mut cube10 = shape::cube::default();
    cube10.material = white_material;
    cube10.transformation = Transformation::translation(20.0, 1.0, 9.0) * small_object;
    let mut cube11 = shape::cube::default();
    cube11.material = blue_material;
    cube11.transformation = Transformation::translation(-0.5, -5.0, 0.25) * large_object;
    let mut cube12 = shape::cube::default();
    cube12.material = red_material;
    cube12.transformation = Transformation::translation(4.0, -4.0, 0.0) * large_object;
    let mut cube13 = shape::cube::default();
    cube13.material = white_material;
    cube13.transformation = Transformation::translation(8.5, -4.0, 0.0) * large_object;
    let mut cube14 = shape::cube::default();
    cube14.material = white_material;
    cube14.transformation = Transformation::translation(0.0, -4.0, 4.0) * large_object;
    let mut cube15 = shape::cube::default();
    cube15.material = purple_material;
    cube15.transformation = Transformation::translation(-0.5, -4.5, 8.0) * large_object;
    let mut cube16 = shape::cube::default();
    cube16.material = white_material;
    cube16.transformation = Transformation::translation(0.0, -8.0, 4.0) * large_object;
    let mut cube17 = shape::cube::default();
    cube17.material = white_material;
    cube17.transformation = Transformation::translation(-0.5, -8.5, 8.0) * large_object;

    world.objects = vec![
        plane, sphere, cube1, cube2, cube3, cube4, cube5, cube6, cube7, cube8, cube9,
        //.
        cube10, cube11, cube12, cube13, cube14, cube15, cube16, cube17,
    ];

    let canvas = camera.render(world);
    let ppm = ppm::new(canvas);
    println!("{}", ppm);
}
