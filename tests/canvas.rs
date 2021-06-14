//
use raytracer::features::{canvas::Canvas, colors};

#[test]
fn create_empty_canvas() {
    let canvas = Canvas::new(10, 20);

    assert_eq!(canvas.height, 20);
    assert_eq!(canvas.width, 10);
    assert_eq!(canvas.grid, vec![vec![colors::BLACK; 10]; 20]);
}

#[test]
fn write_to_canvas() {
    let mut canvas = Canvas::new(10, 20);
    canvas.write(2, 3, colors::RED);

    assert_eq!(canvas.grid[2][3], colors::RED);
}
