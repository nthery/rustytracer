//! Light and shading related abstractions.
//!
//! See TRTC chapter 6.

use crate::color::{self, Color};
use crate::tuple::Tuple;

/// A sizeless light source.
#[derive(Debug, PartialEq)]
pub struct PointLight {
    pub intensity: Color,
    pub position: Tuple,
}

impl PointLight {
    pub fn new(intensity: Color, position: Tuple) -> PointLight {
        debug_assert!(position.is_point());
        PointLight {
            intensity,
            position,
        }
    }
}

/// Phong reflection model parameters.
#[derive(Debug, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Default for Material {
    fn default() -> Self {
        Material {
            color: color::WHITE,
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::ORIGIN;

    #[test]
    fn point_light_has_position_and_intensity() {
        let l = PointLight::new(color::WHITE, ORIGIN);
        assert_eq!(l.intensity, color::WHITE);
        assert_eq!(l.position, ORIGIN);
    }
}
