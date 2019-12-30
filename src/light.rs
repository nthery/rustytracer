//! Light-related abstractions.
//!
//! See TRTC chapter 6.

use crate::color::Color;
use crate::tuple::Tuple;

/// A sizeless light source.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::WHITE;
    use crate::tuple::ORIGIN;

    #[test]
    fn point_light_has_position_and_intensity() {
        let l = PointLight::new(WHITE, ORIGIN);
        assert_eq!(l.intensity, WHITE);
        assert_eq!(l.position, ORIGIN);
    }
}
