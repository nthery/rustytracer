//! World type

use crate::color::{self, Color};
use crate::inter::{self, Computations, Intersection};
use crate::light::{self, PointLight};
use crate::ray::Ray;
use crate::sphere::Sphere;

/// A scene to render.
pub struct World {
    pub light: PointLight,
    pub objects: Vec<Sphere>,
}

impl World {
    /// Computes color of intersection between an object of this world and `ray`.
    pub fn color_at(&self, ray: &Ray) -> Color {
        let xs = self.intersects(ray);
        if xs.len() == 0 {
            color::BLACK
        } else {
            self.shade_hit(&xs[0].prepare_computations(ray))
        }
    }

    /// Computes intersections between this world and `ray`.
    fn intersects(&self, ray: &Ray) -> Vec<Intersection> {
        let mut xs: Vec<Intersection> = self
            .objects
            .iter()
            .flat_map(|o| inter::intersects(o, ray))
            .collect();
        xs.sort_unstable_by(|l, r| l.distance.partial_cmp(&r.distance).expect("NaN unexpected"));
        xs
    }

    /// Computes color of intersection point described by `comps`.
    fn shade_hit(&self, comps: &Computations) -> Color {
        light::lighting(
            &comps.object.material,
            &self.light,
            &comps.point,
            &comps.eye_vec,
            &comps.normal_vec,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::light::Material;
    use crate::transform;
    use crate::tuple::Tuple;

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

    #[test]
    fn shading_intersection() {
        let w = default_world();
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let shape = &w.objects[0];
        let i = Intersection {
            distance: 4.0,
            sphere: shape,
        };
        assert_eq!(
            w.shade_hit(&i.prepare_computations(&r)),
            Color::new(0.38066, 0.47583, 0.2855)
        )
    }

    #[test]
    fn shading_intersection_from_inside() {
        let mut w = default_world();
        w.light = PointLight::new(color::WHITE, Tuple::new_point(0.0, 0.25, 0.0));
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 0.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let shape = &w.objects[1];
        let i = Intersection {
            distance: 0.5,
            sphere: shape,
        };
        assert_eq!(
            w.shade_hit(&i.prepare_computations(&r)),
            Color::new(0.90498, 0.90498, 0.90498)
        )
    }

    #[test]
    fn color_when_ray_misses() {
        let w = default_world();
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 1.0, 0.0),
        );
        assert_eq!(w.color_at(&r), color::BLACK);
    }

    #[test]
    fn color_when_ray_hits() {
        let w = default_world();
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        assert_eq!(w.color_at(&r), Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn color_with_intersection_behind_ray() {
        let mut w = default_world();
        for o in &mut w.objects {
            o.material.ambient = 1.0;
        }
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 0.75),
            Tuple::new_vector(0.0, 0.0, -1.0),
        );
        // Intersection is on inner object.
        assert_eq!(w.color_at(&r), w.objects[0].material.color);
    }
}
