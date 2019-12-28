//! `Ray` type.
//!
//! TRTC chapter 5.

use crate::matrix::Matrix;
use crate::tuple::Tuple;

/// An immutable ray.
#[derive(Debug, PartialEq)]
pub struct Ray {
    origin: Tuple,
    direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Ray {
        debug_assert!(origin.is_point());
        debug_assert!(direction.is_vector());
        Ray { origin, direction }
    }

    /// Creates a ray from language tuples rather than `Tuple` values.
    pub fn from_triplets(origin: (f64, f64, f64), direction: (f64, f64, f64)) -> Ray {
        let (x, y, z) = origin;
        let (dx, dy, dz) = direction;
        Ray {
            origin: Tuple::new_point(x, y, z),
            direction: Tuple::new_vector(dx, dy, dz),
        }
    }

    pub fn origin(&self) -> &Tuple {
        &self.origin
    }

    pub fn direction(&self) -> &Tuple {
        &self.direction
    }

    /// Returns coordinates of point on this ray at distance `t` from origin.
    pub fn position(&self, t: f64) -> Tuple {
        &self.origin + &(&self.direction * t)
    }

    /// Apply transformation encoded in `m` to this ray and return resulting ray.
    pub fn transformed(&self, m: &Matrix) -> Ray {
        let o = m * &self.origin;
        let d = m * &self.direction;
        Ray::new(o, d)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::transform;

    #[test]
    fn creating_ray_from_point_and_vector() {
        let o = Tuple::new_point(1.0, 2.0, 3.0);
        let d = Tuple::new_vector(4.0, 5.0, 6.0);
        let r = Ray::new(o.clone(), d.clone());
        assert_eq!(*r.origin(), o);
        assert_eq!(*r.direction(), d);
    }

    #[test]
    fn creating_ray_from_triplets() {
        let r = Ray::from_triplets((1.0, 2.0, 3.0), (4.0, 5.0, 6.0));
        assert_eq!(*r.origin(), Tuple::new_point(1.0, 2.0, 3.0));
        assert_eq!(*r.direction(), Tuple::new_vector(4.0, 5.0, 6.0));
    }

    #[test]
    fn computing_point_from_distance() {
        let r = Ray::new(
            Tuple::new_point(2.0, 3.0, 4.0),
            Tuple::new_vector(1.0, 0.0, 0.0),
        );

        assert_eq!(r.position(0.0), Tuple::new_point(2.0, 3.0, 4.0));
        assert_eq!(r.position(1.0), Tuple::new_point(3.0, 3.0, 4.0));
        assert_eq!(r.position(-1.0), Tuple::new_point(1.0, 3.0, 4.0));
        assert_eq!(r.position(2.5), Tuple::new_point(4.5, 3.0, 4.0));
    }

    #[test]
    fn translating_ray() {
        let r = Ray::from_triplets((1.0, 2.0, 3.0), (0.0, 1.0, 0.0));
        let tr = r.transformed(&transform::translation(3.0, 4.0, 5.0));
        assert_eq!(tr, Ray::from_triplets((4.0, 6.0, 8.0), (0.0, 1.0, 0.0)));
    }

    #[test]
    fn scaling_ray() {
        let r = Ray::from_triplets((1.0, 2.0, 3.0), (0.0, 1.0, 0.0));
        let tr = r.transformed(&transform::scaling(2.0, 3.0, 4.0));
        assert_eq!(tr, Ray::from_triplets((2.0, 6.0, 12.0), (0.0, 3.0, 0.0)));
    }
}
