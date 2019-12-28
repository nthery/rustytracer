//! Sphere type.
//!
//! TRTC chapter 5.

use std::rc::Rc;

/// Per-sphere data.
#[derive(Debug)]
struct SphereRep;

/// A unit sphere centered on the origin.
///
/// Spheres have identity so a `Sphere` value is a handle to a hidden representation.  Cloning a
/// `Sphere` value creates a new handle to the same underlying representation.
#[derive(Clone, Debug)]
pub struct Sphere {
    // Add a layer of indirection to implement identity.
    // TODO: Store reference instead?
    rep: Rc<Box<SphereRep>>,
}

impl Sphere {
    /// Creates a new unique sphere.
    pub fn new() -> Sphere {
        Sphere {
            rep: Rc::new(Box::new(SphereRep)),
        }
    }
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Sphere) -> bool {
        Rc::ptr_eq(&self.rep, &other.rep)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_unique_spheres() {
        assert_ne!(Sphere::new(), Sphere::new());

        let s = Sphere::new();
        assert_eq!(s, s);
    }

    #[test]
    fn sphere_is_clonable() {
        let s = Sphere::new();
        assert_eq!(s.clone(), s.clone());
    }
}
