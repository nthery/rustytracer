//! `Camera` type
//!
//! See TRTC chapter 7.

use crate::matrix::Matrix;

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
}

#[cfg(test)]
mod tests {
    use super::*;
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
}
