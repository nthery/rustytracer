//! Internal utilities.

pub const EPSILON: f64 = 0.00001;

/// Return true if arguments are approximatevely equal.
/// We use the implementation from TRTC so that we can copy-paste tests easily.
///
/// This is probably not the best implementation.  See for example:
/// https://users.rust-lang.org/t/assert-eq-for-float-numbers/7034/4
pub fn nearly_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}
