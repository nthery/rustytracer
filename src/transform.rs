//! This module implements matrix transformations.
//!
//! See TRTC chapter 4.
//!
//! TODO: Multiplying more than 2 matrices is ugly becuse of explicit references.
//! TODO: Is it possible to implement expression templates?

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

/// Returns matrix encoding rotation of `angle` radiants arond the x-axis.
pub fn rotation_x(angle: f64) -> Matrix {
    let mut res = Matrix::new_4x4_identity();
    res.set(1, 1, angle.cos());
    res.set(1, 2, -angle.sin());
    res.set(2, 1, angle.sin());
    res.set(2, 2, angle.cos());
    res
}

/// Returns matrix encoding rotation of `angle` radiants arond the y-axis.
pub fn rotation_y(angle: f64) -> Matrix {
    let mut res = Matrix::new_4x4_identity();
    res.set(0, 0, angle.cos());
    res.set(0, 2, angle.sin());
    res.set(2, 0, -angle.sin());
    res.set(2, 2, angle.cos());
    res
}

/// Returns matrix encoding rotation of `angle` radiants arond the z-axis.
pub fn rotation_z(angle: f64) -> Matrix {
    let mut res = Matrix::new_4x4_identity();
    res.set(0, 0, angle.cos());
    res.set(0, 1, -angle.sin());
    res.set(1, 0, angle.sin());
    res.set(1, 1, angle.cos());
    res
}

/// Returns matrix encoding shearing effect.
pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix {
    let mut res = Matrix::new_4x4_identity();
    res.set(0, 1, xy);
    res.set(0, 2, xz);
    res.set(1, 0, yx);
    res.set(1, 2, yz);
    res.set(2, 0, zx);
    res.set(2, 1, zy);
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::Tuple;
    use std::f64::consts::PI;

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

    #[test]
    fn rotating_around_x_axis() {
        let p = Tuple::new_point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        let full_quarter = rotation_x(PI / 2.0);
        assert_eq!(
            &half_quarter * &p,
            Tuple::new_point(0.0, 2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0)
        );
        assert_eq!(&full_quarter * &p, Tuple::new_point(0.0, 0.0, 1.0));
    }

    #[test]
    fn inverting_x_rotation_rotates_in_opposite_direction() {
        let p = Tuple::new_point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        assert_eq!(
            &half_quarter.inverted() * &p,
            Tuple::new_point(0.0, 2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0)
        );
    }

    #[test]
    fn rotating_around_y_axis() {
        let p = Tuple::new_point(0.0, 0.0, 1.0);
        let half_quarter = rotation_y(PI / 4.0);
        let full_quarter = rotation_y(PI / 2.0);
        assert_eq!(
            &half_quarter * &p,
            Tuple::new_point(2_f64.sqrt() / 2.0, 0.0, 2_f64.sqrt() / 2.0)
        );
        assert_eq!(&full_quarter * &p, Tuple::new_point(1.0, 0.0, 0.0));
    }

    #[test]
    fn rotating_around_z_axis() {
        let p = Tuple::new_point(0.0, 1.0, 0.0);
        let half_quarter = rotation_z(PI / 4.0);
        let full_quarter = rotation_z(PI / 2.0);
        assert_eq!(
            &half_quarter * &p,
            Tuple::new_point(-2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0, 0.0)
        );
        assert_eq!(&full_quarter * &p, Tuple::new_point(-1.0, 0.0, 0.0));
    }

    #[test]
    fn shearing_moves_x_in_proportion_to_y() {
        let t = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::new_point(2.0, 3.0, 4.0);
        assert_eq!(&t * &p, Tuple::new_point(5.0, 3.0, 4.0));
    }

    #[test]
    fn shearing_moves_x_in_proportion_to_z() {
        let t = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::new_point(2.0, 3.0, 4.0);
        assert_eq!(&t * &p, Tuple::new_point(6.0, 3.0, 4.0));
    }

    #[test]
    fn shearing_moves_y_in_proportion_to_x() {
        let t = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Tuple::new_point(2.0, 3.0, 4.0);
        assert_eq!(&t * &p, Tuple::new_point(2.0, 5.0, 4.0));
    }

    #[test]
    fn shearing_moves_y_in_proportion_to_z() {
        let t = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Tuple::new_point(2.0, 3.0, 4.0);
        assert_eq!(&t * &p, Tuple::new_point(2.0, 7.0, 4.0));
    }

    #[test]
    fn shearing_moves_z_in_proportion_to_x() {
        let t = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Tuple::new_point(2.0, 3.0, 4.0);
        assert_eq!(&t * &p, Tuple::new_point(2.0, 3.0, 6.0));
    }

    #[test]
    fn shearing_moves_z_in_proportion_to_y() {
        let t = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Tuple::new_point(2.0, 3.0, 4.0);
        assert_eq!(&t * &p, Tuple::new_point(2.0, 3.0, 7.0));
    }

    #[test]
    fn individual_transforms_are_applied_in_sequence() {
        let p = Tuple::new_point(1.0, 0.0, 1.0);
        let rot = rotation_x(PI / 2.0);
        let scal = scaling(5.0, 5.0, 5.0);
        let trans = translation(10.0, 5.0, 7.0);

        let p2 = &rot * &p;
        assert_eq!(p2, Tuple::new_point(1.0, -1.0, 0.0));

        let p3 = &scal * &p2;
        assert_eq!(p3, Tuple::new_point(5.0, -5.0, 0.0));

        let p4 = &trans * &p3;
        assert_eq!(p4, Tuple::new_point(15.0, 0.0, 7.0));
    }

    #[test]
    fn chained_transforms_are_applied_in_reverse_order() {
        let p = Tuple::new_point(1.0, 0.0, 1.0);
        let rot = rotation_x(PI / 2.0);
        let scal = scaling(5.0, 5.0, 5.0);
        let trans = translation(10.0, 5.0, 7.0);

        let t = &(&trans * &scal) * &rot;
        assert_eq!(&t * &p, Tuple::new_point(15.0, 0.0, 7.0));
    }
}
