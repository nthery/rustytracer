//! Tuples, points and vectors.
//!
//! TRTC chapter 1.
//!
//! TODO: Use iterator where possible rather than hand-coded loops

use crate::util;
use std::f64;

/// A point at the origin of the coordinate system.
pub const ORIGIN: Tuple = Tuple {
    xyzw: [0.0, 0.0, 0.0, 1.0],
};

/// A quadruplet that can represent a 3D point (w == 1.0) or vector (w == 0.0).
/// TODO: introduce Vector and Point types?
/// TODO: Use builtin tuples instead?
#[derive(Clone, Debug)]
pub struct Tuple {
    xyzw: [f64; 4],
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        Tuple { xyzw: [x, y, z, w] }
    }

    pub fn new_point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple {
            xyzw: [x, y, z, 1.0],
        }
    }

    pub fn new_vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple {
            xyzw: [x, y, z, 0.0],
        }
    }

    pub fn new_zero() -> Tuple {
        Tuple { xyzw: [0.0; 4] }
    }

    pub fn is_point(&self) -> bool {
        // cast to avoid clippy warning
        self.w() as i32 == 1
    }

    pub fn is_vector(&self) -> bool {
        // cast to avoid clippy warning
        self.w() as i32 == 0
    }

    pub fn x(&self) -> f64 {
        self.xyzw[0]
    }

    pub fn y(&self) -> f64 {
        self.xyzw[1]
    }

    pub fn z(&self) -> f64 {
        self.xyzw[2]
    }

    pub fn w(&self) -> f64 {
        self.xyzw[3]
    }

    pub fn get(&self, i: usize) -> f64 {
        self.xyzw[i]
    }

    pub fn set(&mut self, i: usize, v: f64) {
        self.xyzw[i] = v;
    }

    // Return dot product.
    pub fn dot(&self, other: &Tuple) -> f64 {
        let mut res = 0.0;
        for i in 0..4 {
            res += self.xyzw[i] * other.xyzw[i];
        }
        res
    }

    // Return cross product.
    pub fn cross(&self, o: &Tuple) -> Tuple {
        assert!(self.is_vector() && o.is_vector());
        Tuple::new_vector(
            self.y() * o.z() - self.z() * o.y(),
            self.z() * o.x() - self.x() * o.z(),
            self.x() * o.y() - self.y() * o.x(),
        )
    }

    pub fn magnitude(&self) -> f64 {
        let mut res = 0.0;
        for i in 0..4 {
            res += self.xyzw[i] * self.xyzw[i];
        }
        res.sqrt()
    }

    pub fn normalized(&self) -> Tuple {
        let m = self.magnitude();
        let mut res = (*self).clone();
        for i in 0..4 {
            res.xyzw[i] /= m;
        }
        res
    }
}

impl PartialEq for Tuple {
    /// Return true if arguments are approximately equal.
    fn eq(&self, o: &Tuple) -> bool {
        for i in 0..4 {
            if !util::nearly_equal(self.xyzw[i], o.xyzw[i]) {
                return false;
            }
        }
        true
    }
}

impl std::ops::Neg for &Tuple {
    type Output = Tuple;
    fn neg(self) -> Self::Output {
        Tuple::new(-self.x(), -self.y(), -self.z(), -self.w())
    }
}

impl std::ops::Add for &Tuple {
    type Output = Tuple;
    fn add(self, o: Self) -> Self::Output {
        Tuple::new(
            self.x() + o.x(),
            self.y() + o.y(),
            self.z() + o.z(),
            self.w() + o.w(),
        )
    }
}

impl std::ops::Sub for &Tuple {
    type Output = Tuple;
    fn sub(self, o: Self) -> Tuple {
        Tuple::new(
            self.x() - o.x(),
            self.y() - o.y(),
            self.z() - o.z(),
            self.w() - o.w(),
        )
    }
}

impl std::ops::Mul<f64> for &Tuple {
    type Output = Tuple;
    fn mul(self, o: f64) -> Self::Output {
        Tuple::new(self.x() * o, self.y() * o, self.z() * o, self.w() * o)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_with_w0_is_point() {
        let t = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert_eq!(t.x(), 4.3);
        assert_eq!(t.y(), -4.2);
        assert_eq!(t.z(), 3.1);
        assert_eq!(t.w(), 1.0);
        assert!(t.is_point());
        assert!(!t.is_vector());
    }

    #[test]
    fn tuple_with_w1_is_vector() {
        let t = Tuple::new(4.3, -4.2, 3.1, 0.0);
        assert_eq!(t.x(), 4.3);
        assert_eq!(t.y(), -4.2);
        assert_eq!(t.z(), 3.1);
        assert_eq!(t.w(), 0.0);
        assert!(!t.is_point());
        assert!(t.is_vector());
    }

    #[test]
    fn create_point() {
        let t = Tuple::new_point(4.3, -4.2, 3.1);
        assert_eq!(t.x(), 4.3);
        assert_eq!(t.y(), -4.2);
        assert_eq!(t.z(), 3.1);
        assert_eq!(t.w(), 1.0);
        assert!(t.is_point());
    }

    #[test]
    fn create_vector() {
        let t = Tuple::new_vector(4.3, -4.2, 3.1);
        assert_eq!(t.x(), 4.3);
        assert_eq!(t.y(), -4.2);
        assert_eq!(t.z(), 3.1);
        assert_eq!(t.w(), 0.0);
        assert!(t.is_vector());
    }

    #[test]
    fn set_and_get_tuple_values() {
        let mut t = Tuple::new(1.0, 2.0, 3.0, 4.0);
        t.set(0, 42.0);
        t.set(3, 24.0);
        assert_eq!(t.get(0), 42.0);
        assert_eq!(t.get(3), 24.0);
    }

    #[test]
    fn tuples_with_same_values_are_equal() {
        let l = Tuple::new_point(1.1, 2.2, 3.3);
        let r = Tuple::new_point(1.1, 2.2, 3.3);
        assert_eq!(l, r);
    }

    #[test]
    fn tuples_with_very_similar_values_are_equal() {
        let l = Tuple::new_point(1.1, 2.2, 3.3);
        let r = Tuple::new_point(1.1, 2.2 - f64::EPSILON, 3.3);
        assert_eq!(l, r);
    }

    #[test]
    fn add_tuples() {
        let l = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let r = Tuple::new(-2.0, 3.0, 1.0, 0.0);
        let sum = &l + &r;
        assert_eq!(sum, Tuple::new(1.0, 1.0, 6.0, 1.0));
    }

    #[test]
    fn subtracting_two_points_gives_a_vector() {
        let l = Tuple::new_point(3.0, 2.0, 1.0);
        let r = Tuple::new_point(5.0, 6.0, 7.0);
        let diff = &l - &r;
        assert_eq!(diff, Tuple::new_vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_a_vector_from_a_point_gives_a_point() {
        let l = Tuple::new_point(3.0, 2.0, 1.0);
        let r = Tuple::new_vector(5.0, 6.0, 7.0);
        let diff = &l - &r;
        assert_eq!(diff, Tuple::new_point(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_two_vectors_gives_a_vector() {
        let l = Tuple::new_vector(3.0, 2.0, 1.0);
        let r = Tuple::new_vector(5.0, 6.0, 7.0);
        let diff = &l - &r;
        assert_eq!(diff, Tuple::new_vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn negating_a_tuple() {
        assert_eq!(
            -&Tuple::new(3.0, -2.0, 1.0, 4.0),
            Tuple::new(-3.0, 2.0, -1.0, -4.0)
        );
    }

    #[test]
    fn multiplying_tuple_by_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(&a * 3.5, Tuple::new(3.5, -7.0, 10.5, -14.0));
    }

    #[test]
    fn multiplying_tuple_by_fraction() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(&a * 0.5, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn magnitude() {
        assert_eq!(Tuple::new_vector(0.0, 1.0, 0.0).magnitude(), 1.0);
        assert_eq!(Tuple::new_vector(0.0, 0.0, 1.0).magnitude(), 1.0);
        assert_eq!(Tuple::new_vector(1.0, 2.0, 3.0).magnitude(), 14_f64.sqrt());
        assert_eq!(
            Tuple::new_vector(-1.0, -2.0, -3.0).magnitude(),
            14_f64.sqrt()
        );
    }

    #[test]
    fn normalizing_vector() {
        assert_eq!(
            Tuple::new_vector(4.0, 0.0, 0.0).normalized(),
            Tuple::new_vector(1.0, 0.0, 0.0)
        );
        assert_eq!(
            Tuple::new_vector(1.0, 2.0, 3.0).normalized(),
            Tuple::new_vector(0.26726, 0.53452, 0.80178)
        );
    }

    #[test]
    fn magnitude_of_normalized_vector_is_one() {
        assert_eq!(
            Tuple::new_vector(1.0, 2.0, 3.0).normalized().magnitude(),
            1.0
        );
    }

    #[test]
    fn dot_product() {
        assert_eq!(
            Tuple::dot(
                &Tuple::new_vector(1.0, 2.0, 3.0),
                &Tuple::new_vector(2.0, 3.0, 4.0)
            ),
            20.0
        );
    }

    #[test]
    fn cross_product() {
        let l = Tuple::new_vector(1.0, 2.0, 3.0);
        let r = Tuple::new_vector(2.0, 3.0, 4.0);
        assert_eq!(Tuple::cross(&l, &r), Tuple::new_vector(-1.0, 2.0, -1.0));
        assert_eq!(Tuple::cross(&r, &l), Tuple::new_vector(1.0, -2.0, 1.0));
    }
}
