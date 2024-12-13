//! This module contains implementations specific to hex tile based grids.
//!
//! TODO: Examples.
//!
pub mod coordinate;
pub mod edge;
#[cfg(feature = "std")]
pub mod grid;
#[cfg(any(feature = "std", feature = "alloc"))]
pub mod shape;
pub mod vertex;
