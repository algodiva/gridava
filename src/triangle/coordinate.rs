//! Coordinate system for triangle based grids.

use crate::lib::*;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub struct Triangle {
    pub x: i32,
    pub y: i32,
}

impl Default for Triangle {
    fn default() -> Self {
        triangle!(0, 0)
    }
}

/// Helper macro to create [`Triangle`] structs.
#[macro_export]
macro_rules! triangle {
    ($x:expr, $y:expr) => {
        Triangle { x: $x, y: $y }
    };
}
pub use triangle;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub enum TriOrientation {
    Up,
    Down,
}

impl From<Triangle> for TriOrientation {
    fn from(value: Triangle) -> Self {
        match value.x & 1 != 0 {
            true => TriOrientation::Down,
            false => TriOrientation::Up,
        }
    }
}

pub enum TriDirection {
    /// Left direction, correlates to negative x
    Left,
    /// Right direction, correlates to positive x
    Right,
    /// Base of the triangle, the third leg after the left and right
    Base,
}

impl From<i32> for TriDirection {
    fn from(value: i32) -> Self {
        match value.rem_euclid(3) {
            0 => TriDirection::Left,
            1 => TriDirection::Right,
            2 => TriDirection::Base,
            _ => unreachable!(),
        }
    }
}

impl From<TriDirection> for i32 {
    fn from(value: TriDirection) -> Self {
        match value {
            TriDirection::Left => 0,
            TriDirection::Right => 1,
            TriDirection::Base => 2,
        }
    }
}

impl TriDirection {
    pub fn to_movement_vector(&self, orientation: TriOrientation) -> Triangle {
        match (self, orientation) {
            (TriDirection::Left, TriOrientation::Up) => triangle!(-1, 1),
            (TriDirection::Left, TriOrientation::Down) => triangle!(-1, -1),
            (TriDirection::Right, TriOrientation::Up) => triangle!(1, 1),
            (TriDirection::Right, TriOrientation::Down) => triangle!(1, -1),
            (TriDirection::Base, TriOrientation::Up) => triangle!(1, -1),
            (TriDirection::Base, TriOrientation::Down) => triangle!(-1, 1),
        }
    }
}

impl Triangle {
    pub fn compute_z(&self) -> i32 {
        -self.x - self.y + (self.y & 1)
    }

    pub fn is_triangle_face(&self) -> bool {
        self.x & 1 == self.y & 1
    }

    pub fn orientation(&self) -> TriOrientation {
        (*self).into()
    }

    pub fn make_vector(&self, magnitude: i32, rot_dir: i32) -> Self {
        *self + TriDirection::from(rot_dir).to_movement_vector((*self).into()) * magnitude
    }

    pub fn neighbor(&self, direction: TriDirection) -> Self {
        self.make_vector(1, direction.into())
    }

    pub fn neighbors(&self) -> [Self; 3] {
        [
            self.neighbor(TriDirection::Left),
            self.neighbor(TriDirection::Right),
            self.neighbor(TriDirection::Base),
        ]
    }

    pub fn are_neighbors(&self, coords: &[Self]) -> bool {
        let neighbors = self.neighbors();

        for coord in coords {
            if !neighbors.contains(coord) {
                return false;
            }
        }
        true
    }

    pub fn distance(&self, b: Self) -> i32 {
        (self.x - b.x)
            .abs()
            .max((self.y - b.y).abs())
            .max((self.compute_z() - b.compute_z()).abs())
    }
}

impl Add for Triangle {
    type Output = Self;

    fn add(self, rhs: Triangle) -> Self::Output {
        triangle!(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Triangle {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Triangle {
    type Output = Triangle;

    fn sub(self, rhs: Self) -> Self::Output {
        triangle!(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Triangle {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<T> Mul<T> for Triangle
where
    i32: Mul<T, Output = i32>,
    T: Copy,
{
    type Output = Triangle;

    fn mul(self, rhs: T) -> Self::Output {
        triangle!(self.x * rhs, self.y * rhs)
    }
}

impl Div<i32> for Triangle {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        triangle!(self.x / rhs, self.y / rhs)
    }
}

impl Neg for Triangle {
    type Output = Self;

    fn neg(self) -> Self::Output {
        triangle!(-self.x, -self.y)
    }
}
