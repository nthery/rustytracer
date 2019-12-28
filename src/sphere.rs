//! Sphere type.
//!
//! TRTC chapter 5.

/// A unit sphere centered on the origin.
#[derive(Debug)]
pub struct Sphere {
    // The address of this field is a unique identifier of this sphere.  Its value is irrelevant.
    id: u8,
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere { id: 0 }
    }
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Sphere) -> bool {
        &self.id as *const u8 == &other.id as *const u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spheres_are_unique() {
        assert_ne!(Sphere::new(), Sphere::new());
    }
}
