use super::coordinate::{axial, Axial};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Vertex spin is a orientation of the vertex.
///
/// A vertex needs to know its `spin`. Spin correlates to which side [`VertexSpin::Up`] or [`VertexSpin::Down`]
/// has two hexagons.
///
/// see [`Vertex`]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub enum VertexSpin {
    Up,
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
    Up,
    UpRight,
    DownRight,
    Down,
    DownLeft,
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
            _ => panic!(), // should never reach
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
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub struct Vertex {
    pub q: i32,
    pub r: i32,
    pub spin: VertexSpin,
}

/// Helper macro to create [`Vertex`] structs.
#[macro_export]
macro_rules! vertex {
    ($q:expr, $r:expr, $sp:expr) => {
        Vertex {
            q: $q,
            r: $r,
            spin: $sp,
        }
    };
}
pub use vertex;

impl Default for Vertex {
    fn default() -> Self {
        Self {
            q: 0,
            r: 0,
            spin: VertexSpin::Up,
        }
    }
}

impl From<VertexDirection> for Vertex {
    fn from(value: VertexDirection) -> Self {
        match value {
            VertexDirection::Up => vertex!(0, 0, VertexSpin::Up),
            VertexDirection::UpRight => vertex!(1, -1, VertexSpin::Down),
            VertexDirection::DownRight => vertex!(0, 1, VertexSpin::Up),
            VertexDirection::Down => vertex!(0, 0, VertexSpin::Down),
            VertexDirection::DownLeft => vertex!(-1, 1, VertexSpin::Up),
            VertexDirection::UpLeft => vertex!(0, -1, VertexSpin::Down),
        }
    }
}

impl Vertex {
    /// Get all 3 adjacent hexes to this vertex.
    ///
    /// # Example
    /// ```
    /// use gridava::hex::coordinate::{Axial, Vertex, VertexSpin, vertex};
    ///
    /// let coords = vertex!(2, 0, VertexSpin::Down).adjacent_hexes();
    /// ```
    pub fn adjacent_hexes(&self) -> [Axial; 3] {
        if self.spin == VertexSpin::Up {
            [
                axial!(self.q, self.r),
                axial!(self.q, self.r - 1),
                axial!(self.q + 1, self.r - 1),
            ]
        } else {
            // Spin down
            [
                axial!(self.q, self.r),
                axial!(self.q, self.r + 1),
                axial!(self.q - 1, self.r + 1),
            ]
        }
    }

    pub fn adjacent_vertices(&self) -> [Self; 3] {
        if self.spin == VertexSpin::Up {
            [
                vertex!(self.q + 1, self.r - 1, VertexSpin::Down),
                vertex!(self.q, self.r - 1, VertexSpin::Down),
                vertex!(self.q + 1, self.r - 2, VertexSpin::Down),
            ]
        } else {
            [
                vertex!(self.q, self.r + 1, VertexSpin::Up),
                vertex!(self.q - 1, self.r + 2, VertexSpin::Up),
                vertex!(self.q - 1, self.r + 1, VertexSpin::Up),
            ]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{axial, Axial};
    use super::{Vertex, VertexSpin};

    #[test]
    pub fn adjacent_hexes() {
        assert_eq!(
            vertex!(0, 0, VertexSpin::Down).adjacent_hexes(),
            [axial!(0, 0), axial!(0, 1), axial!(-1, 1)]
        );
        assert_eq!(
            vertex!(0, 0, VertexSpin::Up).adjacent_hexes(),
            [axial!(0, 0), axial!(0, -1), axial!(1, -1)]
        );
        assert_eq!(
            vertex!(1, 0, VertexSpin::Down).adjacent_hexes(),
            [axial!(1, 0), axial!(1, 1), axial!(0, 1)]
        );
    }

    #[test]
    pub fn adjacent_vertices() {
        assert_eq!(
            vertex!(0, 0, VertexSpin::Down).adjacent_vertices(),
            [
                vertex!(0, 1, VertexSpin::Up),
                vertex!(-1, 2, VertexSpin::Up),
                vertex!(-1, 1, VertexSpin::Up)
            ]
        );
        assert_eq!(
            vertex!(0, 0, VertexSpin::Up).adjacent_vertices(),
            [
                vertex!(1, -1, VertexSpin::Down),
                vertex!(0, -1, VertexSpin::Down),
                vertex!(1, -2, VertexSpin::Down)
            ]
        );
        assert_eq!(
            vertex!(1, 0, VertexSpin::Down).adjacent_vertices(),
            [
                vertex!(1, 1, VertexSpin::Up),
                vertex!(0, 2, VertexSpin::Up),
                vertex!(0, 1, VertexSpin::Up)
            ]
        );
    }
}
