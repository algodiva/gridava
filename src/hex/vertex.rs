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
/// A hexagonal vertex is aligned to a triangular grid with the origin tri(0,0) at vertex hex(0,0) [`VertexDirection::DownRight`].
///
/// The triangular grid used for hexagon vertices also allows for hexagon centers to be used.
///
/// To convert from axial to a vertex use the member function [`Axial::vertex()`].
///
/// To convert to axial from a vertex, use the member function [`Vertex::try_to_axial()`].
///
/// See [`Triangle`] for more information.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Default)]
pub struct Vertex {
    /// Wrapped triangle coordinate struct used for hex vertices.
    pub coord: Triangle,
}

/// Helper macro to create [`Vertex`] structs.
#[macro_export]
macro_rules! vertex {
    ($q:expr, $r:expr, $s:expr) => {{
        use $crate::triangle::coordinate::{triangle, Triangle};
        Vertex {
            coord: triangle!($q, $r, $s),
        }
    }};
}
pub use vertex;

impl From<VertexDirection> for Vertex {
    fn from(value: VertexDirection) -> Self {
        match value {
            VertexDirection::Up => vertex!(1, 0, 1),
            VertexDirection::UpRight => vertex!(1, 0, 0),
            VertexDirection::DownRight => vertex!(1, 1, 0),
            VertexDirection::Down => vertex!(0, 1, 0),
            VertexDirection::DownLeft => vertex!(0, 1, 1),
            VertexDirection::UpLeft => vertex!(0, 0, 1),
        }
    }
}

impl From<(Axial, VertexDirection)> for Vertex {
    fn from(value: (Axial, VertexDirection)) -> Self {
        let vert_dir: Vertex = VertexDirection::into(value.1);

        vertex!(
            value.0.q + vert_dir.coord.x,
            value.0.r + vert_dir.coord.y,
            value.0.compute_s() + vert_dir.coord.z
        )
    }
}

impl From<Axial> for Vertex {
    fn from(value: Axial) -> Self {
        vertex!(value.q, value.r, value.compute_s())
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

    /// Convert to [`Triangle`]
    ///
    /// # Example
    /// ```
    /// use gridava::hex::vertex::{vertex, Vertex};
    /// use gridava::triangle::coordinate::Triangle;
    ///
    /// let tri = vertex!(0, 0, 0).into_inner();
    /// ```
    pub fn into_inner(&self) -> &Triangle {
        &self.coord
    }

    /// Try to convert to an [`Axial`] coordinate representation.
    ///
    /// Produces [`None`] if the coordinate is not a tri face according to [`Triangle::is_tri_face()`]
    ///
    /// # Example
    /// ```
    /// use gridava::hex::vertex::{vertex, Vertex, VertexDirection, VertexSpin};
    /// use gridava::hex::coordinate::{axial, Axial};
    ///
    /// assert!(vertex!(0, 0, 0).try_to_axial().is_none());
    /// assert_eq!(vertex!(1, 1, 0).try_to_axial().unwrap(), (axial!(0, 1), VertexSpin::Up));
    /// ```
    pub fn try_to_axial(&self) -> Option<(Axial, VertexSpin)> {
        if self.coord.is_tri_face() {
            match self.coord.orientation() {
                TriOrientation::Up => {
                    Some((axial!(self.coord.x - 1, self.coord.y), VertexSpin::Up))
                }
                TriOrientation::Down => {
                    Some((axial!(self.coord.x, self.coord.y - 1), VertexSpin::Down))
                }
            }
        } else {
            None
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
        if self.coord.is_tri_face() {
            let neighbors = self.coord.neighbors();
            Some([
                neighbors[0].into(),
                neighbors[1].into(),
                neighbors[2].into(),
            ])
        } else {
            None
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
    pub fn distance(self, b: Self) -> u32 {
        self.coord.distance(b.coord)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_axial() {
        assert_eq!(Vertex::from(axial!(0, 0)), vertex!(0, 0, 0));

        assert_eq!(Vertex::from(axial!(1, 0)), vertex!(1, 0, -1));
        assert_eq!(Vertex::from(axial!(2, 0)), vertex!(2, 0, -2));
        assert_eq!(Vertex::from(axial!(3, 0)), vertex!(3, 0, -3));

        assert_eq!(Vertex::from(axial!(0, 1)), vertex!(0, 1, -1));
        assert_eq!(Vertex::from(axial!(0, 2)), vertex!(0, 2, -2));
        assert_eq!(Vertex::from(axial!(0, 3)), vertex!(0, 3, -3));

        assert_eq!(Vertex::from(axial!(1, 1)), vertex!(1, 1, -2));
        assert_eq!(Vertex::from(axial!(1, 2)), vertex!(1, 2, -3));
        assert_eq!(Vertex::from(axial!(2, 2)), vertex!(2, 2, -4));
    }

    #[test]
    fn into_inner() {
        assert_eq!(*vertex!(0, 0, 0).into_inner(), triangle!(0, 0, 0));
    }

    #[test]
    fn try_to_axial() {
        assert_eq!(
            vertex!(1, 1, 0).try_to_axial().unwrap(),
            (axial!(0, 1), VertexSpin::Up)
        );
        assert_eq!(
            vertex!(1, 0, 0).try_to_axial().unwrap(),
            (axial!(1, -1), VertexSpin::Down)
        );
        assert_eq!(
            vertex!(1, 0, 1).try_to_axial().unwrap(),
            (axial!(0, 0), VertexSpin::Up)
        );
        assert_eq!(
            vertex!(0, 1, 0).try_to_axial().unwrap(),
            (axial!(0, 0), VertexSpin::Down)
        );
    }

    #[test]
    fn adjacent_hexes() {
        assert_eq!(
            axial!(-1, 0)
                .vertex(VertexDirection::UpRight)
                .adjacent_hexes()
                .unwrap(),
            [axial!(0, -1), axial!(0, 0), axial!(-1, 0)]
        );
        assert_eq!(
            vertex!(1, 0, 1).adjacent_hexes().unwrap(),
            [axial!(0, 0), axial!(0, -1), axial!(1, -1)]
        );
        assert_eq!(
            vertex!(1, 1, -1).adjacent_hexes().unwrap(),
            [axial!(1, 0), axial!(1, 1), axial!(0, 1)]
        );

        assert!(vertex!(0, 0, 0).adjacent_hexes().is_none());
    }

    #[test]
    fn adjacent_vertices() {
        assert_eq!(
            vertex!(1, 0, 0).adjacent_vertices().unwrap(),
            [vertex!(1, 0, 1), vertex!(2, 0, 0), vertex!(1, 1, 0)]
        );
        assert_eq!(
            vertex!(0, 1, 1).adjacent_vertices().unwrap(),
            [vertex!(-1, 1, 1), vertex!(0, 1, 0), vertex!(0, 0, 1)]
        );
        assert_eq!(
            vertex!(0, 2, 0).adjacent_vertices().unwrap(),
            [vertex!(-1, 2, 0), vertex!(0, 2, -1), vertex!(0, 1, 0)]
        );

        assert!(vertex!(0, 0, 0).adjacent_vertices().is_none());
    }

    #[test]
    fn adjacent_edges() {
        assert_eq!(
            vertex!(1, 0, 1).adjacent_edges().unwrap(),
            [
                edge!(1, -1, EdgeDirection::West),
                edge!(0, 0, EdgeDirection::NorthEast),
                edge!(0, 0, EdgeDirection::NorthWest),
            ]
        );

        assert_eq!(
            vertex!(0, 1, 0).adjacent_edges().unwrap(),
            [
                edge!(0, 1, EdgeDirection::NorthWest),
                edge!(0, 1, EdgeDirection::West),
                edge!(-1, 1, EdgeDirection::NorthEast),
            ]
        );

        assert!(vertex!(0, 0, 0).adjacent_edges().is_none());
    }

    #[test]
    fn distance() {
        assert_eq!(vertex!(0, 0, 0).distance(vertex!(0, 0, 0)), 0);

        assert_eq!(vertex!(0, 0, 1).distance(vertex!(1, 0, 0)), 2);

        assert_eq!(vertex!(-1, 1, 2).distance(vertex!(0, 2, -1)), 5);

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
        assert_eq!(Vertex::from(VertexDirection::Up), vertex!(1, 0, 1));
        assert_eq!(Vertex::from(VertexDirection::UpRight), vertex!(1, 0, 0));
        assert_eq!(Vertex::from(VertexDirection::DownRight), vertex!(1, 1, 0));
        assert_eq!(Vertex::from(VertexDirection::Down), vertex!(0, 1, 0));
        assert_eq!(Vertex::from(VertexDirection::DownLeft), vertex!(0, 1, 1));
        assert_eq!(Vertex::from(VertexDirection::UpLeft), vertex!(0, 0, 1));
    }

    #[test]
    fn default() {
        assert_eq!(Vertex::default(), vertex!(0, 0, 0));
    }
}
