use std::{
    cmp::PartialEq,
    ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign},
};

use crate::core::transform::Transformable;

#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub struct Axial {
    pub q: i32,
    pub r: i32,
}

#[macro_export]
macro_rules! axial {
    ($q:expr, $r:expr) => {
        Axial { q: $q, r: $r }
    };
}
pub use axial;

//Positive Q denotes forward vector
pub enum HexDirection {
    Front,
    FrontRight,
    BackRight,
    Back,
    BackLeft,
    FrontLeft,
}

pub enum Axes {
    Q,
    R,
    S,
}

impl Axial {
    pub fn compute_s(&self) -> i32 {
        -self.q - self.r
    }

    pub fn swizzle_l(&self) -> Self {
        axial!(self.r, self.compute_s())
    }

    pub fn swizzle_r(&self) -> Self {
        axial!(self.compute_s(), self.q)
    }
}

impl Transformable<Axial> for Axial {
    fn apply_rotation(&self, rotation: i32) -> Axial {
        self.rotate(None, rotation)
    }
}

impl Add for Axial {
    type Output = Axial;

    fn add(self, rhs: Self) -> Self::Output {
        axial!(self.q + rhs.q, self.r + rhs.r)
    }
}

impl AddAssign for Axial {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Axial {
    type Output = Axial;

    fn sub(self, rhs: Self) -> Self::Output {
        axial!(self.q - rhs.q, self.r - rhs.r)
    }
}

impl SubAssign for Axial {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

// TODO: determine if we wish to return floats to handle potential truncation
impl Div<i32> for Axial {
    type Output = Axial;

    fn div(self, rhs: i32) -> Self::Output {
        axial!(self.q / rhs, self.r / rhs)
    }
}

impl<T> Mul<T> for Axial
where
    i32: Mul<T, Output = i32>,
    T: Copy,
{
    type Output = Axial;

    fn mul(self, rhs: T) -> Self::Output {
        axial!(self.q * rhs, self.r * rhs)
    }
}

impl Neg for Axial {
    type Output = Self;

    fn neg(self) -> Self::Output {
        axial!(-self.q, -self.r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn axial_macro() {
        assert_eq!(Axial { q: 4, r: 3 }, axial!(4, 3));
        assert_ne!(Axial { q: 2, r: -1 }, axial!(1, -1));
        assert_ne!(Axial { q: 2, r: -1 }, axial!(2, -2));
    }

    #[test]
    fn compute_s() {
        assert_eq!(axial!(4, 3).compute_s(), -7);
        assert_eq!(axial!(-3, -2).compute_s(), 5);
    }

    #[test]
    fn add() {
        assert!(axial!(4, 2) + axial!(1, 3) == axial!(5, 5));
    }

    #[test]
    fn sub() {
        assert!(axial!(4, 2) - axial!(1, 3) == axial!(3, -1));
    }

    #[allow(clippy::erasing_op)]
    #[test]
    fn mult() {
        assert!(axial!(4, 2) * 2 == axial!(8, 4));
        assert!(axial!(4, 2) * 0 == axial!(0, 0));
    }

    #[test]
    fn div() {
        assert!(axial!(4, 2) / 2 == axial!(2, 1));
        assert!(axial!(41, 23) / 6 == axial!(6, 3));
    }
}
