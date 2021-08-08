use crate::{
    math::{point::Point, ray::Ray, transformations::Transformation},
    objects::world::World,
};

use super::canvas::Canvas;

#[derive(Debug, Clone, PartialEq)]
/// Camera has the canvas always one unit away.
pub struct Camera {
    /// horizontal
    pub hsize: usize,
    /// vertical
    pub vsize: usize,
    /// field of view, (angle in Radians)
    pub fov: f64,
    /// defaults to identity
    pub transform: Transformation,
    /// pixel size in world units
    pub pixel_size: f64,
    /// half the width of the canvas
    pub half_width: f64,
    /// half the height of the canvas
    pub half_height: f64,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, fov: f64) -> Self {
        let transform = Transformation::identity();
        let (pixel_size, half_width, half_height) =
            Camera::pixel_size_width_and_height(hsize as f64, vsize as f64, fov);

        Self {
            hsize,
            vsize,
            fov,
            transform,
            pixel_size,
            half_width,
            half_height,
        }
    }

    /// returns (pixel_size, half_width, half_height)
    fn pixel_size_width_and_height(hsize: f64, vsize: f64, fov: f64) -> (f64, f64, f64) {
        // width of half of the canvas
        let half_view = (fov / 2.0).tan();

        let half_height;
        let half_width;

        let aspect = hsize / vsize;

        if aspect < 1.0 {
            // vsize > hsize
            half_width = half_view * aspect;
            half_height = half_view;
        } else {
            half_width = half_view;
            half_height = half_view / aspect;
        }
        ((half_width * 2.0) / hsize, half_width, half_height)
    }

    /// returns a new ray starting at camera and passing through pixel on canvas
    /// x and y indicate pixel position.
    fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        // offset from the edge of the canvas to the pixel's center
        let xoffset = (x as f64 + 0.5) * self.pixel_size;
        let yoffset = (y as f64 + 0.5) * self.pixel_size;

        // untransformed coordinates of the pixel in world space.
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let transformation = self
            .transform
            .clone()
            .inverse()
            .expect("Illegal Camera Transformation!");

        // using the camera matrix, transform canvas point and the origin.
        let pixel = transformation.clone() * Point::new(world_x, world_y, -1.0);
        let origin = transformation * Point::new(0.0, 0.0, 0.0);

        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }

    pub fn render(&self, world: World) -> Canvas {
        let mut image = Canvas::new(self.hsize as usize, self.vsize as usize);
        for y in 0..(self.vsize as usize) {
            for x in 0..(self.hsize as usize) {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(ray);
                image.write(x, y, color);
            }
        }
        image
    }
}

// ================================= TESTS =======================================

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, SQRT_2};

    use crate::{
        math::{point::Point, vector::Vector},
        testing::Testing,
    };

    use super::*;

    #[test]
    fn ray_through_center_of_canvas() {
        let camera = Camera::new(201, 101, FRAC_PI_2);
        let ray = camera.ray_for_pixel(100, 50);

        assert_eq!(ray.origin, Point::new(0.0, 0.0, 0.0));
        assert_eq!(ray.direction, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn ray_through_corner_of_canvas() {
        let camera = Camera::new(201, 101, FRAC_PI_2);
        let ray = camera.ray_for_pixel(0, 0);

        assert_eq!(ray.origin, Point::new(0.0, 0.0, 0.0));
        Testing::assert_nearly_eq(ray.direction, Vector::new(0.66519, 0.33259, -0.66851))
    }

    #[test]
    fn ray_when_camera_is_transformed() {
        let mut camera = Camera::new(201, 101, FRAC_PI_2);
        camera.transform =
            Transformation::rotate_y(FRAC_PI_4) * Transformation::translation(0.0, -2.0, 5.0);
        let ray = camera.ray_for_pixel(100, 50);

        assert_eq!(ray.origin, Point::new(0.0, 2.0, -5.0));
        Testing::assert_nearly_eq(ray.direction, Vector::new(SQRT_2 / 2.0, 0.0, -SQRT_2 / 2.0))
    }
}
