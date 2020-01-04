//! `Camera` type
//!
//! See TRTC chapter 7.

use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::tuple::{Tuple, ORIGIN};

/// Parameters to map the 3D world to a 2D canvas.
pub struct Camera {
    /// Canvas width in pixels.
    hsize: usize,

    /// Canvas height in pixels.
    vsize: usize,

    /// In radians.
    field_of_view: f64,

    /// Canvas half width in world units.
    half_width: f64,

    /// Canvas half height in world units.
    half_height: f64,

    /// Pixel width or height in world units.
    pixel_size: f64,

    pub transform: Matrix,
}

impl Camera {
    /// Constructs a new camera for canvas `(hsize, vsize)`, `field_of_view` angle and
    /// an identity view transformation.
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Camera {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize as f64 / vsize as f64;
        let (half_width, half_height) = if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };
        let pixel_size = (half_width * 2.0) / hsize as f64;
        Camera {
            hsize,
            vsize,
            field_of_view,
            transform: Matrix::new_4x4_identity(),
            half_width,
            half_height,
            pixel_size,
        }
    }

    pub fn hsize(&self) -> usize {
        self.hsize
    }

    pub fn vsize(&self) -> usize {
        self.vsize
    }

    pub fn field_of_view(&self) -> f64 {
        self.field_of_view
    }

    pub fn pixel_size(&self) -> f64 {
        self.pixel_size
    }

    /// Computes a ray cast from the camera to `(x,y)` on canvas.
    pub fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        // Compute point offsets from canvas top-left in world units.
        let x_off = (x as f64 + 0.5) * self.pixel_size;
        let y_off = (y as f64 + 0.5) * self.pixel_size;

        // Compute coordinates of untransformed point in world space.
        let x_world = self.half_width - x_off;
        let y_world = self.half_height - y_off;

        // Compute transformed coordinates.
        // The canvas is at z = -1.
        let t = self.transform.inverted();
        let pixel = &t * &Tuple::new_point(x_world, y_world, -1.0);
        let origin = &t * &ORIGIN;
        let direction = (&pixel - &origin).normalized();

        Ray::new(origin, direction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transform;
    use crate::util;
    use std::f64::consts::PI;

    #[test]
    fn constructing_camera() {
        let c = Camera::new(160, 120, PI / 2.0);
        assert_eq!(c.hsize(), 160);
        assert_eq!(c.vsize(), 120);
        assert_eq!(c.field_of_view(), PI / 2.0);
        assert_eq!(c.transform, Matrix::new_4x4_identity());
    }

    #[test]
    fn pixel_size_for_horizontal_canvas() {
        let c = Camera::new(200, 125, PI / 2.0);
        assert!(util::nearly_equal(c.pixel_size(), 0.01));
    }

    #[test]
    fn pixel_size_for_vertical_canvas() {
        let c = Camera::new(125, 200, PI / 2.0);
        assert!(util::nearly_equal(c.pixel_size(), 0.01));
    }

    #[test]
    fn constructing_ray_through_canvas_center() {
        let c = Camera::new(201, 101, PI / 2.0);
        assert_eq!(
            c.ray_for_pixel(100, 50),
            Ray::from_triplets((0.0, 0.0, 0.0), (0.0, 0.0, -1.0))
        );
    }

    #[test]
    fn constructing_ray_through_canvas_border() {
        let c = Camera::new(201, 101, PI / 2.0);
        assert_eq!(
            c.ray_for_pixel(0, 0),
            Ray::from_triplets((0.0, 0.0, 0.0), (0.66519, 0.33259, -0.66851))
        );
    }

    #[test]
    fn constructing_ray_when_camera_transformed() {
        let mut c = Camera::new(201, 101, PI / 2.0);
        c.transform = &transform::rotation_y(PI / 4.0) * &transform::translation(0.0, -2.0, 5.0);
        assert_eq!(
            c.ray_for_pixel(100, 50),
            Ray::from_triplets(
                (0.0, 2.0, -5.0),
                (2_f64.sqrt() / 2.0, 0.0, -2_f64.sqrt() / 2.0)
            )
        );
    }
}
