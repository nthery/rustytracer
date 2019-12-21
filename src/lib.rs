/// Ray tracing library developed with
/// https://pragprog.com/book/jbtracer/the-ray-tracer-challenge

/// A 3D point (w == 1.0) or vector (w == 0.0).
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}

impl Tuple {
    pub fn new(x:f64, y:f64, z:f64, w: f64) -> Tuple {
        assert!(w == 0.0 || w == 1.0);
        Tuple{x, y, z, w}
    }

    pub fn new_point(x:f64, y:f64, z:f64) -> Tuple {
        Tuple{x, y, z, w:1.0}
    }

    pub fn new_vector(x:f64, y:f64, z:f64) -> Tuple {
        Tuple{x, y, z, w:0.0}
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
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
}

