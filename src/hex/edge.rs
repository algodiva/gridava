//! Handles edges in a hexagonal grid.

use crate::lib::*;

use super::{
    coordinate::{axial, Axial, HexDirection},
    vertex::{Vertex, VertexDirection},
};

/// Orientation of an edge.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub enum EdgeDirection {
    /// West of the hex.
    West,
    /// North West of the hex
    NorthWest,
    /// North East of the hex
    NorthEast,
}

/// A hexagonal edge.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub struct Edge {
    /// q (x) coordinate
    pub q: i32,
    /// r (y) coordinate
    pub r: i32,
    /// Edge direction
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

    /// Get the adjacent edges to this edge.
    ///
    /// ```
    /// use gridava::hex::edge::{Edge, EdgeDirection, edge};
    ///
    /// let edges = edge!(0,0, EdgeDirection::West).adjacent_edges();
    /// ```
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
                axial!(self.q, self.r).vertex(VertexDirection::DownLeft),
                axial!(self.q, self.r).vertex(VertexDirection::UpLeft),
            ],
            EdgeDirection::NorthWest => [
                axial!(self.q, self.r).vertex(VertexDirection::UpLeft),
                axial!(self.q, self.r).vertex(VertexDirection::Up),
            ],
            EdgeDirection::NorthEast => [
                axial!(self.q, self.r).vertex(VertexDirection::Up),
                axial!(self.q, self.r).vertex(VertexDirection::UpRight),
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        assert_eq!(Edge::default(), edge!(0, 0, EdgeDirection::West));
    }

    #[test]
    fn from_hex_dir() {
        assert_eq!(
            Edge::from(HexDirection::Front),
            edge!(1, 0, EdgeDirection::West)
        );

        assert_eq!(
            Edge::from(HexDirection::FrontRight),
            edge!(0, 1, EdgeDirection::NorthWest)
        );

        assert_eq!(
            Edge::from(HexDirection::BackRight),
            edge!(-1, 1, EdgeDirection::NorthEast)
        );

        assert_eq!(
            Edge::from(HexDirection::Back),
            edge!(0, 0, EdgeDirection::West)
        );

        assert_eq!(
            Edge::from(HexDirection::BackLeft),
            edge!(0, 0, EdgeDirection::NorthWest)
        );

        assert_eq!(
            Edge::from(HexDirection::FrontLeft),
            edge!(0, 0, EdgeDirection::NorthEast)
        );
    }

    #[test]
    fn adjacent_hexes() {
        assert_eq!(
            edge!(0, 0, EdgeDirection::West).adjacent_hexes(),
            [axial!(0, 0), axial!(-1, 0)]
        );

        assert_eq!(
            edge!(0, 0, EdgeDirection::NorthWest).adjacent_hexes(),
            [axial!(0, 0), axial!(0, -1)]
        );

        assert_eq!(
            edge!(0, 0, EdgeDirection::NorthEast).adjacent_hexes(),
            [axial!(0, 0), axial!(1, -1)]
        );
    }

    #[test]
    fn adjacent_edges() {
        assert_eq!(
            edge!(0, 0, EdgeDirection::West).adjacent_edges(),
            [
                edge!(-1, 1, EdgeDirection::NorthEast),
                edge!(0, 0, EdgeDirection::NorthWest),
                edge!(-1, 1, EdgeDirection::NorthWest),
                edge!(-1, 0, EdgeDirection::NorthEast),
            ]
        );

        assert_eq!(
            edge!(0, 0, EdgeDirection::NorthWest).adjacent_edges(),
            [
                edge!(1, -1, EdgeDirection::West),
                edge!(0, 0, EdgeDirection::NorthEast),
                edge!(0, 0, EdgeDirection::West),
                edge!(-1, 0, EdgeDirection::NorthEast),
            ]
        );

        assert_eq!(
            edge!(0, 0, EdgeDirection::NorthEast).adjacent_edges(),
            [
                edge!(1, 0, EdgeDirection::NorthWest),
                edge!(1, 0, EdgeDirection::West),
                edge!(0, 0, EdgeDirection::NorthWest),
                edge!(1, -1, EdgeDirection::West),
            ]
        );
    }

    #[test]
    fn endpoints() {
        assert_eq!(
            edge!(0, 0, EdgeDirection::West).endpoints(),
            [
                axial!(0, 0).vertex(VertexDirection::DownLeft),
                axial!(0, 0).vertex(VertexDirection::UpLeft),
            ]
        );

        assert_eq!(
            edge!(0, 0, EdgeDirection::NorthWest).endpoints(),
            [
                axial!(0, 0).vertex(VertexDirection::UpLeft),
                axial!(0, 0).vertex(VertexDirection::Up),
            ]
        );

        assert_eq!(
            edge!(0, 0, EdgeDirection::NorthEast).endpoints(),
            [
                axial!(0, 0).vertex(VertexDirection::Up),
                axial!(0, 0).vertex(VertexDirection::UpRight),
            ]
        );

        assert_eq!(
            edge!(1, 0, EdgeDirection::NorthEast).endpoints(),
            [
                axial!(1, 0).vertex(VertexDirection::Up),
                axial!(1, 0).vertex(VertexDirection::UpRight),
            ]
        );
    }
}
