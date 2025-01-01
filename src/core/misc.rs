//! Miscellaneous useful functions

use crate::lib::*;

/// Defines axes for a 3-dimensional coordinate system.
#[derive(Debug)]
pub enum Axes3D {
    /// X axis
    X,
    /// Y Axis
    Y,
    /// Z Axis
    Z,
}

/// Linear interpolation from a to b along t.
pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}
