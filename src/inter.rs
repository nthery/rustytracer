//! This module implements intersections between rays and objects.
//!
//! See TRTC chapter 5.

use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::tuple::{Tuple, ORIGIN};

/// Computes intersection between sphere and ray.
///
/// Returns sequence containing intersection points encoded as distance from ray origin.
/// If there is no intersection, the sequence is empty.
/// If the ray is tangent to the sphere, the sequence contains two identical points.
pub fn intersects(_sphere: &Sphere, ray: &Ray) -> Vec<f64> {
    let sphere_to_ray = ray.origin() - &ORIGIN;
    let a = Tuple::dot(ray.direction(), ray.direction());
    let b = 2.0 * Tuple::dot(ray.direction(), &sphere_to_ray);
    let c = Tuple::dot(&sphere_to_ray, &sphere_to_ray) - 1.0;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        Vec::new()
    } else {
        let dis_sqrt = discriminant.sqrt();
        let a2 = 2.0 * a;
        vec![(-b - dis_sqrt) / a2, (-b + dis_sqrt) / a2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let r = Ray::from_triplets((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = intersects(&s, &r);
        assert_eq!(xs, [4.0, 6.0]);
    }

    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = Ray::from_triplets((0.0, 1.0, -5.0), (0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = intersects(&s, &r);
        assert_eq!(xs, [5.0, 5.0]);
    }

    #[test]
    fn ray_misses_sphere() {
        let r = Ray::from_triplets((0.0, 2.0, -5.0), (0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = intersects(&s, &r);
        assert_eq!(xs, []);
    }

    #[test]
    fn ray_originates_inside_spher() {
        let r = Ray::from_triplets((0.0, 0.0, 0.0), (0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = intersects(&s, &r);
        assert_eq!(xs, [-1.0, 1.0]);
    }

    #[test]
    fn sphere_is_behind_ray() {
        let r = Ray::from_triplets((0.0, 0.0, 5.0), (0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = intersects(&s, &r);
        assert_eq!(xs, [-6.0, -4.0]);
    }
}
