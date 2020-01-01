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

/// Computes color of point `pt` illuminated by light source `light` using Phong reflection model.
///
/// `m` contains the characteristics of the surface at point `pt`.
///
/// `eye_vec` is a vector encoding the direction from `pt` to the eye.
///
/// `normal_vec`  is the surface normal at point `pt`.
pub fn lighting(
    m: &Material,
    light: &PointLight,
    pt: &Tuple,
    eye_vec: &Tuple,
    normal_vec: &Tuple,
) -> Color {
    debug_assert!(pt.is_point());
    debug_assert!(eye_vec.is_vector());
    debug_assert!(normal_vec.is_vector());

    let effective_color = &m.color * &light.intensity;
    let light_vec = (&light.position - pt).normalized();
    let ambient = &effective_color * m.ambient;

    let mut diffuse = color::BLACK;
    let mut specular = color::BLACK;
    let light_dot_normal = Tuple::dot(&light_vec, &normal_vec);
    if light_dot_normal >= 0.0 {
        diffuse = &effective_color * (m.diffuse * light_dot_normal);
        let reflect_vec = Tuple::reflected(&-&light_vec, &normal_vec);
        let reflect_dot_eye = Tuple::dot(&reflect_vec, &eye_vec);

        if reflect_dot_eye >= 0.0 {
            let factor = reflect_dot_eye.powf(m.shininess);
            specular = &light.intensity * (m.specular * factor);
        } else {
            // The reflection does not reach the eye.
        }
    } else {
        // The light source illuminates the other side of the surface.
    }

    &(&ambient + &diffuse) + &specular
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

    #[test]
    fn lighting_with_eye_between_light_and_surface() {
        let eye_vec = Tuple::new_vector(0.0, 0.0, -1.0);
        let normal_vec = Tuple::new_vector(0.0, 0.0, -1.0);
        let light = PointLight::new(color::WHITE, Tuple::new_point(0.0, 0.0, -10.0));
        let res = lighting(&Material::default(), &light, &ORIGIN, &eye_vec, &normal_vec);
        assert_eq!(res, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_eye_offset_45() {
        let eye_vec = Tuple::new_vector(0.0, 2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0);
        let normal_vec = Tuple::new_vector(0.0, 0.0, -1.0);
        let light = PointLight::new(color::WHITE, Tuple::new_point(0.0, 0.0, -10.0));
        let res = lighting(&Material::default(), &light, &ORIGIN, &eye_vec, &normal_vec);
        assert_eq!(res, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45() {
        let eye_vec = Tuple::new_vector(0.0, 0.0, -1.0);
        let normal_vec = Tuple::new_vector(0.0, 0.0, -1.0);
        let light = PointLight::new(color::WHITE, Tuple::new_point(0.0, 10.0, -10.0));
        let res = lighting(&Material::default(), &light, &ORIGIN, &eye_vec, &normal_vec);
        assert_eq!(res, Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_with_eye_in_path_of_reflexion_vector() {
        let eye_vec = Tuple::new_vector(0.0, -2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0);
        let normal_vec = Tuple::new_vector(0.0, 0.0, -1.0);
        let light = PointLight::new(color::WHITE, Tuple::new_point(0.0, 10.0, -10.0));
        let res = lighting(&Material::default(), &light, &ORIGIN, &eye_vec, &normal_vec);
        assert_eq!(res, Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn lighting_with_light_behind_surface() {
        let eye_vec = Tuple::new_vector(0.0, 0.0, -1.0);
        let normal_vec = Tuple::new_vector(0.0, 0.0, -1.0);
        let light = PointLight::new(color::WHITE, Tuple::new_point(0.0, 0.0, 10.0));
        let res = lighting(&Material::default(), &light, &ORIGIN, &eye_vec, &normal_vec);
        assert_eq!(res, Color::new(0.1, 0.1, 0.1));
    }
}
