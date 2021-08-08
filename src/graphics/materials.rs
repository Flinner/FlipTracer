use super::{
    color::{self, Color},
    lights::PointLight,
};
use crate::math::{point::Point, vector::Vector};

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
    pub fn lighting(
        &self,
        light: PointLight,
        position: Point,
        eye: Vector,
        normal: Vector,
        in_shadow: bool,
    ) -> Color {
        let diffuse;
        let specular;
        let ambient;
        // combine the surface color with the light's color
        let effective_color = self.color * light.color;

        // find the direction of the light source
        let lightv = (light.position - position).normalize();

        // compute the ambient contribution
        ambient = effective_color * self.ambient;

        // light_dot_normal represents the cosine of the angle between
        // light vector and the normal vector. a negative number means
        // that the light is on the other side of the surface
        let light_dot_normal = lightv.dot_product(&normal);

        if in_shadow || light_dot_normal < 0.0 {
            diffuse = color::BLACK;
            specular = color::BLACK;
        } else {
            // compute the diffuse contribution
            diffuse = effective_color * self.diffuse * light_dot_normal;

            //reflect_dot_eye represents the cosine of the angle between the
            // reflection vector and the eye vector. A negative number
            // means that the light reflects away from the eye
            let reflectv = -lightv.reflect(normal);
            let reflect_dot_eye = reflectv.dot_product(&eye);

            if reflect_dot_eye <= 0.0 {
                specular = color::BLACK
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.color * self.specular * factor;
            }
        };
        // add three contributions together to get the final shading
        ambient + diffuse + specular
    }
}
