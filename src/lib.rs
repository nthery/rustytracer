//! Ray tracing library developed with
//! TRTC book (https://pragprog.com/book/jbtracer/the-ray-tracer-challenge)
//!
//! TODO: Really need to overload operators for T and &T?
//! TODO: Implement approximate equality using PartialEq rather than provide nearly_equal()?

// TODO: public or crate internal?
pub mod canvas;
pub mod color;
pub mod matrix;
pub mod tuple;

mod util;
