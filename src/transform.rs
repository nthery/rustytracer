//! This module implements matrix transformations.
//!
//! See TRTC chapter 4.

use crate::matrix::Matrix;

/// Returns a matrix that encodes the given translation.
pub fn translation(dx: f64, dy: f64, dz: f64) -> Matrix {
    let mut res = Matrix::new_4x4_identity();
    res.set(0, 3, dx);
    res.set(1, 3, dy);
    res.set(2, 3, dz);
    res
}

/// Returns a matrix that encodes the given scaling.
pub fn scaling(x: f64, y: f64, z: f64) -> Matrix {
    let mut res = Matrix::new_4x4_identity();
    res.set(0, 0, x);
    res.set(1, 1, y);
    res.set(2, 2, z);
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::Tuple;

    #[test]
    fn multiplying_by_translation_matrix() {
        let t = translation(5.0, -3.0, 2.0);
        assert_eq!(
            &t * &Tuple::new_point(-3.0, 4.0, 5.0),
            Tuple::new_point(2.0, 1.0, 7.0)
        );
    }

    #[test]
    fn multiplying_by_inverted_translation_matrix() {
        let t = translation(5.0, -3.0, 2.0).inverted();
        assert_eq!(
            &t * &Tuple::new_point(-3.0, 4.0, 5.0),
            Tuple::new_point(-8.0, 7.0, 3.0)
        );
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let t = translation(5.0, -3.0, 2.0);
        assert_eq!(
            &t * &Tuple::new_vector(-3.0, 4.0, 5.0),
            Tuple::new_vector(-3.0, 4.0, 5.0)
        );
    }

    #[test]
    fn scaling_point() {
        let t = scaling(2.0, 3.0, 4.0);
        assert_eq!(
            &t * &Tuple::new_point(-4.0, 6.0, 8.0),
            Tuple::new_point(-8.0, 18.0, 32.0)
        );
    }

    #[test]
    fn scaling_vector() {
        let t = scaling(2.0, 3.0, 4.0);
        assert_eq!(
            &t * &Tuple::new_vector(-4.0, 6.0, 8.0),
            Tuple::new_vector(-8.0, 18.0, 32.0)
        );
    }

    #[test]
    fn multiplying_by_inverted_scaling_matrix() {
        let t = scaling(2.0, 3.0, 4.0).inverted();
        assert_eq!(
            &t * &Tuple::new_vector(-4.0, 6.0, 8.0),
            Tuple::new_vector(-2.0, 2.0, 2.0)
        );
    }

    #[test]
    fn reflecting_by_scaling_with_negative_value() {
        let t = scaling(-1.0, 1.0, 1.0);
        assert_eq!(
            &t * &Tuple::new_vector(2.0, 3.0, 4.0),
            Tuple::new_vector(-2.0, 3.0, 4.0)
        );
    }
}