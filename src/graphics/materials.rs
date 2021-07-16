use super::color::Color;

/// Bui Tuong Phong Material
#[derive(Clone, Copy, Debug)]
pub struct Material {
    pub color: Color,
    /// Value between 0 and 1
    pub ambient: f64,
    /// Value between 0 and 1
    pub diffuse: f64,
    /// Value between 0 and 1
    pub specular: f64,
    /// Value between 10 and 200 work best,
    /// no limits apart from `f64`
    pub shininess: f64,
}

impl Material {
    pub fn new(color: Color, ambient: f64, diffuse: f64, specular: f64, shininess: f64) -> Self {
        Self {
            ambient,
            color,
            diffuse,
            specular,
            shininess,
        }
    }
}
