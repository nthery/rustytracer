//! TRTC chapter 5 "putting it together" task

use std::fs::File;

use rustytracer::canvas::Canvas;
use rustytracer::color;
use rustytracer::inter::{self, IntersectionList};
use rustytracer::ray::Ray;
use rustytracer::sphere::Sphere;
use rustytracer::tuple::Tuple;

#[test]
fn sphere_shadow() {
    // Primitive constants.
    let ray_origin = Tuple::new_point(0.0, 0.0, -5.0);
    const WALL_Z: f64 = 10.0;
    const WALL_SIZE: f64 = 7.0;
    const CANVAS_PIXELS: usize = 100;

    // Derived constants.
    const PIXEL_SIZE: f64 = WALL_SIZE / CANVAS_PIXELS as f64;
    const WALL_HALF_SIZE: f64 = WALL_SIZE / 2.0;

    let mut canvas = Canvas::new(CANVAS_PIXELS, CANVAS_PIXELS);
    let sphere = Sphere::new();

    for y in 0..CANVAS_PIXELS {
        let world_y = WALL_HALF_SIZE - PIXEL_SIZE * y as f64;
        for x in 0..CANVAS_PIXELS {
            let world_x = -WALL_HALF_SIZE + PIXEL_SIZE * x as f64;
            let pos = Tuple::new_point(world_x, world_y, WALL_Z);
            let ray = Ray::new(ray_origin.clone(), (&pos - &ray_origin).normalize());
            let xs = inter::intersects(&sphere, &ray);
            if xs.hit() != None {
                canvas.set(x, y, &color::RED);
            }
        }
    }

    let file = File::create("/tmp/sphere_shadow.ppm").unwrap();
    canvas.to_ppm(file).unwrap();
}