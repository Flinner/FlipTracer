use super::color::Color;

/// Bui Tuong Phong Material
#[derive(Clone, PartialEq, Copy, Debug)]
pub struct Material {
    pub color: Color,
    /// Value between 0 and 1, default: 0.1
    pub ambient: f64,
    /// Value between 0 and 1, default: 0.9
    pub diffuse: f64,
    /// Value between 0 and 1, default: 0.9
    pub specular: f64,
    /// Value between 10 and 200 work best,default: 200.0
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
    /// default material
    pub fn default() -> Self {
        Self {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}
