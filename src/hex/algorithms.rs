use std::ops::Neg;
use num::{One, Zero};

use super::coordinate::{axial, Axial, HexDirection};


pub fn neighbor<T: One + Zero + Neg<Output = T>>(coord: Axial<T>, direction:HexDirection) -> Axial<T> {
    match direction {
        HexDirection::Front => coord + axial!(T::one(), T::zero()),
        HexDirection::FrontRight => coord + axial!(T::zero(), T::one()),
        HexDirection::BackRight => coord + axial!(-T::one(), -T::one()),
        HexDirection::Back => coord + axial!(-T::one(), T::zero()),
        HexDirection::BackLeft => coord + axial!(T::zero(), -T::one()),
        HexDirection::FrontLeft => coord + axial!(-T::one(), T::one()),
    }
}