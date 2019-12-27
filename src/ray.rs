//! `Ray` type.
//!
//! TRTC chapter 5.

use crate::tuple::Tuple;

/// An immutable ray.
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

    pub fn origin(&self) -> &Tuple {
        &self.origin
    }
    pub fn direction(&self) -> &Tuple {
        &self.direction
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_and_querying_ray() {
        let o = Tuple::new_point(1.0, 2.0, 3.0);
        let d = Tuple::new_vector(4.0, 5.0, 6.0);
        let r = Ray::new(o.clone(), d.clone());
        assert_eq!(*r.origin(), o);
        assert_eq!(*r.direction(), d);
    }
}
