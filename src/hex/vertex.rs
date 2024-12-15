//! Handles vertices in a hexagonal grid.

use crate::lib::*;

use crate::edge;
use crate::triangle::coordinate::{triangle, TriOrientation, Triangle};

use super::{
    coordinate::{axial, Axial},
    edge::{Edge, EdgeDirection},
};

/// Vertex spin is a orientation of the vertex.
///
/// A vertex needs to know its `spin`. Spin correlates to which side [`VertexSpin::Up`] or [`VertexSpin::Down`]
/// has two hexagons.
///
/// see [`Vertex`]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub enum VertexSpin {
    /// On top of the hex
    Up,
    /// On the bottom of the hex
    Down,
}

/// A vertex direction denotes the direction from the hexagon center the vertex is.
///
/// Reference pointy-top hexagons for vertex direction, where up being directly above the center.
///
/// see [`Vertex`]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum VertexDirection {
    /// The vertex at the top
    Up,
    /// The vertex at the top right
    UpRight,
    /// The vertex at the bottom right
    DownRight,
    /// The vertex at the bottom
    Down,
    /// The vertex at the bottom left
    DownLeft,
    /// The vertex at the top left
    UpLeft,
}

impl From<i32> for VertexDirection {
    fn from(value: i32) -> Self {
        match value.rem_euclid(6) {
            0 => VertexDirection::Up,
            1 => VertexDirection::UpRight,
            2 => VertexDirection::DownRight,
            3 => VertexDirection::Down,
            4 => VertexDirection::DownLeft,
            5 => VertexDirection::UpLeft,
            _ => unreachable!(), // should never reach
        }
    }
}

impl From<VertexDirection> for i32 {
    fn from(value: VertexDirection) -> Self {
        match value {
            VertexDirection::Up => 0,
            VertexDirection::UpRight => 1,
            VertexDirection::DownRight => 2,
            VertexDirection::Down => 3,
            VertexDirection::DownLeft => 4,
            VertexDirection::UpLeft => 5,
        }
    }
}

/// Vertex associated with hexagon grids.
///
/// A hexagonal vertex follows the same ruleset as axial coordinates with one exception.
///
/// It needs to know its `spin`. Spin correlates to which side [`VertexSpin::Up`] or [`VertexSpin::Down`]
/// has two hexagons.
///
/// See [`vertex`] for helper macro to instantiate these structs.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Default)]
pub struct Vertex {
    pub coord: Triangle,
}

/// Helper macro to create [`Vertex`] structs.
#[macro_export]
macro_rules! vertex {
    ($q:expr, $r:expr) => {{
        use $crate::triangle::coordinate::{triangle, Triangle};
        Vertex {
            coord: triangle!($q, $r),
        }
    }};
}
pub use vertex;

impl From<VertexDirection> for Vertex {
    fn from(value: VertexDirection) -> Self {
        match value {
            VertexDirection::Up => vertex!(0, -1),
            VertexDirection::UpRight => vertex!(1, 0),
            VertexDirection::DownRight => vertex!(0, 1),
            VertexDirection::Down => vertex!(-1, 2),
            VertexDirection::DownLeft => vertex!(-2, 1),
            VertexDirection::UpLeft => vertex!(-1, 0),
        }
    }
}

impl Neg for VertexDirection {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            VertexDirection::Up => VertexDirection::Down,
            VertexDirection::UpRight => VertexDirection::DownLeft,
            VertexDirection::DownRight => VertexDirection::UpLeft,
            VertexDirection::Down => VertexDirection::Up,
            VertexDirection::DownLeft => VertexDirection::UpRight,
            VertexDirection::UpLeft => VertexDirection::DownRight,
        }
    }
}

impl From<Axial> for Vertex {
    fn from(value: Axial) -> Self {
        let x = match value.r <= 0 {
            true => (2 * value.q) - 1, // self.r <= 0
            false => 2 * value.q,      // self.r > 0
        };

        let y = match value.r > 0 {
            true => (2 * value.r) - 1, // self.r > 0
            false => 2 * value.r,      // self.r <= 0
        };

        vertex!(x, y)
    }
}

impl From<Triangle> for Vertex {
    fn from(value: Triangle) -> Self {
        Vertex { coord: value }
    }
}

impl Vertex {
    /// Get all 3 adjacent hexes to this vertex.
    ///
    /// # Example
    /// ```
    /// use gridava::hex::vertex::{Vertex, VertexDirection};
    /// use gridava::hex::coordinate::{axial, Axial};
    ///
    /// let coords = axial!(0,0).vertex(VertexDirection::Down).adjacent_hexes();
    /// ```
    pub fn adjacent_hexes(&self) -> Option<[Axial; 3]> {
        if let Some((coord, spin)) = self.try_to_axial() {
            match spin {
                VertexSpin::Up => Some([
                    axial!(coord.q, coord.r),
                    axial!(coord.q, coord.r - 1),
                    axial!(coord.q + 1, coord.r - 1),
                ]),
                VertexSpin::Down => Some([
                    axial!(coord.q, coord.r),
                    axial!(coord.q, coord.r + 1),
                    axial!(coord.q - 1, coord.r + 1),
                ]),
            }
        } else {
            None
        }
    }

    pub fn into_inner(&self) -> &Triangle {
        &self.coord
    }

    fn tri_to_axial(&self) -> Axial {
        let q = match self.coord.y <= 0 {
            true => (self.coord.x + 1) / 2, // self.y <= 0
            false => self.coord.x / 2,      // self.y > 0
        };

        let r = match self.coord.y > 0 {
            true => (self.coord.y + 1) / 2,
            false => self.coord.y / 2,
        };

        axial!(q, r)
    }

    pub fn try_to_axial(&self) -> Option<(Axial, VertexSpin)> {
        if !self.coord.is_triangle_face() {
            None
        } else {
            match (self.coord.orientation(), self.coord.y >= 0) {
                (TriOrientation::Up, true) => Some((
                    (*self + -Vertex::from(VertexDirection::Down)).tri_to_axial(),
                    VertexSpin::Down,
                )),
                (TriOrientation::Up, false) => Some((
                    (*self + Vertex::from(VertexDirection::Up)).tri_to_axial(),
                    VertexSpin::Down,
                )),
                (TriOrientation::Down, true) => Some((
                    (*self + -Vertex::from(VertexDirection::Up)).tri_to_axial(),
                    VertexSpin::Up,
                )),
                (TriOrientation::Down, false) => Some((
                    (*self + Vertex::from(VertexDirection::Down)).tri_to_axial(),
                    VertexSpin::Up,
                )),
            }
        }
    }

    /// Get all 3 adjacent vertices to this vertex.
    ///
    /// # Example
    /// ```
    /// use gridava::hex::vertex::{Vertex, VertexDirection};
    /// use gridava::hex::coordinate::{axial, Axial};
    ///
    /// let vertices = axial!(0,0).vertex(VertexDirection::Down).adjacent_vertices();
    /// ```
    pub fn adjacent_vertices(&self) -> Option<[Self; 3]> {
        if !self.coord.is_triangle_face() {
            None
        } else {
            let neighbors = self.coord.neighbors();
            Some([
                neighbors[0].into(),
                neighbors[1].into(),
                neighbors[2].into(),
            ])
        }
    }

    /// Generate the edges adjacent to this vertex.
    ///
    /// ```
    /// use gridava::hex::vertex::{Vertex, VertexDirection};
    /// use gridava::hex::coordinate::{axial, Axial};
    ///
    /// let edges = axial!(0,0).vertex(VertexDirection::Down).adjacent_edges();
    /// ```
    pub fn adjacent_edges(&self) -> Option<[Edge; 3]> {
        if let Some((coord, spin)) = self.try_to_axial() {
            match spin {
                VertexSpin::Up => Some([
                    edge!(coord.q + 1, coord.r - 1, EdgeDirection::West),
                    edge!(coord.q, coord.r, EdgeDirection::NorthEast),
                    edge!(coord.q, coord.r, EdgeDirection::NorthWest),
                ]),
                VertexSpin::Down => Some([
                    edge!(coord.q, coord.r + 1, EdgeDirection::NorthWest),
                    edge!(coord.q, coord.r + 1, EdgeDirection::West),
                    edge!(coord.q - 1, coord.r + 1, EdgeDirection::NorthEast),
                ]),
            }
        } else {
            None
        }
    }

    /// Compute the L1 distance between two vertices.
    ///
    /// ```
    /// use gridava::hex::vertex::{Vertex, VertexDirection};
    /// use gridava::hex::coordinate::{axial, Axial};
    ///
    /// let verta = axial!(0,0).vertex(VertexDirection::Up);
    /// let vertb = axial!(1,0).vertex(VertexDirection::Up);
    ///
    /// let dist = verta.distance(vertb);
    /// ```
    #[inline]
    pub fn distance(&self, b: Self) -> i32 {
        self.coord.distance(b.coord)
    }
}

impl Add for Vertex {
    type Output = Self;

    fn add(self, rhs: Vertex) -> Self::Output {
        Vertex {
            coord: self.coord + rhs.coord,
        }
    }
}

impl AddAssign for Vertex {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Vertex {
    type Output = Vertex;

    fn sub(self, rhs: Self) -> Self::Output {
        Vertex {
            coord: self.coord - rhs.coord,
        }
    }
}

impl SubAssign for Vertex {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<T> Mul<T> for Vertex
where
    i32: Mul<T, Output = i32>,
    T: Copy,
{
    type Output = Vertex;

    fn mul(self, rhs: T) -> Self::Output {
        Vertex {
            coord: self.coord * rhs,
        }
    }
}

impl Div<i32> for Vertex {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        Vertex {
            coord: self.coord / rhs,
        }
    }
}

impl Neg for Vertex {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vertex { coord: -self.coord }
    }
}

#[cfg(test)]
mod tests {

    use std::println;

    use super::*;

    #[test]
    fn tri_to_axial() {
        macro_rules! test {
            ($a:expr) => {
                let vert = Vertex::from($a);
                assert_eq!($a, vert.tri_to_axial());
            };
        }
        for q in -100..=100 {
            for r in -100..=100 {
                test!(axial!(q, r));
            }
        }
    }

    #[test]
    fn from_axial() {
        assert_eq!(Vertex::from(axial!(0, 0)), vertex!(-1, 0));

        assert_eq!(Vertex::from(axial!(1, 0)), vertex!(1, 0));
        assert_eq!(Vertex::from(axial!(2, 0)), vertex!(3, 0));
        assert_eq!(Vertex::from(axial!(3, 0)), vertex!(5, 0));

        assert_eq!(Vertex::from(axial!(0, 1)), vertex!(0, 1));
        assert_eq!(Vertex::from(axial!(0, 2)), vertex!(0, 3));
        assert_eq!(Vertex::from(axial!(0, 3)), vertex!(0, 5));

        assert_eq!(Vertex::from(axial!(1, 1)), vertex!(2, 1));
        assert_eq!(Vertex::from(axial!(1, 2)), vertex!(2, 3));
        assert_eq!(Vertex::from(axial!(2, 2)), vertex!(4, 3));
    }

    #[test]
    fn adjacent_hexes() {
        println!("{:?}", axial!(-1, 0).vertex(VertexDirection::UpRight));
        assert_eq!(
            axial!(-1, 0)
                .vertex(VertexDirection::UpRight)
                .adjacent_hexes()
                .unwrap(),
            [axial!(0, -1), axial!(0, 0), axial!(-1, 0)]
        );
        assert_eq!(
            vertex!(0, -2).adjacent_hexes().unwrap(),
            [axial!(0, 0), axial!(0, -1), axial!(1, -1)]
        );
        assert_eq!(
            vertex!(1, 1).adjacent_hexes().unwrap(),
            [axial!(1, 0), axial!(1, 1), axial!(0, 1)]
        );
    }

    #[test]
    fn adjacent_vertices() {
        assert_eq!(
            vertex!(0, 0).adjacent_vertices().unwrap(),
            [vertex!(-1, 1), vertex!(1, 1), vertex!(1, -1)]
        );
        assert_eq!(
            vertex!(1, 1).adjacent_vertices().unwrap(),
            [vertex!(0, 0), vertex!(2, 0), vertex!(0, 2)]
        );
        assert_eq!(
            vertex!(1, -1).adjacent_vertices().unwrap(),
            [vertex!(0, -2), vertex!(2, -2), vertex!(0, 0)]
        );
    }

    #[test]
    fn adjacent_edges() {
        assert_eq!(
            vertex!(0, -2).adjacent_edges().unwrap(),
            [
                edge!(1, -1, EdgeDirection::West),
                edge!(0, 0, EdgeDirection::NorthEast),
                edge!(0, 0, EdgeDirection::NorthWest),
            ]
        );

        assert_eq!(
            vertex!(-1, 1).adjacent_edges().unwrap(),
            [
                edge!(0, 1, EdgeDirection::NorthWest),
                edge!(0, 1, EdgeDirection::West),
                edge!(-1, 1, EdgeDirection::NorthEast),
            ]
        );
    }

    #[test]
    fn distance() {
        assert_eq!(vertex!(0, 0).distance(vertex!(0, 0)), 0);

        assert_eq!(vertex!(0, -2).distance(vertex!(-1, 1)), 3);

        assert_eq!(vertex!(0, -2).distance(vertex!(2, 0)), 4);

        assert_eq!(
            axial!(-1, 0)
                .vertex(VertexDirection::Up)
                .distance(axial!(1, 1).vertex(VertexDirection::Down)),
            7
        );

        assert_eq!(
            axial!(0, 0)
                .vertex(VertexDirection::Down)
                .distance(axial!(0, 1).vertex(VertexDirection::Up)),
            1
        );

        assert_eq!(
            axial!(0, 0)
                .vertex(VertexDirection::Down)
                .distance(axial!(0, 1).vertex(VertexDirection::Down)),
            2
        );

        assert_eq!(
            axial!(0, 0)
                .vertex(VertexDirection::Down)
                .distance(axial!(1, -1).vertex(VertexDirection::Up)),
            5
        );

        assert_eq!(
            axial!(0, 0)
                .vertex(VertexDirection::Up)
                .distance(axial!(2, -1).vertex(VertexDirection::Up)),
            4
        );

        assert_eq!(
            axial!(-1, 0)
                .vertex(VertexDirection::Up)
                .distance(axial!(1, 1).vertex(VertexDirection::Up)),
            6
        );
    }

    #[test]
    fn from_i32() {
        for i in 0..=5 {
            let vd = VertexDirection::from(i);
            assert_eq!(vd, i.into());
        }
    }

    #[test]
    fn from_vd_i32() {
        assert_eq!(i32::from(VertexDirection::Up), 0);
        assert_eq!(i32::from(VertexDirection::UpRight), 1);
        assert_eq!(i32::from(VertexDirection::DownRight), 2);
        assert_eq!(i32::from(VertexDirection::Down), 3);
        assert_eq!(i32::from(VertexDirection::DownLeft), 4);
        assert_eq!(i32::from(VertexDirection::UpLeft), 5);
    }

    #[test]
    fn from_vd() {
        assert_eq!(Vertex::from(VertexDirection::Up), vertex!(0, -1));
        assert_eq!(Vertex::from(VertexDirection::UpRight), vertex!(1, 0));
        assert_eq!(Vertex::from(VertexDirection::DownRight), vertex!(0, 1));
        assert_eq!(Vertex::from(VertexDirection::Down), vertex!(-1, 2));
        assert_eq!(Vertex::from(VertexDirection::DownLeft), vertex!(-2, 1));
        assert_eq!(Vertex::from(VertexDirection::UpLeft), vertex!(-1, 0));

        // Neg
        assert_eq!(
            Vertex::from(VertexDirection::Up),
            Vertex::from(-VertexDirection::Down)
        );
        assert_eq!(
            Vertex::from(VertexDirection::UpRight),
            Vertex::from(-VertexDirection::DownLeft)
        );
        assert_eq!(
            Vertex::from(VertexDirection::DownRight),
            Vertex::from(-VertexDirection::UpLeft)
        );
        assert_eq!(
            Vertex::from(VertexDirection::Down),
            Vertex::from(-VertexDirection::Up)
        );
        assert_eq!(
            Vertex::from(VertexDirection::DownLeft),
            Vertex::from(-VertexDirection::UpRight)
        );
        assert_eq!(
            Vertex::from(VertexDirection::UpLeft),
            Vertex::from(-VertexDirection::DownRight)
        );
    }

    #[test]
    fn default() {
        assert_eq!(Vertex::default(), vertex!(0, 0));
    }
}
