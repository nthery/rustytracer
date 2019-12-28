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
    /// Creates a new sphere with identity transformation.
    pub fn new() -> Sphere {
        Sphere {
            transform: Matrix::new_4x4_identity(),
        }
    }

    /// Creates a new sphere with the speficied transformation.
    pub fn with_transform(t: Matrix) -> Sphere {
        Sphere { transform: t }
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
    fn creating_sphere_with_non_default_transformation() {
        let t = transform::scaling(1.0, 2.0, 3.0);
        let s = Sphere::with_transform(t.clone());
        assert_eq!(s.transform, t);
    }

    #[test]
    fn changing_sphere_transformation() {
        let mut s = Sphere::new();
        let t = transform::translation(2.0, 3.0, 4.0);
        s.transform = t.clone();
        assert_eq!(s.transform, t);
    }
}
