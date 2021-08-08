use crate::{
    graphics::{
        color::{self, Color},
        lights::PointLight,
    },
    math::{point::Point, ray::Ray, transformations::Transformation},
};

use super::{intersections::Intersections, sphere::Sphere};

/// A world of `objects` (now only `Spheres`!) and `Pointlight`
#[derive(PartialEq, Debug, Clone)]
pub struct World {
    pub objects: Vec<Sphere>,
    pub light: Option<PointLight>,
}

impl Default for World {
    fn default() -> Self {
        let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let mut s1 = Sphere::default();
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;

        let s2 = Sphere::new(Transformation::scaling(0.5, 0.5, 0.5));
        World {
            objects: vec![s1, s2],
            light: Some(light),
        }
    }
}

impl World {
    pub fn new() -> Self {
        World {
            objects: vec![],
            light: None,
        }
    }
    /// intersects every object in the world with the ray, returns sorted Intersections.
    pub fn intersect(&self, ray: Ray) -> Intersections {
        let mut intersections = Intersections { list: vec![] };

        for object in &self.objects {
            let mut i = object
                .intersects(&ray)
                // empty
                .unwrap_or(Intersections { list: vec![] });
            intersections.list.append(&mut i.list);
        }
        intersections
            .list
            .sort_by(|a, b| a.intersects_at.partial_cmp(&b.intersects_at).unwrap());
        intersections
    }

    /// intersects with the world given the ray and then return color at resulting intersection
    pub fn color_at(&self, ray: Ray) -> Color {
        let is = self.intersect(ray.clone());
        if let Some(hit) = is.hit() {
            let comp = hit.prepare_computations(ray).unwrap();
            comp.shade_hit(self)
        } else {
            color::BLACK
        }
    }

    pub fn is_shadowed(&self, point: Point) -> bool {
        if let Some(light) = self.light {
            let v = light.position - point;
            let distance = v.magnitude();
            let direction = v.normalize();

            let ray = Ray::new(point, direction);
            let intersections = self.intersect(ray);
            match intersections.hit() {
                Some(hit) => hit.intersects_at < distance,
                None => false,
            }
        } else {
            false // no light = no shadow
        }
    }
}
