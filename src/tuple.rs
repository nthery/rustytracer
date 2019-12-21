//! Tuples, points and vectors.
//!
//! TRTC chapter 1.
//!
//! TODO: Implement Tuple as array (like Color)?

use std::f64;

/// A quadruplet that can represent a 3D point (w == 1.0) or vector (w == 0.0).
/// TODO: introduce Vector and Point types?
#[derive(Debug)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        Tuple { x, y, z, w }
    }

    pub fn new_point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple { x, y, z, w: 1.0 }
    }

    pub fn new_vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple { x, y, z, w: 0.0 }
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    pub fn nearly_equal(&self, other: &Tuple) -> bool {
        nearly_equal(self.x, other.x)
            && nearly_equal(self.y, other.y)
            && nearly_equal(self.z, other.z)
            && nearly_equal(self.w, other.w)
    }

    // Return dot product.
    pub fn dot(&self, other: &Tuple) -> f64 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z) + (self.w * other.w)
    }

    // Return cross product.
    pub fn cross(&self, o: &Tuple) -> Tuple {
        assert!(self.is_vector() && o.is_vector());
        Tuple::new_vector(
            self.y * o.z - self.z * o.y,
            self.z * o.x - self.x * o.z,
            self.x * o.y - self.y * o.x,
        )
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn normalize(&self) -> Tuple {
        let m = self.magnitude();
        Tuple::new(self.x / m, self.y / m, self.z / m, self.w / m)
    }
}

impl PartialEq for Tuple {
    fn eq(&self, o: &Tuple) -> bool {
        self.x == o.x && self.y == o.y && self.z == o.z && self.w == o.w
    }
}

impl std::ops::Neg for &Tuple {
    type Output = Tuple;
    fn neg(self) -> Tuple {
        Tuple::new(-self.x, -self.y, -self.z, -self.w)
    }
}

impl std::ops::Neg for Tuple {
    type Output = Tuple;
    fn neg(self) -> Tuple {
        -&self
    }
}

impl std::ops::Add for &Tuple {
    type Output = Tuple;
    fn add(self, o: Self) -> Tuple {
        Tuple::new(self.x + o.x, self.y + o.y, self.z + o.z, self.w + o.w)
    }
}

impl std::ops::Add for Tuple {
    type Output = Tuple;
    fn add(self, o: Self) -> Tuple {
        &self + &o
    }
}

impl std::ops::Sub for &Tuple {
    type Output = Tuple;
    fn sub(self, o: Self) -> Tuple {
        Tuple::new(self.x - o.x, self.y - o.y, self.z - o.z, self.w - o.w)
    }
}

impl std::ops::Sub for Tuple {
    type Output = Tuple;
    fn sub(self, o: Self) -> Tuple {
        &self - &o
    }
}

impl std::ops::Mul<f64> for &Tuple {
    type Output = Tuple;
    fn mul(self, o: f64) -> Self::Output {
        Tuple::new(self.x * o, self.y * o, self.z * o, self.w * o)
    }
}

impl std::ops::Mul<f64> for Tuple {
    type Output = Tuple;
    fn mul(self, o: f64) -> Self::Output {
        &self * o
    }
}

/// Return true if arguments are approximatevely equal.
/// We use the implementation from TRTC so that we can copy-paste tests easily.
///
/// This is probably not the best implementation.  See for example:
/// https://users.rust-lang.org/t/assert-eq-for-float-numbers/7034/4
fn nearly_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < 0.00001
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_with_w0_is_point() {
        let t = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert_eq!(t.x, 4.3);
        assert_eq!(t.y, -4.2);
        assert_eq!(t.z, 3.1);
        assert_eq!(t.w, 1.0);
        assert!(t.is_point());
        assert!(!t.is_vector());
    }

    #[test]
    fn tuple_with_w1_is_vector() {
        let t = Tuple::new(4.3, -4.2, 3.1, 0.0);
        assert_eq!(t.x, 4.3);
        assert_eq!(t.y, -4.2);
        assert_eq!(t.z, 3.1);
        assert_eq!(t.w, 0.0);
        assert!(!t.is_point());
        assert!(t.is_vector());
    }

    #[test]
    fn create_point() {
        let t = Tuple::new_point(4.3, -4.2, 3.1);
        assert_eq!(t.x, 4.3);
        assert_eq!(t.y, -4.2);
        assert_eq!(t.z, 3.1);
        assert_eq!(t.w, 1.0);
        assert!(t.is_point());
    }

    #[test]
    fn create_vector() {
        let t = Tuple::new_vector(4.3, -4.2, 3.1);
        assert_eq!(t.x, 4.3);
        assert_eq!(t.y, -4.2);
        assert_eq!(t.z, 3.1);
        assert_eq!(t.w, 0.0);
        assert!(t.is_vector());
    }

    #[test]
    fn tuples_with_same_values_are_equal() {
        let l = Tuple::new_point(1.1, 2.2, 3.3);
        let r = Tuple::new_point(1.1, 2.2, 3.3);
        assert!(l.nearly_equal(&r));
    }

    #[test]
    fn tuples_with_very_similar_values_are_equal() {
        let l = Tuple::new_point(1.1, 2.2, 3.3);
        let r = Tuple::new_point(1.1, 2.2 - f64::EPSILON, 3.3);
        assert!(l.nearly_equal(&r));
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
            -Tuple::new(3.0, -2.0, 1.0, 4.0),
            Tuple::new(-3.0, 2.0, -1.0, -4.0)
        );
    }

    #[test]
    fn multiplying_tuple_by_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a * 3.5, Tuple::new(3.5, -7.0, 10.5, -14.0));
    }

    #[test]
    fn multiplying_tuple_by_fraction() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a * 0.5, Tuple::new(0.5, -1.0, 1.5, -2.0));
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
            Tuple::new_vector(4.0, 0.0, 0.0).normalize(),
            Tuple::new_vector(1.0, 0.0, 0.0)
        );
        assert!(Tuple::nearly_equal(
            &Tuple::new_vector(1.0, 2.0, 3.0).normalize(),
            &Tuple::new_vector(0.26726, 0.53452, 0.80178)
        ));
    }

    #[test]
    fn magnitude_of_normalized_vector_is_one() {
        assert_eq!(
            Tuple::new_vector(1.0, 2.0, 3.0).normalize().magnitude(),
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
