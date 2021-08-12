use raytracer::graphics::canvas::Canvas;
use raytracer::graphics::color;
use raytracer::graphics::color::Color;
use raytracer::graphics::lights::PointLight;
use raytracer::graphics::materials::Material;
use raytracer::graphics::ppm;
use raytracer::math::point::Point;
use raytracer::math::ray::Ray;
use raytracer::objects::shape::Shape;
//use raytracer::math::transformations::Transformation;
use raytracer::objects::sphere::Sphere;

pub fn main() {
    let mut canvas = Canvas::new_color(500, 500, color::BLACK);

    let origin = Point::new(0.0, 0.0, -5.0);

    let mut material = Material::default();
    material.color = Color::new(1.0, 0.2, 1.0); // purple
    material.shininess = 900.0;
    material.ambient = 0.2;
    material.diffuse = 1.0;
    let sphere: Shape = Sphere {
        material,
        ..Default::default()
    }
    .into();

    let light = PointLight::new(Point::new(-5.0, 10.0, -15.0), Color::new(1.0, 1.0, 1.0));

    let wall_z = 10.0;
    let wall_size = 7.0;
    let half_wall = wall_size / 2.0;
    let pixel_size = wall_size / canvas.width as f64;

    for y in 0..canvas.height {
        let world_y = half_wall - pixel_size * y as f64;

        for x in 0..canvas.width {
            let world_x = -half_wall + pixel_size * x as f64;

            let position = Point::new(world_x, world_y, wall_z);

            let ray = Ray::new(origin, (position - origin).normalize());
            let i_wrapped = sphere.intersects(&ray);

            let i = match i_wrapped {
                Some(intersections) => intersections,
                None => continue,
            };

            let intersection = match i.hit() {
                Some(intersection) => intersection,
                None => continue,
            };

            let hit_point = ray.position(intersection.intersects_at);
            let normal = intersection.object.normal_at(hit_point).unwrap();
            let eye = ray.direction;

            let color = sphere
                .material()
                .lighting(light, hit_point, eye, normal, false);
            canvas.write(x, y, color);
        }
    }

    let ppm = ppm::new(canvas);
    println!("{}", ppm)
}
