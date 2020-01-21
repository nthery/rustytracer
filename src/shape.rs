//! Shape type.
//!
//! TRTC chapters 5 (sphere) and 9 (abstract shape).

use std::cmp::Ordering;

use crate::light::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::tuple::{Tuple, ORIGIN};
use crate::util;

/// An abstract object that can be rendered in the world.
///
/// This structure stores bits common to all objects. The object-specific bits
/// are in `Object`.
#[derive(PartialEq, Debug)]
pub struct Shape {
    pub transform: Matrix,
    pub material: Material,
    pub object: Object,
}

impl Shape {
    pub fn new(object: Object) -> Shape {
        Shape {
            transform: Matrix::new_4x4_identity(),
            material: Material::default(),
            object,
        }
    }

    /// Creates a new shape with the specified transformation.
    pub fn with_transform(object: Object, t: Matrix) -> Shape {
        Shape {
            transform: t,
            material: Material::default(),
            object,
        }
    }

    /// Creates a new shape with the specified transformation and material.
    pub fn with_transform_and_material(
        object: Object,
        transform: Matrix,
        material: Material,
    ) -> Shape {
        Shape {
            transform,
            material,
            object,
        }
    }

    /// Computes normal vector on this shape at point `p` in world space.
    pub fn normal_at(&self, world_pt: &Tuple) -> Tuple {
        debug_assert!(world_pt.is_point());

        let it = self.transform.inverted();
        let obj_pt = &it * world_pt;
        let obj_normal = self.object.normal_at(&obj_pt);
        let mut world_normal = &it.transposed() * &obj_normal;
        world_normal.set(3, 0.0);
        world_normal.normalized()
    }

    /// Computes intersection between this shape and `ray`.
    ///
    /// Returns sequence of intersections.  If there is no intersection, the sequence is empty.  If the
    /// ray is tangent to the shape, the sequence contains two identical intersections.
    pub fn intersections<'a, 'b>(&'a self, ray: &'b Ray) -> Vec<Intersection<'a>> {
        let trans_ray = ray.transformed(&self.transform.inverted());
        self.object.intersections(self, trans_ray)
    }
}

/// `Shape` helper storing object-specific bits.
///
/// An alternative design would be to store in `Shape` a box to a trait object
/// storing the object-specific bits. This would be more extensible but slightly
/// more complicated to implement and less efficient.
#[derive(PartialEq, Debug)]
pub enum Object {
    Sphere,
}

impl Object {
    /// See Shape::intersections()
    fn intersections<'a>(&self, shape: &'a Shape, trans_ray: Ray) -> Vec<Intersection<'a>> {
        match self {
            Self::Sphere => {
                let sphere_to_ray = trans_ray.origin() - &ORIGIN;
                let a = Tuple::dot(trans_ray.direction(), trans_ray.direction());
                let b = 2.0 * Tuple::dot(trans_ray.direction(), &sphere_to_ray);
                let c = Tuple::dot(&sphere_to_ray, &sphere_to_ray) - 1.0;
                let discriminant = b * b - 4.0 * a * c;
                if discriminant < 0.0 {
                    Vec::new()
                } else {
                    let dis_sqrt = discriminant.sqrt();
                    let a2 = 2.0 * a;
                    vec![
                        Intersection {
                            distance: (-b - dis_sqrt) / a2,
                            shape,
                        },
                        Intersection {
                            distance: (-b + dis_sqrt) / a2,
                            shape,
                        },
                    ]
                }
            }
        }
    }

    /// See Shape::normal_at()
    fn normal_at(&self, pt: &Tuple) -> Tuple {
        debug_assert!(pt.is_point());
        match self {
            Object::Sphere => pt - &ORIGIN,
        }
    }
}

/// Intersection between a `Shape` and a `Ray`.
#[derive(PartialEq, Debug)]
pub struct Intersection<'a> {
    /// Object being intersected.
    pub shape: &'a Shape,

    /// Distance from origin of intersecting ray.
    pub distance: f64,
}

impl Intersection<'_> {
    /// Precomputes data used to compute lighting and shading.
    ///
    /// `ray` is cast from the eye to this intersection point.
    pub fn prepare_computations(&self, ray: &Ray) -> Computations {
        debug_assert!(ray.direction().is_vector());
        let point = ray.position(self.distance);
        let mut normal_vec = self.shape.normal_at(&point);
        let eye_vec = -ray.direction();
        let mut inside = false;
        if Tuple::dot(&normal_vec, &eye_vec) < 0.0 {
            inside = true;
            normal_vec = -&normal_vec;
        }
        let over_point = &point + &(&normal_vec * util::EPSILON);
        Computations {
            distance: self.distance,
            object: self.shape,
            normal_vec,
            point,
            over_point,
            eye_vec,
            inside,
        }
    }
}

/// Additional data about an `Intersection` used to compute lighting and shading.
#[derive(PartialEq, Debug)]
pub struct Computations<'a> {
    /// `Intersection::distance` copy.
    pub distance: f64,

    /// `Intersection::shape` copy.
    pub object: &'a Shape,

    /// Intersection point.
    pub point: Tuple,

    /// Point slightly over intersection point.
    pub over_point: Tuple,

    /// Vector from intersection point to eye.
    pub eye_vec: Tuple,

    /// Surface normal vector at intersection point.
    pub normal_vec: Tuple,

    /// The intersection is inside `object`.
    pub inside: bool,
}

/// Behavior associated with a sequence of `Intersection`.
pub trait IntersectionList {
    /// Returns intersection with the smallest non-negative distance.
    fn hit(&self) -> Option<&Intersection>;
}

impl IntersectionList for Vec<Intersection<'_>> {
    fn hit(&self) -> Option<&Intersection> {
        self.iter().filter(|i| i.distance >= 0.0).min_by(|l, r| {
            if l.distance < r.distance {
                Ordering::Less
            } else {
                Ordering::Greater // or Ordering::Equal
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transform;
    use std::f64::consts::PI;

    #[test]
    fn sphere_default_transformation_is_identity() {
        assert_eq!(
            Shape::new(Object::Sphere).transform,
            Matrix::new_4x4_identity()
        );
    }

    #[test]
    fn creating_sphere_with_non_default_transformation() {
        let t = transform::scaling(1.0, 2.0, 3.0);
        let s = Shape::with_transform(Object::Sphere, t.clone());
        assert_eq!(s.transform, t);
    }

    #[test]
    fn changing_sphere_transformation() {
        let mut s = Shape::new(Object::Sphere);
        let t = transform::translation(2.0, 3.0, 4.0);
        s.transform = t.clone();
        assert_eq!(s.transform, t);
    }

    #[test]
    fn normal_at_point_on_x_axis() {
        assert_eq!(
            Shape::new(Object::Sphere).normal_at(&Tuple::new_point(1.0, 0.0, 0.0)),
            Tuple::new_vector(1.0, 0.0, 0.0)
        );
    }

    #[test]
    fn normal_at_point_on_y_axis() {
        assert_eq!(
            Shape::new(Object::Sphere).normal_at(&Tuple::new_point(0.0, 1.0, 0.0)),
            Tuple::new_vector(0.0, 1.0, 0.0)
        );
    }

    #[test]
    fn normal_at_point_on_z_axis() {
        assert_eq!(
            Shape::new(Object::Sphere).normal_at(&Tuple::new_point(0.0, 0.0, 1.0)),
            Tuple::new_vector(0.0, 0.0, 1.0)
        );
    }

    #[test]
    fn normal_at_non_axial_point() {
        let z = 3_f64.sqrt() / 3.0;
        assert_eq!(
            Shape::new(Object::Sphere).normal_at(&Tuple::new_point(z, z, z)),
            Tuple::new_vector(z, z, z)
        );
    }

    #[test]
    fn normal_is_normalized() {
        let z = 3_f64.sqrt() / 3.0;
        let n = Shape::new(Object::Sphere).normal_at(&Tuple::new_point(z, z, z));
        assert_eq!(n, n.normalized());
    }

    #[test]
    fn normal_on_translated_sphere() {
        let s = Shape::with_transform(Object::Sphere, transform::translation(0.0, 1.0, 0.0));
        assert_eq!(
            s.normal_at(&Tuple::new_point(0.0, 1.70711, -0.70711)),
            Tuple::new_vector(0.0, 0.70711, -0.70711)
        );
    }

    #[test]
    fn normal_on_transformed_sphere() {
        let s = Shape::with_transform(
            Object::Sphere,
            &transform::scaling(1.0, 0.5, 1.0) * &transform::rotation_z(PI / 5.0),
        );
        assert_eq!(
            s.normal_at(&Tuple::new_point(
                0.0,
                2_f64.sqrt() / 2.0,
                -2_f64.sqrt() / 2.0
            )),
            Tuple::new_vector(0.0, 0.97014, -0.24254)
        );
    }

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let r = Ray::from_triplets((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));
        let s = Shape::new(Object::Sphere);
        let xs = s.intersections(&r);
        assert_eq!(
            xs,
            [
                Intersection {
                    shape: &s,
                    distance: 4.0
                },
                Intersection {
                    shape: &s,
                    distance: 6.0
                }
            ]
        );
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = Ray::from_triplets((0.0, 1.0, -5.0), (0.0, 0.0, 1.0));
        let s = Shape::new(Object::Sphere);
        let xs = s.intersections(&r);
        assert_eq!(
            xs,
            [
                Intersection {
                    shape: &s,
                    distance: 5.0
                },
                Intersection {
                    shape: &s,
                    distance: 5.0
                }
            ]
        );
    }

    #[test]
    fn ray_misses_sphere() {
        let r = Ray::from_triplets((0.0, 2.0, -5.0), (0.0, 0.0, 1.0));
        let s = Shape::new(Object::Sphere);
        let xs = s.intersections(&r);
        assert_eq!(xs, []);
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = Ray::from_triplets((0.0, 0.0, 0.0), (0.0, 0.0, 1.0));
        let s = Shape::new(Object::Sphere);
        let xs = s.intersections(&r);
        assert_eq!(
            xs,
            [
                Intersection {
                    shape: &s,
                    distance: -1.0
                },
                Intersection {
                    shape: &s,
                    distance: 1.0
                }
            ]
        );
    }

    #[test]
    fn sphere_is_behind_ray() {
        let r = Ray::from_triplets((0.0, 0.0, 5.0), (0.0, 0.0, 1.0));
        let s = Shape::new(Object::Sphere);
        let xs = s.intersections(&r);
        assert_eq!(
            xs,
            [
                Intersection {
                    shape: &s,
                    distance: -6.0
                },
                Intersection {
                    shape: &s,
                    distance: -4.0
                }
            ]
        );
    }

    #[test]
    fn no_hit_when_no_intersections() {
        let xs = Vec::<Intersection>::new();
        assert_eq!(xs.hit(), None);
    }

    #[test]
    fn hit_when_all_distances_are_positive() {
        let s = Shape::new(Object::Sphere);
        let xs = vec![
            Intersection {
                shape: &s,
                distance: 1.0,
            },
            Intersection {
                shape: &s,
                distance: 2.0,
            },
        ];
        assert_eq!(*xs.hit().unwrap(), xs[0]);
    }

    #[test]
    fn hit_when_some_distances_are_negative() {
        let s = Shape::new(Object::Sphere);
        let xs = vec![
            Intersection {
                shape: &s,
                distance: -1.0,
            },
            Intersection {
                shape: &s,
                distance: 1.0,
            },
        ];
        assert_eq!(*xs.hit().unwrap(), xs[1]);
    }

    #[test]
    fn hit_when_all_distances_are_negative() {
        let s = Shape::new(Object::Sphere);
        let xs = vec![
            Intersection {
                shape: &s,
                distance: -2.0,
            },
            Intersection {
                shape: &s,
                distance: -1.0,
            },
        ];
        assert_eq!(xs.hit(), None);
    }

    #[test]
    fn hit_is_lowest_nonnegative_distance() {
        let s = Shape::new(Object::Sphere);
        let xs = vec![
            Intersection {
                shape: &s,
                distance: 5.0,
            },
            Intersection {
                shape: &s,
                distance: 7.0,
            },
            Intersection {
                shape: &s,
                distance: -3.0,
            },
            Intersection {
                shape: &s,
                distance: 2.0,
            },
        ];
        assert_eq!(*xs.hit().unwrap(), xs[3]);
    }

    #[test]
    fn intersecting_scaled_sphere_with_ray() {
        let r = Ray::from_triplets((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));
        let s = Shape::with_transform(Object::Sphere, transform::scaling(2.0, 2.0, 2.0));
        assert_eq!(
            s.intersections(&r),
            vec![
                Intersection {
                    shape: &s,
                    distance: 3.0,
                },
                Intersection {
                    shape: &s,
                    distance: 7.0,
                },
            ]
        );
    }

    #[test]
    fn intersecting_translated_sphere_with_ray() {
        let r = Ray::from_triplets((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));
        let s = Shape::with_transform(Object::Sphere, transform::translation(5.0, 0.0, 0.0));
        assert!(s.intersections(&r).is_empty());
    }

    #[test]
    fn precomputing_intersection_state() {
        let r = Ray::from_triplets((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));
        let s = Shape::new(Object::Sphere);
        let i = Intersection {
            distance: 4.0,
            shape: &s,
        };
        assert_eq!(
            i.prepare_computations(&r),
            Computations {
                distance: 4.0,
                object: &s,
                point: Tuple::new_point(0.0, 0.0, -1.0),
                over_point: Tuple::new_point(0.0, 0.0, -1.00001),
                eye_vec: Tuple::new_vector(0.0, 0.0, -1.0),
                normal_vec: Tuple::new_vector(0.0, 0.0, -1.0),
                inside: false,
            }
        );
    }

    #[test]
    fn precomputing_when_intersection_is_inside_object() {
        let r = Ray::from_triplets((0.0, 0.0, 0.0), (0.0, 0.0, 1.0));
        let s = Shape::new(Object::Sphere);
        let i = Intersection {
            distance: 1.0,
            shape: &s,
        };
        assert_eq!(
            i.prepare_computations(&r),
            Computations {
                distance: 1.0,
                object: &s,
                point: Tuple::new_point(0.0, 0.0, 1.0),
                over_point: Tuple::new_point(0.0, 0.0, 0.99999),
                eye_vec: Tuple::new_vector(0.0, 0.0, -1.0),
                normal_vec: Tuple::new_vector(0.0, 0.0, -1.0),
                inside: true,
            }
        );
    }

    #[test]
    fn hit_should_offset_point() {
        let r = Ray::from_triplets((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));
        let s = Shape {
            transform: transform::translation(0.0, 0.0, 1.0),
            ..Shape::new(Object::Sphere)
        };
        let i = Intersection {
            distance: 5.0,
            shape: &s,
        };
        let comps = i.prepare_computations(&r);
        assert!(comps.over_point.z() < -util::EPSILON / 2.0);
        assert!(comps.point.z() > comps.over_point.z());
    }
}
