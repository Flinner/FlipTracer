use raytracer::graphics::{color::Color, materials::Material};

#[test]
fn new_material_light() {
    let color = Color::new(1.0, 1.0, 1.0);

    let material = Material::new(color, 0.1, 0.9, 0.3, 200.0);

    assert_eq!(material.ambient, 0.1);
    assert_eq!(material.diffuse, 0.9);
    assert_eq!(material.specular, 0.3);
    assert_eq!(material.shininess, 200.0);
}
