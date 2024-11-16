use std::cmp::PartialEq;
use std::convert::From;
use std::ops::{Add, Mul, Neg, Sub};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Cube<T> {
    pub q: T,
    pub r: T,
    pub s: T,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Axial<T> {
    pub q: T,
    pub r: T,
}

#[macro_export]
macro_rules! cube {
    ($q:expr, $r:expr, $s:expr) => {
        Cube {
            q: $q,
            r: $r,
            s: $s,
        }
    };
}
pub use cube;

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

pub fn compute_s<T>(from: (T, T)) -> T
where
    T: Neg<Output = T> + Sub<Output = T>,
{
    -from.0 - from.1
}

// Cube conversion

impl<T: Copy> From<Axial<T>> for Cube<T>
where
    T: Neg<Output = T> + Sub<Output = T>,
{
    fn from(value: Axial<T>) -> Self {
        cube!(value.q, value.r, compute_s((value.q, value.r)))
    }
}

// Cube Comparison

// Cube Arithmetic

impl<T: Add<Output = T>> Add for Cube<T> {
    type Output = Cube<T>;

    fn add(self, rhs: Self) -> Self::Output {
        cube!(self.q + rhs.q, self.r + rhs.r, self.s + rhs.s)
    }
}

impl<T: Sub<Output = T>> Sub for Cube<T> {
    type Output = Cube<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        cube!(self.q - rhs.q, self.r - rhs.r, self.s - rhs.s)
    }
}

impl<T: Copy + Mul<Output = T>> Mul<T> for Cube<T> {
    type Output = Cube<T>;

    fn mul(self, rhs: T) -> Self::Output {
        cube!(self.q * rhs, self.r * rhs, self.s * rhs)
    }
}

// Axial conversion

impl<T> From<Cube<T>> for Axial<T> {
    fn from(value: Cube<T>) -> Self {
        axial!(value.q, value.r)
    }
}

// Axial Arithmetic

impl<T: Add<Output = T>> Add for Axial<T> {
    type Output = Axial<T>;

    fn add(self, rhs: Self) -> Self::Output {
        axial!(self.q + rhs.q, self.r + rhs.r)
    }
}

impl<T: Sub<Output = T>> Sub for Axial<T> {
    type Output = Axial<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        axial!(self.q - rhs.q, self.r - rhs.r)
    }
}

impl<T: Copy + Mul<Output = T>> Mul<T> for Axial<T> {
    type Output = Axial<T>;

    fn mul(self, rhs: T) -> Self::Output {
        axial!(self.q * rhs, self.r * rhs)
    }
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn identity_structs() {
        use super::*;
        use num::{One, Zero};

        let axial = axial!(-i64::one(), i64::zero());
        assert_eq!(axial.q, -1);
        assert_eq!(axial.r, 0);

        let cube = cube!(i32::one(), -i32::one(), i32::zero());
        assert_eq!(cube.q, 1);
        assert_eq!(cube.r, -1);
        assert_eq!(cube.s, 0);

    }

    #[test]
    fn coordinate_macros() {
        use super::*;
        let axial = axial!(32, -45);
        let cube = cube!(32, -45, 16);

        assert_eq!(axial, Axial { q: 32, r: -45 });
        assert_eq!(
            cube,
            Cube {
                q: 32,
                r: -45,
                s: 16
            }
        );
    }

    #[test]
    fn coordinate_conversion() {
        use super::*;

        let axial1 = axial!(5, -8);
        let cube2 = Cube::from(axial1);

        assert_eq!(cube2, cube!(5, -8, 3));

        let cube1 = cube!(10, 1, -3);
        let axial2 = Axial::from(cube1);

        assert_eq!(axial2, axial!(10, 1));
    }

    #[test]
    fn coordinate_addition() {
        use super::*;

        let axial1 = axial!(0, 1);
        let axial2 = axial!(1, 1);

        assert_eq!(axial1 + axial2, axial!(1, 2));

        let cube1 = cube!(0, 1, 0);
        let cube2 = cube!(1, 1, 2);

        assert_eq!(cube1 + cube2, cube!(1, 2, 2));
    }

    #[test]
    fn coordinate_subtraction() {
        use super::*;

        let axial1 = axial!(0, 1);
        let axial2 = axial!(1, 1);

        assert_eq!(axial1 - axial2, axial!(-1, 0));

        let cube1 = cube!(0, 1, 0);
        let cube2 = cube!(1, 1, 2);

        assert_eq!(cube1 - cube2, cube!(-1, 0, -2));
    }

    #[test]
    fn coordinate_scale() {
        use super::*;

        let axial1 = axial!(0, 1);

        assert_eq!(axial1 * 4, axial!(0, 4));

        let cube1 = cube!(1, 2, 0);

        assert_eq!(cube1 * 4, cube!(4, 8, 0));
    }
}
