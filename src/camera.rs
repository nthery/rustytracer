//! `Camera` type
//!
//! See TRTC chapter 7.

use crate::matrix::Matrix;

/// Parameters to map the 3D world to a 2D canvas.
pub struct Camera {
    hsize: usize,
    vsize: usize,
    field_of_view: f64,
    pub transform: Matrix,
}

impl Camera {
    /// Constructs a new camera for canvas `(hsize, vsize)`, `field_of_view` angle and
    /// an identity view transformation.
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Camera {
        Camera {
            hsize,
            vsize,
            field_of_view,
            transform: Matrix::new_4x4_identity(),
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn constructing_camera() {
        let c = Camera::new(160, 120, PI / 2.0);
        assert_eq!(c.hsize(), 160);
        assert_eq!(c.vsize(), 120);
        assert_eq!(c.field_of_view(), PI / 2.0);
        assert_eq!(c.transform, Matrix::new_4x4_identity());
    }
}
