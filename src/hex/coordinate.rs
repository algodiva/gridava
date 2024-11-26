//! Coordinate system for hex based grids.

use std::{
    cmp::PartialEq,
    ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign},
};

/// Axial based coordinates for hexagon grids.
///
/// This coordinate system follows the law that `q + r + s = 0`.
/// Only the q and r axes are stored and we calculate the s when we need to.
///
/// The coordinate system is similar but not fully analogus to cartesian 3D X, Y, Z.
#[derive(PartialEq, Debug, Copy, Clone, Default)]
pub struct Axial {
    pub q: i32,
    pub r: i32,
}

impl From<Axial> for (i32, i32) {
    fn from(value: Axial) -> Self {
        (value.q, value.r)
    }
}

/// Helper macro to create [`Axial`] structs.
#[macro_export]
macro_rules! axial {
    ($q:expr, $r:expr) => {
        Axial { q: $q, r: $r }
    };
}
pub use axial;

/// Describes a direction.
///
/// Positive q is the forward vector for a tile, meaning these directions are in relation to that.
#[derive(PartialEq, Eq, Debug)]
pub enum HexDirection {
    Front,
    FrontRight,
    BackRight,
    Back,
    BackLeft,
    FrontLeft,
}

impl From<i32> for HexDirection {
    fn from(value: i32) -> Self {
        match value.rem_euclid(6) {
            0 => HexDirection::Front,
            1 => HexDirection::FrontRight,
            2 => HexDirection::BackRight,
            3 => HexDirection::Back,
            4 => HexDirection::BackLeft,
            5 => HexDirection::FrontLeft,
            _ => panic!(), // should never reach
        }
    }
}

impl From<HexDirection> for i32 {
    fn from(value: HexDirection) -> Self {
        match value {
            HexDirection::Front => 0,
            HexDirection::FrontRight => 1,
            HexDirection::BackRight => 2,
            HexDirection::Back => 3,
            HexDirection::BackLeft => 4,
            HexDirection::FrontLeft => 5,
        }
    }
}

impl HexDirection {
    /// Converts a HexDirection to an [`Axial`] unit vector.
    ///
    /// # Example
    /// ```
    /// use gridava::hex::coordinate::*;
    ///
    /// // Creates a unit vector of (1, 0)
    /// let front_uv = HexDirection::to_movement_vector(&HexDirection::Front);
    ///
    /// // Creates a unit vector of (-1, 1)
    /// let dir = HexDirection::BackRight;
    /// let uv = dir.to_movement_vector();
    /// ```
    pub fn to_movement_vector(&self) -> Axial {
        match self {
            HexDirection::Front => axial!(1, 0),
            HexDirection::FrontRight => axial!(0, 1),
            HexDirection::BackRight => axial!(-1, 1),
            HexDirection::Back => axial!(-1, 0),
            HexDirection::BackLeft => axial!(0, -1),
            HexDirection::FrontLeft => axial!(1, -1),
        }
    }
}

/// Represents the three axes of symmetry in hexagons.
pub enum Axes {
    Q,
    R,
    S,
}

impl Axial {
    /// Computes the S component.
    ///
    /// Follows the law of `q + r + s = 0`
    ///
    /// # Example
    /// ```
    /// use gridava::hex::coordinate::*;
    /// // Computes the s component where q and r are 1.
    /// let s = axial!(1, 1).compute_s(); // s will be -2.
    /// ```
    pub fn compute_s(&self) -> i32 {
        -self.q - self.r
    }

    /// Swizzles the coordinate components left.
    ///
    /// Performs an operation of the coordinate components where they all get shifted left as follows;
    ///
    /// `q = r`
    ///
    /// `r = s`
    ///
    /// `s = q`
    ///
    /// # Example
    /// ```
    /// use gridava::hex::coordinate::*;
    ///
    /// let coordinate = axial!(1, 1); // q = 1, r = 1, s = -2
    /// let new_coordinate = coordinate.swizzle_l(); // q = 1, r = -2, s = 1
    /// ```
    pub fn swizzle_l(&self) -> Self {
        axial!(self.r, self.compute_s())
    }

    /// Swizzles the coordinate components right.
    ///
    /// Performs an operation of the coordinate components where they all get shifted right as such;
    ///
    /// `q = s`
    ///
    /// `r = q`
    ///
    /// `s = r`
    ///
    /// # Example
    /// ```
    /// use gridava::hex::coordinate::*;
    ///
    /// let coordinate = axial!(1, 1); // q = 1, r = 1, s = -2
    /// let new_coordinate = coordinate.swizzle_r(); // q = -2, r = 1, s = 1
    /// ```
    pub fn swizzle_r(&self) -> Self {
        axial!(self.compute_s(), self.q)
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
    fn hex_dir_from() {
        assert!(HexDirection::from(0) == HexDirection::Front);
        assert!(HexDirection::from(5) == HexDirection::from(-1));
        assert!(HexDirection::from(4) == HexDirection::from(-2));
        assert!(HexDirection::from(3) == HexDirection::from(-3));
        assert!(HexDirection::from(2) == HexDirection::from(-4));
        assert!(HexDirection::from(1) == HexDirection::from(-5));
        assert!(HexDirection::from(6) == HexDirection::from(-6));
        assert!(HexDirection::from(6) == HexDirection::from(0));
    }

    #[test]
    fn compute_s() {
        assert_eq!(axial!(4, 3).compute_s(), -7);
        assert_eq!(axial!(-3, -2).compute_s(), 5);
    }

    #[test]
    fn swizzle_l() {
        assert_eq!(axial!(4, 3).swizzle_l(), axial!(3, -7));
    }

    #[test]
    fn swizzle_r() {
        assert_eq!(axial!(4, 3).swizzle_r(), axial!(-7, 4));
    }

    #[test]
    fn add() {
        assert!(axial!(4, 2) + axial!(1, 3) == axial!(5, 5));
    }

    #[test]
    fn add_assign() {
        let mut ax = axial!(4, 2);

        ax += axial!(-1, -3);

        assert!(ax == axial!(3, -1));
    }

    #[test]
    fn sub() {
        assert!(axial!(4, 2) - axial!(1, 3) == axial!(3, -1));
    }

    #[test]
    fn sub_assign() {
        let mut ax = axial!(4, 2);

        ax -= axial!(-1, -3);

        assert!(ax == axial!(5, 5));
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

    #[test]
    #[should_panic]
    #[allow(unused)]
    fn div_by_zero() {
        axial!(41, 23) / 0;
    }
}
