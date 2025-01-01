//! A library for working with grids of different shapes.

#![forbid(unsafe_code)]
#![no_std]
#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

/// Unified library imports in order to support no_std
mod lib {
    mod core {
        #[cfg(not(feature = "std"))]
        pub use core::*;
        #[cfg(feature = "std")]
        pub use std::*;
    }

    // Because Rust has determined to hide a constant behind an 'unstable' tag we restate it here.
    /// Constant calculation of square root of 3.
    #[allow(clippy::excessive_precision)]
    pub const SQRT_3: f64 = 1.732050807568877293527446341505872367_f64;

    pub use self::core::cmp::PartialEq;
    pub use self::core::f64;
    pub use self::core::fmt::{self, Display};
    pub use self::core::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Rem, Sub, SubAssign};

    #[cfg(all(feature = "alloc", not(feature = "std")))]
    pub use alloc::vec::Vec;

    #[cfg(feature = "std")]
    pub use std::{vec, vec::Vec};

    // Use serde if enabled.
    #[cfg(feature = "serde")]
    pub use serde::{Deserialize, Serialize};

    #[cfg(any(feature = "std", feature = "alloc"))]
    pub use ndarray::{array, Array, Array2};

    // Use libm when no_std
    #[cfg(not(feature = "std"))]
    pub use libm::{atan2, fabs, round};
}
pub mod core;
pub mod hex;
pub mod triangle;
