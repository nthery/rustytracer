/// Ray tracing library developed with
/// https://pragprog.com/book/jbtracer/the-ray-tracer-challenge
use std::f64;

/// A 3D point (w == 1.0) or vector (w == 0.0).
#[derive(Debug)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        assert!(w == 0.0 || w == 1.0);
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
}

// Stolen from https://users.rust-lang.org/t/assert-eq-for-float-numbers/7034/4
fn nearly_equal(a: f64, b: f64) -> bool {
    let abs_a = a.abs();
    let abs_b = b.abs();
    let diff = (a - b).abs();

    if a == b {
        // Handle infinities.
        true
    } else if a == 0.0 || b == 0.0 || diff < f64::MIN_POSITIVE {
        // One of a or b is zero (or both are extremely close to it,) use absolute error.
        diff < (f64::EPSILON * f64::MIN_POSITIVE)
    } else {
        // Use relative error.
        (diff / f64::min(abs_a + abs_b, f64::MAX)) < f64::EPSILON
    }
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
}
