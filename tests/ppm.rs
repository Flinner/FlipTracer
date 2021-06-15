use raytracer::features::canvas::Canvas;
use raytracer::features::colors::Color;
use raytracer::graphics::ppm;

#[test]
fn header_created() {
    let canvas = Canvas::new(5, 3);

    let ppm = ppm::new(canvas);
    let mut lines = ppm.lines();

    assert_eq!(Some("P3"), lines.next());
    assert_eq!(Some("5 3"), lines.next());
    assert_eq!(Some("255"), lines.next());
}

#[test]
fn pixel_data_constructed() {
    let mut canvas = Canvas::new(5, 3);

    let c1 = Color::new(1.5, 0.0, 0.0);
    let c2 = Color::new(0.0, 0.5, 0.0);
    let c3 = Color::new(-0.5, 0.0, 1.0);

    canvas.write(0, 0, c1);
    canvas.write(2, 1, c2);
    canvas.write(4, 2, c3);

    let image = ppm::new(canvas);
    let image_line: Vec<&str> = image.split("\n").collect();

    assert_eq!("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0", image_line[3]);
    assert_eq!("0 0 0 0 0 0 0 128 0 0 0 0 0 0 0", image_line[4]);
    assert_eq!("0 0 0 0 0 0 0 0 0 0 0 0 0 0 255", image_line[5]);
}