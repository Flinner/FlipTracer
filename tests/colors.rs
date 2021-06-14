use raytracer::features::colors::Color;

#[test]
fn generate_color_with_new() {
    let color = Color::new(-0.5, 0.4, 0.7);
    assert_eq!(color.red, -0.5);
    assert_eq!(color.green, 0.4);
    assert_eq!(color.blue, 0.7);
}

#[test]
fn add_two_colors() {
    let color1 = Color::new(0.9, 0.6, 0.75);
    let color2 = Color::new(0.7, 0.1, 0.25);

    let exepected_color = Color::new(1.6, 0.7, 1.0);
    assert_eq!(color1 + color2, exepected_color)
}

#[test]
fn subtract_two_colors() {
    let color1 = Color::new(0.9, 0.6, 0.25);
    let color2 = Color::new(0.7, 0.1, 0.75);

    let exepected_color = Color::new(0.2, 0.5, -0.5);
    assert_eq!(color1 - color2, exepected_color)
}

#[test]
fn multiply_two_colors() {
    let color1 = Color::new(0.9, 0.6, -1.0);
    let color2 = Color::new(1.0, 0.1, 0.25);

    let exepected_color = Color::new(0.9, 0.06, -0.25);
    assert_eq!(color1 * color2, exepected_color)
}

#[test]
fn multiply_a_color_by_scalar() {
    let color1 = Color::new(0.9, -0.6, 0.75);

    let exepected_color = Color::new(1.8, -1.2, 1.5);
    assert_eq!(color1 * 2.0, exepected_color)
}
