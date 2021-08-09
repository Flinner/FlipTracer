use raytracer::graphics::canvas::Canvas;
use raytracer::graphics::color;
use raytracer::graphics::ppm;
use raytracer::math::point::Point;

pub fn main() {
    let mut canvas = Canvas::new_color(100, 100, color::WHITE);
    let point = Point::new(0.0, 0.0, 1.0);

    let angle = 30.0_f64.to_radians();
    let color = color::RED;
    let radius = 25.0;
    let center = canvas.height as f64 / 2.0;

    (0..12).for_each(|i| {
        // println!(
        //     "POINT: z:{}, x:{}, angle:{}",
        //     (point.rotate_y(angle * i as f64).z * 10.0 + 50.0) as usize,
        //     (point.rotate_y(angle * i as f64).x * 10.0 + 50.0) as usize,
        //     angle.to_degrees() * i as f64
        // );
        canvas.write(
            (point.rotate_y(angle * i as f64).z * radius + center) as usize,
            (point.rotate_y(angle * i as f64).x * radius + center) as usize,
            color,
        );
    });

    let ppm = ppm::new(canvas);
    println!("{}", ppm);
}
