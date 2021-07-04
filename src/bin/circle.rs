use raytracer::graphics::canvas::Canvas;
use raytracer::graphics::color;
use raytracer::graphics::ppm;
use raytracer::math::point::Point;
use raytracer::math::ray::Ray;
use raytracer::math::transformations::Transformation;
use raytracer::objects::sphere::Sphere;

pub fn main() {
    let mut canvas = Canvas::new_color(100, 100, color::BLACK);

    let origin = Point::new(0.0, 0.0, -5.0);
    let color = color::RED;
    let transformation = Transformation::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0)
        * Transformation::scaling(1.0, 0.5, 0.3);

    let sphere = Sphere::new_with_transformation(transformation);

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
            let i_wrapped = sphere.intersects(ray);

            let i = match i_wrapped {
                Some(intersections) => intersections,
                None => continue,
            };

            match i.hit() {
                Some(intersection) => intersection,
                None => continue,
            };

            canvas.write(x, y, color);
        }
    }

    let ppm = ppm::new(canvas);
    println!("{}", ppm)
}
