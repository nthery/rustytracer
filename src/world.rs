//! World type

use crate::color::{self, Color};
use crate::light::{self, PointLight, PointStatus};
use crate::ray::Ray;
use crate::shape::{Computations, Intersection, IntersectionList, Shape};
use crate::tuple::Tuple;

/// A scene to render.
pub struct World {
    pub light: PointLight,
    pub objects: Vec<Shape>,
}

impl World {
    /// Computes color of intersection between an object of this world and `ray`.
    pub fn color_at(&self, ray: &Ray) -> Color {
        let xs = self.intersects(ray);
        if xs.is_empty() {
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
            .flat_map(|o| o.intersections(ray))
            .collect();
        xs.sort_unstable_by(|l, r| l.distance.partial_cmp(&r.distance).expect("NaN unexpected"));
        xs
    }

    /// Computes color of intersection point described by `comps`.
    fn shade_hit(&self, comps: &Computations) -> Color {
        light::lighting(
            &comps.object.material,
            &self.light,
            &comps.over_point,
            &comps.eye_vec,
            &comps.normal_vec,
            self.point_status(&comps.over_point),
        )
    }

    /// Returns whether point `pt` is in shadow.
    fn point_status(&self, pt: &Tuple) -> PointStatus {
        debug_assert!(pt.is_point());
        let vec = &self.light.position - pt;
        let ray = Ray::new(pt.clone(), vec.normalized());
        let xs = self.intersects(&ray);
        if let Some(i) = xs.hit() {
            if i.distance < vec.magnitude() {
                PointStatus::InShadow
            } else {
                PointStatus::InLight
            }
        } else {
            PointStatus::InLight
        }
    }
}

#[cfg(test)]
pub(crate) mod test_util {
    use super::*;
    use crate::light::Material;
    use crate::shape::Object;
    use crate::transform;
    use crate::tuple::Tuple;

    pub fn default_world() -> World {
        World {
            light: PointLight::new(color::WHITE, Tuple::new_point(-10.0, 10.0, -10.0)),
            objects: vec![
                Shape {
                    material: Material {
                        color: Color::new(0.8, 1.0, 0.6),
                        diffuse: 0.7,
                        specular: 0.2,
                        ..Material::default()
                    },
                    ..Shape::new(Object::Sphere)
                },
                Shape {
                    transform: transform::scaling(0.5, 0.5, 0.5),
                    ..Shape::new(Object::Sphere)
                },
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::test_util;
    use crate::shape::Object;
    use crate::transform;
    use crate::tuple::Tuple;

    #[test]
    fn intersect_world_with_ray() {
        let w = test_util::default_world();
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        assert_eq!(
            w.intersects(&r),
            [
                Intersection {
                    shape: &w.objects[0],
                    distance: 4.0
                },
                Intersection {
                    shape: &w.objects[1],
                    distance: 4.5
                },
                Intersection {
                    shape: &w.objects[1],
                    distance: 5.5
                },
                Intersection {
                    shape: &w.objects[0],
                    distance: 6.0
                },
            ]
        );
    }

    #[test]
    fn shading_intersection() {
        let w = test_util::default_world();
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let shape = &w.objects[0];
        let i = Intersection {
            distance: 4.0,
            shape,
        };
        assert_eq!(
            w.shade_hit(&i.prepare_computations(&r)),
            Color::new(0.38066, 0.47583, 0.2855)
        )
    }

    #[test]
    fn shading_intersection_from_inside() {
        let mut w = test_util::default_world();
        w.light = PointLight::new(color::WHITE, Tuple::new_point(0.0, 0.25, 0.0));
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 0.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let shape = &w.objects[1];
        let i = Intersection {
            distance: 0.5,
            shape,
        };
        assert_eq!(
            w.shade_hit(&i.prepare_computations(&r)),
            Color::new(0.90498, 0.90498, 0.90498)
        )
    }

    #[test]
    fn shade_hit_given_intersection_in_shadow() {
        let w = World {
            light: PointLight::new(color::WHITE, Tuple::new_point(0.0, 0.0, -10.0)),
            objects: vec![
                Shape::new(Object::Sphere),
                Shape {
                    transform: transform::translation(0.0, 0.0, 10.0),
                    ..Shape::new(Object::Sphere)
                },
            ],
        };

        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );

        let i = Intersection {
            distance: 4.0,
            shape: &w.objects[1],
        };

        assert_eq!(
            w.shade_hit(&i.prepare_computations(&r)),
            Color::new(0.1, 0.1, 0.1)
        );
    }

    #[test]
    fn color_when_ray_misses() {
        let w = test_util::default_world();
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 1.0, 0.0),
        );
        assert_eq!(w.color_at(&r), color::BLACK);
    }

    #[test]
    fn color_when_ray_hits() {
        let w = test_util::default_world();
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        assert_eq!(w.color_at(&r), Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn color_with_intersection_behind_ray() {
        let mut w = test_util::default_world();
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

    #[test]
    fn no_shadow_when_nothing_colinear_with_point_and_light() {
        let w = test_util::default_world();
        let p = Tuple::new_point(0.0, 10.0, 0.0);
        assert_eq!(w.point_status(&p), PointStatus::InLight);
    }

    #[test]
    fn shadow_when_object_between_point_and_light() {
        let w = test_util::default_world();
        let p = Tuple::new_point(10.0, -10.0, 10.0);
        assert_eq!(w.point_status(&p), PointStatus::InShadow);
    }

    #[test]
    fn no_shadow_when_object_behind_light() {
        let w = test_util::default_world();
        let p = Tuple::new_point(-20.0, 20.0, -20.0);
        assert_eq!(w.point_status(&p), PointStatus::InLight);
    }

    #[test]
    fn no_shadow_when_object_behind_point() {
        let w = test_util::default_world();
        let p = Tuple::new_point(-2.0, 2.0, -2.0);
        assert_eq!(w.point_status(&p), PointStatus::InLight);
    }
}
