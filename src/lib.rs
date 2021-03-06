//! Ray tracing library developed with
//! TRTC book (https://pragprog.com/book/jbtracer/the-ray-tracer-challenge)
//!
//! TODO: Benchmark, profile and optimize primitives (avoid copies...)

// TODO: public or crate internal?
// TODO: re-export instead some entities?
pub mod camera;
pub mod canvas;
pub mod color;
pub mod light;
pub mod matrix;
pub mod ray;
pub mod shape;
pub mod transform;
pub mod tuple;
pub mod world;

mod util;
