//! Handles edges in a hexagonal grid.

// use crate::vertex;

use super::{
    coordinate::{axial, Axial, HexDirection},
    vertex::{vertex, Vertex, VertexSpin},
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub enum EdgeDirection {
    West,
    NorthWest,
    NorthEast,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub struct Edge {
    pub q: i32,
    pub r: i32,
    pub dir: EdgeDirection,
}

/// Helper macro to create [`Edge`] structs.
#[macro_export]
macro_rules! edge {
    ($q:expr, $r:expr, $dir:expr) => {
        Edge {
            q: $q,
            r: $r,
            dir: $dir,
        }
    };
}
pub use edge;

impl Default for Edge {
    fn default() -> Self {
        Self {
            q: 0,
            r: 0,
            dir: EdgeDirection::West,
        }
    }
}

impl From<HexDirection> for Edge {
    fn from(value: HexDirection) -> Self {
        match value {
            HexDirection::Front => edge!(1, 0, EdgeDirection::West),
            HexDirection::FrontRight => edge!(0, 1, EdgeDirection::NorthWest),
            HexDirection::BackRight => edge!(-1, 1, EdgeDirection::NorthEast),
            HexDirection::Back => edge!(0, 0, EdgeDirection::West),
            HexDirection::BackLeft => edge!(0, 0, EdgeDirection::NorthWest),
            HexDirection::FrontLeft => edge!(0, 0, EdgeDirection::NorthEast),
        }
    }
}

impl Edge {
    /// Get the adjacent hexes that share this edge.
    ///
    /// The first coordinate in the array will always be the (q, r) coordinate.
    ///
    /// # Example
    /// ```
    /// use gridava::hex::edge::{Edge, edge, EdgeDirection};
    /// use gridava::hex::coordinate::Axial;
    ///
    /// let coords = edge!(0, 0, EdgeDirection::West).adjacent_hexes();
    /// ```
    pub fn adjacent_hexes(&self) -> [Axial; 2] {
        match self.dir {
            EdgeDirection::West => [axial!(self.q, self.r), axial!(self.q - 1, self.r)],
            EdgeDirection::NorthWest => [axial!(self.q, self.r), axial!(self.q, self.r - 1)],
            EdgeDirection::NorthEast => [axial!(self.q, self.r), axial!(self.q + 1, self.r - 1)],
        }
    }

    pub fn adjacent_edges(&self) -> [Self; 4] {
        match self.dir {
            EdgeDirection::West => [
                edge!(self.q - 1, self.r + 1, EdgeDirection::NorthEast),
                edge!(self.q, self.r, EdgeDirection::NorthWest),
                edge!(self.q - 1, self.r + 1, EdgeDirection::NorthWest),
                edge!(self.q - 1, self.r, EdgeDirection::NorthEast),
            ],
            EdgeDirection::NorthWest => [
                edge!(self.q + 1, self.r - 1, EdgeDirection::West),
                edge!(self.q, self.r, EdgeDirection::NorthEast),
                edge!(self.q, self.r, EdgeDirection::West),
                edge!(self.q - 1, self.r, EdgeDirection::NorthEast),
            ],
            EdgeDirection::NorthEast => [
                edge!(self.q + 1, self.r, EdgeDirection::NorthWest),
                edge!(self.q + 1, self.r, EdgeDirection::West),
                edge!(self.q, self.r, EdgeDirection::NorthWest),
                edge!(self.q + 1, self.r - 1, EdgeDirection::West),
            ],
        }
    }

    /// Get the endpoints to this edge.
    ///
    /// The first vertex will always be the left-most going clockwise.
    ///
    /// # Example
    /// ```
    /// use gridava::hex::edge::{Edge, edge, EdgeDirection};
    /// use gridava::hex::vertex::Vertex;
    /// use gridava::hex::coordinate::Axial;
    ///
    /// let vertices = edge!(0, 0, EdgeDirection::West).endpoints();
    /// ```
    pub fn endpoints(&self) -> [Vertex; 2] {
        match self.dir {
            EdgeDirection::West => [
                vertex!(self.q - 1, self.r + 1, VertexSpin::Up),
                vertex!(self.q, self.r - 1, VertexSpin::Down),
            ],
            EdgeDirection::NorthWest => [
                vertex!(self.q, self.r - 1, VertexSpin::Down),
                vertex!(self.q, self.r, VertexSpin::Up),
            ],
            EdgeDirection::NorthEast => [
                vertex!(self.q, self.r, VertexSpin::Up),
                vertex!(self.q + 1, self.r - 1, VertexSpin::Down),
            ],
        }
    }

    /// Compute the distance between two edges.
    pub fn distance(&self, b: Self) -> i32 {
        self.endpoints()[0].distance(b.endpoints()[0])
    }
}
