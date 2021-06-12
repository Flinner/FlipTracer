use raytracer::features::colors::Color;

#[test]
fn generate_color_with_new() {
    let color = Color::new(-0.5, 0.4, 0.7);
    assert_eq!(color.red, -0.5);
    assert_eq!(color.green, 0.4);
    assert_eq!(color.blue, 0.7);
}
