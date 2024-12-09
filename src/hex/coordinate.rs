//! Coordinate system for hex based grids.

use std::{
    cmp::PartialEq,
    ops::{Add, AddAssign, Div, Mul, Neg, Rem, Sub, SubAssign},
};

use super::{
    edge::{Edge, EdgeDirection},
    grid::SQRT_3,
    vertex::{vertex, Vertex, VertexSpin},
};
use crate::{core::transform::Transform, edge};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Axial based coordinates for hexagon grids.
///
/// This coordinate system follows the law that `q + r + s = 0`.
/// Only the q and r axes are stored and we calculate the s when we need to.
///
/// The coordinate system is similar but not fully analogus to cartesian 3D X, Y, Z.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Default)]
pub struct Axial {
    /// q (x) coordinate
    pub q: i32,
    /// r (y) coordinate
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, Debug)]
pub enum HexDirection {
    /// Direction denoting positive q (x) axis
    Front,
    /// Direction 60° CW from [`HexDirection::Front`]
    ///
    /// This direction is also the positive r (y) axis.
    FrontRight,
    /// Direction 120° CW from [`HexDirection::Front`]
    BackRight,
    /// Direction 180° CW from [`HexDirection::Front`]
    Back,
    /// Direction 240° CW from [`HexDirection::Front`]
    BackLeft,
    /// Direction 300° CW from [`HexDirection::Front`]
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
    /// use gridava::hex::coordinate::{Axial, HexDirection,axial};
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
    /// q axis of a hexagonal grid
    Q,
    /// r axis of a hexagonal grid
    R,
    /// s axis of a hexagonal grid
    S,
}

impl Axial {
    /// Computes the S component.
    ///
    /// Follows the law of `q + r + s = 0`
    ///
    /// # Example
    /// ```
    /// use gridava::hex::coordinate::{Axial, axial};
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
    /// use gridava::hex::coordinate::{Axial, axial};
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
    /// use gridava::hex::coordinate::{Axial, axial};
    ///
    /// let coordinate = axial!(1, 1); // q = 1, r = 1, s = -2
    /// let new_coordinate = coordinate.swizzle_r(); // q = -2, r = 1, s = 1
    /// ```
    pub fn swizzle_r(&self) -> Self {
        axial!(self.compute_s(), self.q)
    }

    /// Applies a transform matrix to this coordinate.
    ///
    /// Scale has no meaning with a point so we do not scale here.
    ///
    /// The order of applications is rotation then translation.
    ///
    /// # Example
    /// ```
    /// use gridava::hex::coordinate::{Axial, axial};
    /// use gridava::core::transform::{Transform, transform, Vector2D, vector2d};
    ///
    /// let new_coord = axial!(2, 5).apply_transform(transform!(axial!(1, 1), 4));
    /// ```
    pub fn apply_transform(&self, transform: Transform<Self>) -> Self {
        self.rotate(None, transform.rotation) + transform.translation
    }

    /// Make a vector from its components.
    ///
    /// Forms a vector from a location, magnitude and direction.
    ///
    /// `rot_dir`: positive denotes CW, negative CCW, magnitude denotes how many 60 degree rotations.
    ///
    /// # Example
    ///
    /// ```
    /// use gridava::hex::coordinate::{Axial, axial};
    ///
    /// // Create a unit vector (1, 0)
    /// let unit_vector = axial!(0, 0).make_vector(1, 0);
    ///
    /// // Create a unit vector (0, 1)
    /// let unit_vector = axial!(0, 0).make_vector(1, 1);
    /// ```
    pub fn make_vector(&self, magnitude: i32, rot_dir: i32) -> Self {
        *self + HexDirection::from(rot_dir).to_movement_vector() * magnitude
    }

    /// Get a neighbor coordinate given a direction.
    ///
    /// See [`HexDirection`] for a reference of directionality.
    ///
    /// # Example
    /// ```
    /// use gridava::hex::coordinate::{Axial, HexDirection, axial};
    ///
    /// // Gets the tile (1, 0)
    /// let coord = axial!(0, 0).neighbor(HexDirection::Front);
    /// ```
    pub fn neighbor(&self, direction: HexDirection) -> Self {
        self.make_vector(1, direction.into())
    }

    /// Get all the neighbors for this coordinate.
    ///
    /// See [`HexDirection`] for a reference of directionality.
    ///
    /// # Example
    /// ```
    /// use gridava::hex::coordinate::{Axial, axial};
    ///
    /// let coord = axial!(1, 2);
    ///
    /// if axial!(0, 0).neighbors().contains(&coord) {
    ///     // Do something ...
    /// }
    /// ```
    pub fn neighbors(&self) -> [Self; 6] {
        [
            self.neighbor(HexDirection::Front),
            self.neighbor(HexDirection::FrontRight),
            self.neighbor(HexDirection::BackRight),
            self.neighbor(HexDirection::Back),
            self.neighbor(HexDirection::BackLeft),
            self.neighbor(HexDirection::FrontLeft),
        ]
    }

    /// Checks if ALL of the supplied coordinates are neighbors to self.
    ///
    /// See [`Axial::neighbor`].
    ///
    /// # Example
    /// ```
    /// use gridava::hex::coordinate::{Axial, axial};
    ///
    /// let coord = axial!(1, 2);
    ///
    /// if axial!(0, 0).are_neighbors(&[coord]) {
    ///     // Do something ...
    /// }
    /// ```
    pub fn are_neighbors(&self, coords: &[Self]) -> bool {
        let neighbors = self.neighbors();

        for coord in coords {
            if !neighbors.contains(coord) {
                return false;
            }
        }
        true
    }

    /// Generates all 6 vertices that are associated with this tile.
    ///
    /// See [`Vertex`].
    ///
    /// # Example
    /// ```
    /// use gridava::hex::vertex::Vertex;
    /// use gridava::hex::coordinate::{Axial, axial};
    ///
    /// axial!(0, 0).vertices().iter().map(|vert| {/* ... */} );
    ///
    /// ```
    pub fn vertices(&self) -> [Vertex; 6] {
        [
            vertex!(self.q, self.r, VertexSpin::Up),
            vertex!(self.q + 1, self.r - 1, VertexSpin::Down),
            vertex!(self.q, self.r + 1, VertexSpin::Up),
            vertex!(self.q, self.r, VertexSpin::Down),
            vertex!(self.q - 1, self.r + 1, VertexSpin::Up),
            vertex!(self.q, self.r - 1, VertexSpin::Down),
        ]
    }

    /// Generates all 6 edges that are associated with this tile.
    ///
    /// See [`Vertex`].
    ///
    /// # Example
    /// ```
    /// use gridava::hex::edge::Edge;
    /// use gridava::hex::coordinate::{Axial, axial};
    ///
    /// axial!(0, 0).edges().iter().map(|edge| {/* ... */} );
    ///
    /// ```
    pub fn edges(&self) -> [Edge; 6] {
        [
            edge!(self.q, self.r, EdgeDirection::NorthEast),
            edge!(self.q + 1, self.r, EdgeDirection::West),
            edge!(self.q, self.r + 1, EdgeDirection::NorthWest),
            edge!(self.q - 1, self.r + 1, EdgeDirection::NorthEast),
            edge!(self.q, self.r, EdgeDirection::West),
            edge!(self.q, self.r, EdgeDirection::NorthWest),
        ]
    }

    /// Given two neighboring tiles produce the shared vertices.
    ///
    /// See [`Vertex`].
    ///
    /// # Example
    /// ```
    /// use gridava::hex::vertex::Vertex;
    /// use gridava::hex::coordinate::{Axial, axial};
    ///
    /// let verts = axial!(0, 0).shared_vert_two(axial!(1, 0));
    ///
    /// ```
    pub fn shared_vert_two(&self, b: Self) -> Option<[Vertex; 2]> {
        for i in 0..=5 {
            let dir = HexDirection::from(i);
            let vec = dir.to_movement_vector();
            if b == vec + *self {
                // We found the neighbor and its direction.
                // Front is in the positive q direction here.
                return match dir {
                    HexDirection::Front => Some([
                        vertex!(self.q + 1, self.q - 1, VertexSpin::Down),
                        vertex!(self.q, self.q + 1, VertexSpin::Up),
                    ]),
                    HexDirection::FrontRight => Some([
                        vertex!(self.q, self.q + 1, VertexSpin::Up),
                        vertex!(self.q, self.q, VertexSpin::Down),
                    ]),
                    HexDirection::BackRight => Some([
                        vertex!(self.q, self.q, VertexSpin::Down),
                        vertex!(self.q - 1, self.q + 1, VertexSpin::Up),
                    ]),
                    HexDirection::Back => Some([
                        vertex!(self.q - 1, self.q + 1, VertexSpin::Up),
                        vertex!(self.q, self.q - 1, VertexSpin::Down),
                    ]),
                    HexDirection::BackLeft => Some([
                        vertex!(self.q, self.q - 1, VertexSpin::Down),
                        vertex!(self.q, self.q, VertexSpin::Up),
                    ]),
                    HexDirection::FrontLeft => Some([
                        vertex!(self.q, self.q, VertexSpin::Up),
                        vertex!(self.q + 1, self.q - 1, VertexSpin::Down),
                    ]),
                };
            }
        }
        None
    }

    /// Given three neighboring tiles produce the shared vertex.
    ///
    /// See [`Vertex`].
    ///
    /// # Example
    /// ```
    /// use gridava::hex::vertex::Vertex;
    /// use gridava::hex::coordinate::{Axial, axial};
    ///
    /// let verts = axial!(0, 0).shared_vert_three(axial!(1, 0), axial!(0, 1));
    ///
    /// ```
    pub fn shared_vert_three(&self, b: Self, c: Self) -> Option<Vertex> {
        let ab_verts = self.shared_vert_two(b)?;

        // If c has vert 0 then we exit with 0
        if c.vertices().contains(&ab_verts[0]) {
            Some(ab_verts[0])
        } else if c.vertices().contains(&ab_verts[1]) {
            // Otherwise it's the other
            Some(ab_verts[1])
        } else {
            None
        }
    }

    /// Compute distance between two coordinates.
    ///
    /// # Example
    /// ```
    /// use gridava::hex::coordinate::{Axial, axial};
    ///
    /// // dist will be 2
    /// let dist = axial!(0, 0).distance(axial!(2, 0));
    ///
    /// // dist will be 2
    /// let dist = Axial::distance(&axial!(-1, 3), axial!(1, 1));
    /// ```
    pub fn distance(&self, b: Self) -> i32 {
        let vec = *self - b;
        (i32::abs(vec.q) + i32::abs(vec.q + vec.r) + i32::abs(vec.r)) / 2
    }

    /// Direction to b from self.
    ///
    /// Outputs degrees from hex forward vector, +q, to the target b.
    /// The
    pub fn direction(&self, b: Self) -> f64 {
        // direction to b from the pov of self
        let vec = b - *self;

        let x = SQRT_3 * vec.q as f64 + SQRT_3 / 2.0 * vec.r as f64;
        let y = 3.0 / 2.0 * vec.r as f64;
        -y.atan2(-x).to_degrees() + 180.0
    }

    // utilize f64 to preserve lossless conversion for i32
    fn lerp_internal(a: i32, b: i32, t: f64) -> f64 {
        a as f64 + (b - a) as f64 * t
    }

    /// Rounds a floating hex coordinate to an integer coordinate.
    ///
    /// This algorithm is based on the round function by Jacob Rus
    /// <https://observablehq.com/@jrus/hexround>
    ///
    /// # Example
    /// ```
    /// use gridava::hex::coordinate::{Axial, axial};
    ///
    /// let coord = Axial::round((1.6, 3.2));
    /// ```
    pub fn round(fcoord: (f64, f64)) -> Self {
        let qgrid = fcoord.0.round();
        let rgrid = fcoord.1.round();

        let q_rem = fcoord.0 - qgrid;
        let r_rem = fcoord.1 - rgrid;

        if q_rem.abs() >= r_rem.abs() {
            let q = qgrid + f64::round(q_rem + 0.5 * r_rem);
            axial!(q as i32, rgrid as i32)
        } else {
            let r = rgrid + f64::round(r_rem + 0.5 * q_rem);
            axial!(qgrid as i32, r as i32)
        }
    }

    /// Perferms linear interpolation between two coordinates.
    ///
    /// Given time `t`, or a percentage, calculate an inbetween value along the line.
    ///
    /// # Example
    /// ```
    /// use gridava::hex::coordinate::{Axial, axial};
    ///
    /// // The coordinate 30% of the way to (3, 0) is (1, 0)
    /// let coord = axial!(0, 0).lerp(axial!(3, 0), 0.3);
    /// ```
    pub fn lerp(&self, b: Self, t: f64) -> Self {
        let q = Self::lerp_internal(self.q, b.q, t);
        let r = Self::lerp_internal(self.r, b.r, t);
        Self::round((q, r))
    }

    /// Calculate all the coordinates that form a line between two points.
    ///
    /// # Example
    /// ```
    /// use gridava::hex::coordinate::{Axial, axial};
    ///
    /// // coords will contain (0, 0) (1, 0) and (2, 0)
    /// let coords = axial!(0, 0).line(axial!(2, 0));
    /// ```
    pub fn line(&self, b: Self) -> Vec<Self> {
        let dist = self.distance(b);
        let mut ret = vec![];

        let constant = 1.0 / dist as f64;

        for i in 0..=dist {
            ret.push(self.lerp(b, constant * i as f64));
        }

        ret
    }

    /// Calculate all the coordinates within a range.
    ///
    /// # Example
    /// ```
    /// use gridava::hex::coordinate::{Axial, axial};
    ///
    /// // coords will contain all the neighbors of (0, 0)
    /// let coords = axial!(0, 0).range(1);
    /// ```
    pub fn range(&self, range: i32) -> Vec<Self> {
        let mut ret = vec![];

        for q in -range..=range {
            for r in i32::max(-range, -q - range)..=i32::min(range, -q + range) {
                ret.push(*self + axial!(q, r));
            }
        }

        ret
    }

    // center: Option<Self> denotes a point to reflect about. If provided None, coordinate (0,0) will be used.
    /// Reflect a coordinate across an axis of symmetry.
    ///
    /// `center` can be provided to specify a specific point to reflect across. Otherwise, (0, 0) will be used.
    ///
    /// # Example
    /// ```
    /// use gridava::hex::coordinate::{Axial, Axes, axial};
    ///
    /// // reflected will be the coordinate (-1, 0)
    /// let reflected = axial!(1, 0).reflect(None, Axes::Q);
    ///
    /// // reflected will be the coordinate (1, 0)
    /// let reflected = axial!(1, 0).reflect(None, Axes::R);
    ///
    /// // reflected will be the coordinate (0, 1)
    /// let reflected = axial!(1, 0).reflect(None, Axes::S);
    ///
    /// // reflected will be the coordinate (0, 2)
    /// let reflected = axial!(0, 0).reflect(Some(axial!(0, 1)), Axes::Q);
    /// ```
    pub fn reflect(&self, center: Option<Self>, axes: Axes) -> Self {
        let center = match center {
            Some(c) => c,
            None => axial!(0, 0),
        };

        let centered_coord = *self - center;

        match axes {
            Axes::Q => axial!(centered_coord.q, centered_coord.compute_s()) + center,
            Axes::R => axial!(centered_coord.compute_s(), centered_coord.r) + center,
            Axes::S => axial!(centered_coord.r, centered_coord.q) + center,
        }
    }

    pub(self) fn rotate_recursive(&self, iter: usize, cw: bool) -> Self {
        if iter == 0 {
            *self
        } else {
            let input = match cw {
                true => -self.swizzle_l(),
                false => -self.swizzle_r(),
            };
            input.rotate_recursive(iter - 1, cw)
        }
    }

    /// Rotate a coordinate.
    ///
    /// `center` Optionally can specify a point to rotate about. None will rotate about (0, 0).
    ///
    /// `rot_dir`: positive denotes CW, negative CCW, magnitude denotes how many 60 degree rotations.
    ///
    /// # Example
    /// ```
    /// use gridava::hex::coordinate::{Axial, axial};
    ///
    /// // coord will be (0, 1)
    /// let coord = axial!(1, 0).rotate(None, 1);
    ///
    /// // coord will be (2, -1)
    /// let coord = axial!(1, 0).rotate(Some(axial!(2, 0)), 1);
    /// ```
    pub fn rotate(&self, center: Option<Self>, rot_dir: i32) -> Self {
        let center = match center {
            Some(c) => c,
            None => axial!(0, 0),
        };

        let centered_coord = *self - center;

        if rot_dir < 0 {
            // negative, CCW
            centered_coord.rotate_recursive(rot_dir.rem(6).unsigned_abs() as usize, false) + center
        } else {
            // positive, CW
            centered_coord.rotate_recursive(rot_dir.rem(6).unsigned_abs() as usize, true) + center
        }
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
    use assert_float_eq::*;

    use crate::{
        core::transform::{vector2d, Vector2D},
        transform,
    };

    use super::*;

    #[test]
    fn axial_macro() {
        assert_eq!(Axial { q: 4, r: 3 }, axial!(4, 3));
        assert_ne!(Axial { q: 2, r: -1 }, axial!(1, -1));
        assert_ne!(Axial { q: 2, r: -1 }, axial!(2, -2));
    }

    #[test]
    fn from_tuple() {
        assert_eq!(<(i32, i32)>::from(axial!(0, 0)), (0, 0));
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
    fn apply_transform() {
        let transform = transform!(axial!(1, 1), 1);
        assert_eq!(axial!(0, 0).apply_transform(transform), axial!(1, 1));
        assert_eq!(axial!(1, 1).apply_transform(transform), axial!(0, 3));
    }

    #[test]
    fn neighbors() {
        assert_eq!(
            axial!(0, 0).neighbors(),
            [
                axial!(1, 0),
                axial!(0, 1),
                axial!(-1, 1),
                axial!(-1, 0),
                axial!(0, -1),
                axial!(1, -1)
            ]
        )
    }

    #[test]
    fn are_neighbors() {
        assert!(axial!(0, 0).are_neighbors(&[axial!(1, 0)]));
        assert!(!axial!(0, 0).are_neighbors(&[axial!(1, 1)]));
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

    #[test]
    fn make_vector() {
        assert_eq!(axial!(6, 4).make_vector(4, 0), axial!(10, 4));
        assert_eq!(axial!(6, 4).make_vector(4, 1), axial!(6, 8));
        assert_eq!(axial!(6, 4).make_vector(4, 2), axial!(2, 8));
        assert_eq!(axial!(6, 4).make_vector(4, 3), axial!(2, 4));
        assert_eq!(axial!(6, 4).make_vector(4, 4), axial!(6, 0));
        assert_eq!(axial!(6, 4).make_vector(4, 5), axial!(10, 0));
    }

    #[test]
    fn neighbor() {
        assert_eq!(axial!(6, 4).neighbor(HexDirection::Back), axial!(5, 4));
        assert_eq!(axial!(6, 4).neighbor(HexDirection::BackLeft), axial!(6, 3));
        assert_eq!(axial!(6, 4).neighbor(HexDirection::BackRight), axial!(5, 5));
        assert_eq!(axial!(6, 4).neighbor(HexDirection::Front), axial!(7, 4));
        assert_eq!(axial!(6, 4).neighbor(HexDirection::FrontLeft), axial!(7, 3));
        assert_eq!(
            axial!(6, 4).neighbor(HexDirection::FrontRight),
            axial!(6, 5)
        );
    }

    #[test]
    fn distance() {
        assert_eq!(axial!(-1, -1).distance(axial!(-1, -1)), 0);
        assert_eq!(axial!(-1, -1).distance(axial!(1, -1)), 2);
        assert_eq!(axial!(-1, -1).distance(axial!(-1, 1)), 2);
        assert_eq!(axial!(-1, -1).distance(axial!(2, 1)), 5);
    }

    #[test]
    fn round() {
        assert_eq!(Axial::round((2.5, 1.5)), axial!(2, 2));
        assert_eq!(Axial::round((2.5, -1.5)), axial!(3, -2));
        assert_eq!(Axial::round((-2.5, -1.5)), axial!(-2, -2));
        assert_eq!(Axial::round((-2.5, 1.5)), axial!(-3, 2));
    }

    #[test]
    fn direction() {
        assert_f64_near!(axial!(0, 0).direction(axial!(2, 0)), 0.0);
        assert_f64_near!(axial!(0, 0).direction(axial!(0, 2)), 60.0);
        assert_f64_near!(axial!(0, 0).direction(axial!(-1, 2)), 90.0);
        assert_f64_near!(axial!(0, 0).direction(axial!(-1, 1)), 120.0);
        assert_f64_near!(axial!(0, 0).direction(axial!(-1, 0)), 180.0);
        assert_f64_near!(axial!(0, 0).direction(axial!(2, -1)), 330.0);
        assert_f64_near!(axial!(0, 0).direction(axial!(2, -2)), 300.0);
    }

    #[test]
    fn lerp() {
        assert_eq!(axial!(-1, -1).lerp(axial!(9, 19), -0.25), axial!(-3, -6));
        assert_eq!(axial!(-1, -1).lerp(axial!(9, 19), 0.0), axial!(-1, -1));
        assert_eq!(axial!(-1, -1).lerp(axial!(9, 19), 0.25), axial!(1, 4));
        assert_eq!(axial!(-1, -1).lerp(axial!(9, 19), 0.5), axial!(4, 9));
        assert_eq!(axial!(-1, -1).lerp(axial!(9, 19), 0.75), axial!(6, 14));
        assert_eq!(axial!(-1, -1).lerp(axial!(9, 19), 1.0), axial!(9, 19));
        assert_eq!(axial!(-1, -1).lerp(axial!(9, 19), 1.25), axial!(11, 24));
    }

    #[test]
    fn line() {
        assert_eq!(
            axial!(-1, -1).line(axial!(-1, 1)),
            vec![axial!(-1, -1), axial!(-1, 0), axial!(-1, 1)]
        );
        assert_eq!(
            axial!(-1, -1).line(axial!(1, -1)),
            vec![axial!(-1, -1), axial!(0, -1), axial!(1, -1)]
        );
        assert_eq!(
            axial!(-1, -1).line(axial!(0, 1)),
            vec![axial!(-1, -1), axial!(-1, 0), axial!(0, 0), axial!(0, 1)]
        );
        assert_eq!(
            axial!(-1, -1).line(axial!(1, 0)),
            vec![axial!(-1, -1), axial!(0, -1), axial!(0, 0), axial!(1, 0)]
        );
        assert_eq!(
            axial!(-1, -1).line(axial!(1, 1)),
            vec![
                axial!(-1, -1),
                axial!(0, -1),
                axial!(0, 0),
                axial!(0, 1),
                axial!(1, 1)
            ]
        );
        assert_eq!(
            axial!(-1, 1).line(axial!(1, -1)),
            vec![axial!(-1, 1), axial!(0, 0), axial!(1, -1)]
        );
        assert_eq!(
            axial!(1, 3).line(axial!(3, 1)),
            vec![axial!(1, 3), axial!(2, 2), axial!(3, 1)]
        );
        assert_eq!(
            axial!(0, 0).line(axial!(1, 1)),
            vec![axial!(0, 0), axial!(0, 1), axial!(1, 1)]
        );
    }

    #[test]
    fn range() {
        assert_eq!(axial!(0, 0).range(0), vec![axial!(0, 0)]);
        assert_eq!(
            axial!(0, 0).range(1),
            vec![
                axial!(-1, 0),
                axial!(-1, 1),
                axial!(0, -1),
                axial!(0, 0),
                axial!(0, 1),
                axial!(1, -1),
                axial!(1, 0)
            ]
        );
        assert_eq!(
            axial!(0, 0).range(3),
            vec![
                axial!(-3, 0),
                axial!(-3, 1),
                axial!(-3, 2),
                axial!(-3, 3),
                axial!(-2, -1),
                axial!(-2, 0),
                axial!(-2, 1),
                axial!(-2, 2),
                axial!(-2, 3),
                axial!(-1, -2),
                axial!(-1, -1),
                axial!(-1, 0),
                axial!(-1, 1),
                axial!(-1, 2),
                axial!(-1, 3),
                axial!(0, -3),
                axial!(0, -2),
                axial!(0, -1),
                axial!(0, 0),
                axial!(0, 1),
                axial!(0, 2),
                axial!(0, 3),
                axial!(1, -3),
                axial!(1, -2),
                axial!(1, -1),
                axial!(1, 0),
                axial!(1, 1),
                axial!(1, 2),
                axial!(2, -3),
                axial!(2, -2),
                axial!(2, -1),
                axial!(2, 0),
                axial!(2, 1),
                axial!(3, -3),
                axial!(3, -2),
                axial!(3, -1),
                axial!(3, 0),
            ]
        );
    }

    #[test]
    fn reflect() {
        assert_eq!(axial!(-1, 1).reflect(None, Axes::Q), axial!(-1, 0));
        assert_eq!(
            axial!(1, 3).reflect(Some(axial!(1, 2)), Axes::Q),
            axial!(1, 1)
        );

        assert_eq!(axial!(-1, 1).reflect(None, Axes::R), axial!(0, 1));
        assert_eq!(
            axial!(1, 3).reflect(Some(axial!(1, 2)), Axes::R),
            axial!(0, 3)
        );

        assert_eq!(axial!(-1, 1).reflect(None, Axes::S), axial!(1, -1));
        assert_eq!(
            axial!(1, 3).reflect(Some(axial!(1, 2)), Axes::S),
            axial!(2, 2)
        );
    }

    #[test]
    fn rotate() {
        // CW
        assert_eq!(axial!(-1, 1).rotate(None, 1), axial!(-1, 0));
        assert_eq!(axial!(-1, 1).rotate(None, 2), axial!(0, -1));
        assert_eq!(axial!(-1, 1).rotate(None, 3), axial!(1, -1));
        assert_eq!(axial!(-1, 1).rotate(None, 7), axial!(-1, 0));
        assert_eq!(axial!(-1, 1).rotate(None, 8), axial!(0, -1));
        assert_eq!(axial!(-1, 1).rotate(None, 9), axial!(1, -1));

        // CCW
        assert_eq!(axial!(-1, 1).rotate(None, -1), axial!(0, 1));
        assert_eq!(axial!(-1, 1).rotate(None, -2), axial!(1, 0));
        assert_eq!(axial!(-1, 1).rotate(None, -3), axial!(1, -1));

        // About non (0, 0) center
        assert_eq!(axial!(0, 0).rotate(Some(axial!(1, 1)), 1), axial!(2, -1));
        assert_eq!(axial!(0, 0).rotate(Some(axial!(1, 1)), 2), axial!(3, 0));
        assert_eq!(axial!(0, 0).rotate(Some(axial!(1, 1)), 3), axial!(2, 2));
    }

    #[test]
    fn edges() {
        assert_eq!(
            axial!(0, 0).edges(),
            [
                edge!(0, 0, EdgeDirection::NorthEast),
                edge!(1, 0, EdgeDirection::West),
                edge!(0, 1, EdgeDirection::NorthWest),
                edge!(-1, 1, EdgeDirection::NorthEast),
                edge!(0, 0, EdgeDirection::West),
                edge!(0, 0, EdgeDirection::NorthWest),
            ]
        )
    }

    #[test]
    fn shared_vert_two() {
        assert!(axial!(0, 0).shared_vert_two(axial!(1, 1)).is_none());

        assert_eq!(
            axial!(0, 0).shared_vert_two(axial!(1, 0)).unwrap(),
            [
                vertex!(1, -1, VertexSpin::Down),
                vertex!(0, 1, VertexSpin::Up),
            ]
        );

        assert_eq!(
            axial!(0, 0).shared_vert_two(axial!(0, 1)).unwrap(),
            [
                vertex!(0, 1, VertexSpin::Up),
                vertex!(0, 0, VertexSpin::Down),
            ]
        );

        assert_eq!(
            axial!(0, 0).shared_vert_two(axial!(-1, 1)).unwrap(),
            [
                vertex!(0, 0, VertexSpin::Down),
                vertex!(-1, 1, VertexSpin::Up),
            ]
        );

        assert_eq!(
            axial!(0, 0).shared_vert_two(axial!(-1, 0)).unwrap(),
            [
                vertex!(-1, 1, VertexSpin::Up),
                vertex!(0, -1, VertexSpin::Down),
            ]
        );

        assert_eq!(
            axial!(0, 0).shared_vert_two(axial!(0, -1)).unwrap(),
            [
                vertex!(0, -1, VertexSpin::Down),
                vertex!(0, 0, VertexSpin::Up),
            ]
        );

        assert_eq!(
            axial!(1, 1).shared_vert_two(axial!(2, 0)).unwrap(),
            [
                vertex!(1, 1, VertexSpin::Up),
                vertex!(2, 0, VertexSpin::Down),
            ]
        );
    }

    #[test]
    fn shared_vert_three() {
        assert_eq!(
            axial!(1, 1)
                .shared_vert_three(axial!(2, 0), axial!(2, 1))
                .unwrap(),
            vertex!(2, 0, VertexSpin::Down)
        );

        assert_eq!(
            axial!(1, 1)
                .shared_vert_three(axial!(1, 2), axial!(2, 1))
                .unwrap(),
            vertex!(1, 2, VertexSpin::Up)
        );

        assert_eq!(
            axial!(0, 0)
                .shared_vert_three(axial!(1, 0), axial!(0, 1))
                .unwrap(),
            vertex!(0, 1, VertexSpin::Up)
        );

        assert!(axial!(0, 0)
            .shared_vert_three(axial!(1, 1), axial!(3, 9))
            .is_none());

        assert!(axial!(0, 0)
            .shared_vert_three(axial!(1, 0), axial!(3, 3))
            .is_none());
    }
}
