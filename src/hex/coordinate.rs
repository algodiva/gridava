use std::{
    cmp::PartialEq,
    ops::{Add, Div, Mul, Sub},
};

#[derive(PartialEq, Debug, Copy, Clone)]
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

impl Axial {
    pub fn compute_s(&self) -> i32 {
        -self.q - self.r
    }
}

impl Add for Axial {
    type Output = Axial;

    fn add(self, rhs: Self) -> Self::Output {
        axial!(self.q + rhs.q, self.r + rhs.r)
    }
}

impl Sub for Axial {
    type Output = Axial;

    fn sub(self, rhs: Self) -> Self::Output {
        axial!(self.q - rhs.q, self.r - rhs.r)
    }
}

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
