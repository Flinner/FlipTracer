//
use raytracer::graphics::{canvas::Canvas, color};

#[test]
fn create_empty_canvas() {
    let canvas = Canvas::new(10, 20);

    assert_eq!(canvas.height, 20);
    assert_eq!(canvas.width, 10);
    assert_eq!(canvas.grid, vec![color::BLACK; 200]);
}

#[test]
fn write_to_canvas() {
    let mut canvas = Canvas::new(10, 20);
    canvas.write(2, 3, color::RED);

    assert_eq!(canvas.grid[32], color::RED);
}

#[test]
fn get_pixel_at_coordinates() {
    let mut canvas = Canvas::new(10, 20);
    canvas.write(2, 3, color::RED);

    assert_eq!(canvas.get(2, 3), color::RED);
}
