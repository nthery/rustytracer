//! Sphere type.
//!
//! TRTC chapter 5.

use crate::matrix::Matrix;

/// A unit sphere centered on the origin.
#[derive(PartialEq, Debug)]
pub struct Sphere {
    pub transform: Matrix,
}

impl Sphere {
    /// Creates a new unique sphere.
    pub fn new() -> Sphere {
        Sphere {
            transform: Matrix::new_4x4_identity(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::transform;

    #[test]
    fn sphere_default_transformation_is_identity() {
        assert_eq!(Sphere::new().transform, Matrix::new_4x4_identity());
    }

    #[test]
    fn changing_sphere_transformation() {
        let mut s = Sphere::new();
        let t = transform::translation(2.0, 3.0, 4.0);
        s.transform = t.clone();
        assert_eq!(s.transform, t);
    }
}
