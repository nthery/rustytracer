//! Sphere type.
//!
//! TRTC chapter 5.

use crate::light::Material;
use crate::matrix::Matrix;
use crate::tuple::{Tuple, ORIGIN};

/// A unit sphere centered on the origin.
#[derive(PartialEq, Debug)]
pub struct Sphere {
    pub transform: Matrix,
    pub material: Material,
}

impl Sphere {
    /// Creates a new sphere with identity transformation.
    pub fn new() -> Sphere {
        Sphere {
            transform: Matrix::new_4x4_identity(),
            material: Material::default(),
        }
    }

    /// Creates a new sphere with the speficied transformation.
    pub fn with_transform(t: Matrix) -> Sphere {
        Sphere {
            transform: t,
            material: Material::default(),
        }
    }

    /// Computes normal vector on this sphere at point `p` in world space.
    pub fn normal_at(&self, world_pt: &Tuple) -> Tuple {
        debug_assert!(world_pt.is_point());

        let it = self.transform.inverted();
        let obj_pt = &it * world_pt;
        let obj_normal = &obj_pt - &ORIGIN;
        let mut world_normal = &it.transposed() * &obj_normal;
        world_normal.set(3, 0.0);
        world_normal.normalized()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transform;
    use std::f64::consts::PI;

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

    #[test]
    fn normal_at_point_on_x_axis() {
        assert_eq!(
            Sphere::new().normal_at(&Tuple::new_point(1.0, 0.0, 0.0)),
            Tuple::new_vector(1.0, 0.0, 0.0)
        );
    }

    #[test]
    fn normal_at_point_on_y_axis() {
        assert_eq!(
            Sphere::new().normal_at(&Tuple::new_point(0.0, 1.0, 0.0)),
            Tuple::new_vector(0.0, 1.0, 0.0)
        );
    }

    #[test]
    fn normal_at_point_on_z_axis() {
        assert_eq!(
            Sphere::new().normal_at(&Tuple::new_point(0.0, 0.0, 1.0)),
            Tuple::new_vector(0.0, 0.0, 1.0)
        );
    }

    #[test]
    fn normal_at_non_axial_point() {
        let z = 3_f64.sqrt() / 3.0;
        assert_eq!(
            Sphere::new().normal_at(&Tuple::new_point(z, z, z)),
            Tuple::new_vector(z, z, z)
        );
    }

    #[test]
    fn normal_is_normalized() {
        let z = 3_f64.sqrt() / 3.0;
        let n = Sphere::new().normal_at(&Tuple::new_point(z, z, z));
        assert_eq!(n, n.normalized());
    }

    #[test]
    fn normal_on_translated_sphere() {
        let s = Sphere::with_transform(transform::translation(0.0, 1.0, 0.0));
        assert_eq!(
            s.normal_at(&Tuple::new_point(0.0, 1.70711, -0.70711)),
            Tuple::new_vector(0.0, 0.70711, -0.70711)
        );
    }

    #[test]
    fn normal_on_transformed_sphere() {
        let s = Sphere::with_transform(
            &transform::scaling(1.0, 0.5, 1.0) * &transform::rotation_z(PI / 5.0),
        );
        assert_eq!(
            s.normal_at(&Tuple::new_point(
                0.0,
                2_f64.sqrt() / 2.0,
                -2_f64.sqrt() / 2.0
            )),
            Tuple::new_vector(0.0, 0.97014, -0.24254)
        );
    }
}
