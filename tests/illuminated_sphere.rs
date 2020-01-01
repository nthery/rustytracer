//! TRTC chapter 6 "putting it together" task

use std::fs::File;

use rustytracer::canvas::Canvas;
use rustytracer::color::{self, Color};
use rustytracer::inter::{self, IntersectionList};
use rustytracer::light::{self, PointLight};
use rustytracer::ray::Ray;
use rustytracer::sphere::Sphere;
use rustytracer::tuple::Tuple;

#[test]
fn illuminated_sphere() {
    // Primitive constants.
    let ray_origin = Tuple::new_point(0.0, 0.0, -5.0);
    const WALL_Z: f64 = 10.0;
    const WALL_SIZE: f64 = 7.0;
    const CANVAS_PIXELS: usize = 100;

    // Derived constants.
    const PIXEL_SIZE: f64 = WALL_SIZE / CANVAS_PIXELS as f64;
    const WALL_HALF_SIZE: f64 = WALL_SIZE / 2.0;

    let mut sphere = Sphere::default();
    sphere.material.color = Color::new(1.0, 0.2, 1.0);

    let light = PointLight::new(color::WHITE, Tuple::new_point(-10.0, 10.0, -10.0));

    let mut canvas = Canvas::new(CANVAS_PIXELS, CANVAS_PIXELS);

    for y in 0..CANVAS_PIXELS {
        let world_y = WALL_HALF_SIZE - PIXEL_SIZE * y as f64;
        for x in 0..CANVAS_PIXELS {
            let world_x = -WALL_HALF_SIZE + PIXEL_SIZE * x as f64;
            let canvas_pt = Tuple::new_point(world_x, world_y, WALL_Z);
            let ray = Ray::new(ray_origin.clone(), (&canvas_pt - &ray_origin).normalized());
            let xs = inter::intersects(&sphere, &ray);
            if let Some(hit) = xs.hit() {
                let sphere_pt = ray.position(hit.distance);
                let c = light::lighting(
                    &hit.sphere.material,
                    &light,
                    &sphere_pt,
                    &-ray.direction(),
                    &hit.sphere.normal_at(&sphere_pt),
                );
                canvas.set(x, y, &c);
            }
        }
    }

    let file = File::create("/tmp/illuminated_sphere.ppm").unwrap();
    canvas.to_ppm(file).unwrap();
}
