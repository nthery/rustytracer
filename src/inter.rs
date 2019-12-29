//! This module implements intersections between rays and objects.
//!
//! See TRTC chapter 5.

use std::cmp::Ordering;

use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::tuple::{Tuple, ORIGIN};

/// Intersection between an object and a `Ray`.
#[derive(PartialEq, Debug)]
pub struct Intersection<'a> {
    pub sphere: &'a Sphere,

    /// Distance from origin of intersecting ray.
    pub distance: f64,
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

/// Computes intersection between sphere and ray.
///
/// Returns sequence of intersections.  If there is no intersection, the sequence is empty.  If the
/// ray is tangent to the sphere, the sequence contains two identical intersections.
pub fn intersects<'a>(sphere: &'a Sphere, ray: &Ray) -> Vec<Intersection<'a>> {
    let trans_ray = ray.transformed(&sphere.transform.inverted());
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
                sphere,
            },
            Intersection {
                distance: (-b + dis_sqrt) / a2,
                sphere,
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::transform;

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let r = Ray::from_triplets((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = intersects(&s, &r);
        assert_eq!(
            xs,
            [
                Intersection {
                    sphere: &s,
                    distance: 4.0
                },
                Intersection {
                    sphere: &s,
                    distance: 6.0
                }
            ]
        );
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = Ray::from_triplets((0.0, 1.0, -5.0), (0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = intersects(&s, &r);
        assert_eq!(
            xs,
            [
                Intersection {
                    sphere: &s,
                    distance: 5.0
                },
                Intersection {
                    sphere: &s,
                    distance: 5.0
                }
            ]
        );
    }

    #[test]
    fn ray_misses_sphere() {
        let r = Ray::from_triplets((0.0, 2.0, -5.0), (0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = intersects(&s, &r);
        assert_eq!(xs, []);
    }

    #[test]
    fn ray_originates_inside_sphere() {
        let r = Ray::from_triplets((0.0, 0.0, 0.0), (0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = intersects(&s, &r);
        assert_eq!(
            xs,
            [
                Intersection {
                    sphere: &s,
                    distance: -1.0
                },
                Intersection {
                    sphere: &s,
                    distance: 1.0
                }
            ]
        );
    }

    #[test]
    fn sphere_is_behind_ray() {
        let r = Ray::from_triplets((0.0, 0.0, 5.0), (0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = intersects(&s, &r);
        assert_eq!(
            xs,
            [
                Intersection {
                    sphere: &s,
                    distance: -6.0
                },
                Intersection {
                    sphere: &s,
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
        let s = Sphere::new();
        let xs = vec![
            Intersection {
                sphere: &s,
                distance: 1.0,
            },
            Intersection {
                sphere: &s,
                distance: 2.0,
            },
        ];
        assert_eq!(*xs.hit().unwrap(), xs[0]);
    }

    #[test]
    fn hit_when_some_distances_are_negative() {
        let s = Sphere::new();
        let xs = vec![
            Intersection {
                sphere: &s,
                distance: -1.0,
            },
            Intersection {
                sphere: &s,
                distance: 1.0,
            },
        ];
        assert_eq!(*xs.hit().unwrap(), xs[1]);
    }

    #[test]
    fn hit_when_all_distances_are_negative() {
        let s = Sphere::new();
        let xs = vec![
            Intersection {
                sphere: &s,
                distance: -2.0,
            },
            Intersection {
                sphere: &s,
                distance: -1.0,
            },
        ];
        assert_eq!(xs.hit(), None);
    }

    #[test]
    fn hit_is_lowest_nonnegative_distance() {
        let s = Sphere::new();
        let xs = vec![
            Intersection {
                sphere: &s,
                distance: 5.0,
            },
            Intersection {
                sphere: &s,
                distance: 7.0,
            },
            Intersection {
                sphere: &s,
                distance: -3.0,
            },
            Intersection {
                sphere: &s,
                distance: 2.0,
            },
        ];
        assert_eq!(*xs.hit().unwrap(), xs[3]);
    }

    #[test]
    fn intersecting_scaled_sphere_with_ray() {
        let r = Ray::from_triplets((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));
        let s = Sphere::with_transform(transform::scaling(2.0, 2.0, 2.0));
        assert_eq!(
            intersects(&s, &r),
            vec![
                Intersection {
                    sphere: &s,
                    distance: 3.0,
                },
                Intersection {
                    sphere: &s,
                    distance: 7.0,
                },
            ]
        );
    }

    #[test]
    fn intersecting_translated_sphere_with_ray() {
        let r = Ray::from_triplets((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));
        let s = Sphere::with_transform(transform::translation(5.0, 0.0, 0.0));
        assert!(intersects(&s, &r).is_empty());
    }
}