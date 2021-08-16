use crate::{
    graphics::{
        color::{self, Color},
        lights::PointLight,
        materials::Material,
    },
    math::{point::Point, ray::Ray, transformations::Transformation},
    objects::shape,
};

use super::{
    intersections::{Intersections, PreComputed},
    shape::Shape,
};

/// A world of `objects` (now only `Spheres`!) and `Pointlight`
#[derive(PartialEq, Debug, Clone)]
pub struct World {
    pub objects: Vec<Shape>,
    pub light: Option<PointLight>,
}

impl Default for World {
    fn default() -> Self {
        let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let mut s1 = Shape::default();
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;

        let s2 = shape::sphere::new(Transformation::scaling(0.5, 0.5, 0.5), Material::default());
        World {
            objects: vec![s1, s2],
            light: Some(light),
        }
    }
}

impl World {
    /// An empty new world
    pub fn new() -> Self {
        World {
            objects: vec![],
            light: None,
        }
    }
    /// intersects every object in the world with the ray, returns sorted Intersections.
    pub fn intersect(&self, ray: Ray) -> Intersections {
        let mut intersections = Intersections { list: vec![] };

        self.objects.iter().for_each(|object| {
            let mut i = object
                .intersects(&ray)
                // empty
                .unwrap_or(Intersections { list: vec![] });
            intersections.list.append(&mut i.list);
        });
        intersections
            .list
            .sort_by(|a, b| a.intersects_at.partial_cmp(&b.intersects_at).unwrap());
        intersections
    }

    /// intersects with the world given the ray and then return color at resulting intersection
    /// `remaining` is the number of recurisive calls left. this is to prevent infinite recursion
    pub fn color_at(&self, ray: Ray, remaining: usize) -> Color {
        let is = self.intersect(ray);
        if let Some(hit) = is.hit() {
            let comp = hit.prepare_computations(ray, Some(&is)).unwrap();
            self.shade_hit(&comp, remaining - 1)
        } else {
            color::BLACK
        }
    }
    /// Returns true if there is a shadow
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

    /// calculates the the color at intersection (from `PreComputed`)
    /// `remaining` is the number of recurisive calls left. this is to prevent infinite recursion
    pub fn shade_hit(&self, comps: &PreComputed, remaining: usize) -> Color {
        let shadowed = self.is_shadowed(comps.over_point);

        // color from surface
        let surface = comps.object.material.lighting(
            comps.object,
            self.light.expect("no light!"),
            comps.over_point,
            comps.eyev,
            comps.normalv,
            shadowed,
        );
        // color from reflection
        let reflected = self.reflected_color(comps, remaining - 1);
        let refracted = self.refracted_color(comps, remaining - 1);

        let material = comps.object.material;

        // if both reflective and transparenct. use schlick formula to get Fresnel effect
        if material.reflective > 0.0 && material.transparency > 0.0 {
            let reflectance = comps.schlick();
            surface + reflected * reflectance + refracted * (1.0 - reflectance)
        } else {
            // final color
            reflected + surface + refracted
        }
    }

    /// color of reflected ray
    /// `remaining` is the number of recurisive calls left. this is to prevent infinite recursion
    /// if `remaining` is zero, the function will return `color::BLACK`
    pub fn reflected_color(&self, comps: &PreComputed, remaining: usize) -> Color {
        // reflection to begin with
        if comps.object.material.reflective == 0.0
	    // end recurisive reflection
	    || remaining == 0
        {
            color::BLACK
        } else {
            let reflect_ray = Ray::new(comps.over_point, comps.reflectv);
            let color = self.color_at(reflect_ray, remaining - 1);

            // "dilute" the color with reflective
            color * comps.object.material.reflective
        }
    }

    /// color of refracted ray
    /// `remaining` is the number of recurisive calls left. this is to prevent infinite recursion
    /// if `remaining` is zero, the function will return `color::BLACK`
    pub fn refracted_color(&self, comps: &PreComputed, remaining: usize) -> Color {
        // ratio of first index of refraction to the second.
        let n_ratio = comps.refractive_exited / comps.refractive_entered;
        let cos_i = comps.eyev.dot_product(&comps.normalv);
        let sin2_t = n_ratio.powi(2) * (1.0 - cos_i.powi(2));

        // reflection to begin with
        if comps.object.material.transparency == 0.0
	    // end recurisive reflection
	    || remaining == 0
	    || sin2_t > 1.0
        {
            color::BLACK
        } else {
            // finding refracted ray
            let cos_t = (1.0 - sin2_t).sqrt();
            let direction = comps.normalv * (n_ratio * cos_i - cos_t) - //.
		comps.eyev * n_ratio;

            // create refracted ray
            let refract_ray = Ray::new(comps.under_point, direction);
            self.color_at(refract_ray, remaining - 1) * //.
		comps.object.material.transparency
        }
    }
}
