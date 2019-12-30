//! Ray tracing library developed with
//! TRTC book (https://pragprog.com/book/jbtracer/the-ray-tracer-challenge)
//!
//! TODO: Benchmark, profile and optimize primitives (avoid copies...)

// TODO: public or crate internal?
// TODO: re-export instead some entities?
pub mod canvas;
pub mod color;
pub mod inter;
pub mod light;
pub mod matrix;
pub mod ray;
pub mod sphere;
pub mod transform;
pub mod tuple;

mod util;
