//! TRTC chapter 7 "putting it together"

use std::f64::consts::PI;
use std::fs::File;

use rustytracer::camera::Camera;
use rustytracer::color::{self, Color};
use rustytracer::light::{Material, PointLight};
use rustytracer::sphere::Sphere;
use rustytracer::transform;
use rustytracer::tuple::Tuple;
use rustytracer::world::World;

#[test]
fn first_world() {
    let floor = Sphere::with_transform_and_material(
        transform::scaling(10.0, 0.01, 10.0),
        Material {
            color: Color::new(1.0, 0.9, 0.9),
            specular: 0.0,
            ..Material::default()
        },
    );

    let left_wall = Sphere::with_transform_and_material(
        &(&(&transform::translation(0.0, 0.0, 5.0) * &transform::rotation_y(-PI / 4.0))
            * &transform::rotation_x(PI / 2.0))
            * &transform::scaling(10.0, 0.01, 10.0),
        floor.material.clone(),
    );

    let right_wall = Sphere::with_transform_and_material(
        &(&(&transform::translation(0.0, 0.0, 5.0) * &transform::rotation_y(PI / 4.0))
            * &transform::rotation_x(PI / 2.0))
            * &transform::scaling(10.0, 0.01, 10.0),
        floor.material.clone(),
    );

    let middle = Sphere::with_transform_and_material(
        transform::translation(-0.5, 1.0, 0.5),
        Material {
            color: Color::new(0.1, 1.0, 0.5),
            diffuse: 0.7,
            specular: 0.3,
            ..Material::default()
        },
    );

    let right = Sphere::with_transform_and_material(
        &transform::translation(1.5, 0.5, -0.5) * &transform::scaling(0.5, 0.5, 0.5),
        Material {
            color: Color::new(0.5, 1.0, 0.1),
            diffuse: 0.7,
            specular: 0.3,
            ..Material::default()
        },
    );

    let left = Sphere::with_transform_and_material(
        &transform::translation(-1.5, 0.33, -0.75) * &transform::scaling(0.33, 0.33, 0.33),
        Material {
            color: Color::new(1.0, 0.8, 0.1),
            diffuse: 0.7,
            specular: 0.3,
            ..Material::default()
        },
    );

    let world = World {
        objects: vec![floor, left_wall, right_wall, middle, right, left],
        light: PointLight::new(color::WHITE, Tuple::new_point(-10.0, 10.0, -10.0)),
    };

    let camera = Camera::with_transform(
        500,
        250,
        PI / 3.0,
        transform::view(
            &Tuple::new_point(0.0, 1.5, -5.0),
            &Tuple::new_point(0.0, 1.0, 0.0),
            &Tuple::new_vector(0.0, 1.0, 0.0),
        ),
    );

    let canvas = camera.render(&world);

    let file = File::create("/tmp/first_world.ppm").unwrap();
    canvas.to_ppm(file).unwrap();
}
