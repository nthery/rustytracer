//! World type

use crate::inter::{self, Intersection};
use crate::light::PointLight;
use crate::ray::Ray;
use crate::sphere::Sphere;

/// A scene to render.
pub struct World {
    pub light: PointLight,
    pub objects: Vec<Sphere>,
}

impl World {
    /// Computes intersections between this world and `ray`.
    pub fn intersects(&self, ray: &Ray) -> Vec<Intersection> {
        let mut xs: Vec<Intersection> = self
            .objects
            .iter()
            .flat_map(|o| inter::intersects(o, ray))
            .collect();
        xs.sort_unstable_by(|l, r| l.distance.partial_cmp(&r.distance).expect("NaN unexpected"));
        xs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::color::Color;
    use crate::light::Material;
    use crate::transform;
    use crate::tuple::Tuple;

    use crate::color;

    fn default_world() -> World {
        World {
            light: PointLight::new(color::WHITE, Tuple::new_point(-10.0, 10.0, -10.0)),
            objects: vec![
                Sphere {
                    material: Material {
                        color: Color::new(0.8, 1.0, 0.6),
                        diffuse: 0.7,
                        specular: 0.2,
                        ..Material::default()
                    },
                    ..Sphere::default()
                },
                Sphere {
                    transform: transform::scaling(0.5, 0.5, 0.5),
                    ..Sphere::default()
                },
            ],
        }
    }

    #[test]
    fn intersect_world_with_ray() {
        let w = default_world();
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        assert_eq!(
            w.intersects(&r),
            [
                Intersection {
                    sphere: &w.objects[0],
                    distance: 4.0
                },
                Intersection {
                    sphere: &w.objects[1],
                    distance: 4.5
                },
                Intersection {
                    sphere: &w.objects[1],
                    distance: 5.5
                },
                Intersection {
                    sphere: &w.objects[0],
                    distance: 6.0
                },
            ]
        );
    }
}
