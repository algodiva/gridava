//! Coordinate system for triangle based grids.

use crate::lib::*;

/// A coordinate for a triangular grid.
///
/// Maps a coordinate to every triangular face and vertex on a triangular grid.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Default)]
pub struct Triangle {
    /// X coordinate
    pub x: i32,
    /// Y coordinate
    pub y: i32,
    /// Z coordinate
    pub z: i32,
}

/// Helper macro to create [`Triangle`] structs.
#[macro_export]
macro_rules! triangle {
    ($x:expr, $y:expr, $z:expr) => {
        Triangle {
            x: $x,
            y: $y,
            z: $z,
        }
    };
}
pub use triangle;

/// Orientation of the tri-coordinate
///
/// - Up => the base is facing downwards.
/// - Down => base is facing upwards
/// - Vert => vertex of a triangle
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub enum TriOrientation {
    /// Upwards orientated triangle
    Up,
    /// Downwards orientated triangle
    Down,
}

impl From<Triangle> for TriOrientation {
    fn from(value: Triangle) -> Self {
        match value.x + value.y + value.z {
            sum if sum & 1 == 1 => TriOrientation::Down,
            _ => TriOrientation::Up,
        }
    }
}

/// Primary directions of travel on a triangular grid.
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

impl Triangle {
    /// Compute the z coordinate for a vertex coordinate
    ///
    /// An important distinction is made for this type of coordinate since for
    /// verts, the sum must equal 0.
    pub fn compute_z_vert(&self) -> Self {
        Triangle {
            z: -self.x - self.y,
            ..*self
        }
    }

    /// Compute the z coordinate for a tri-face coordinate
    pub fn compute_z(&self, orientation: TriOrientation) -> Self {
        match orientation {
            TriOrientation::Up => Triangle {
                z: 2 - self.x - self.y,
                ..*self
            },
            TriOrientation::Down => Triangle {
                z: 1 - self.x - self.y,
                ..*self
            },
        }
    }
    /// Determines if the coordinate is a face.
    ///
    /// Since the coordinates can map to faces or vertices it can be
    /// beneficial to check if it is a face or not.
    pub fn is_tri_face(&self) -> bool {
        (self.x + self.y + self.z) != 0
    }

    /// Determines the orientation
    pub fn orientation(&self) -> TriOrientation {
        (*self).into()
    }

    /// Generate a neighbor coordinate
    pub fn neighbor(&self, direction: TriDirection) -> Self {
        match (direction, self.orientation()) {
            (TriDirection::Left, TriOrientation::Up) => triangle!(self.x - 1, self.y, self.z),
            (TriDirection::Base, TriOrientation::Up) => triangle!(self.x, self.y - 1, self.z),
            (TriDirection::Right, TriOrientation::Up) => triangle!(self.x, self.y, self.z - 1),
            (TriDirection::Left, TriOrientation::Down) => triangle!(self.x, self.y, self.z + 1),
            (TriDirection::Base, TriOrientation::Down) => triangle!(self.x, self.y + 1, self.z),
            (TriDirection::Right, TriOrientation::Down) => {
                triangle!(self.x + 1, self.y, self.z)
            }
        }
    }

    /// Generates the neighboring coordinates
    pub fn neighbors(&self) -> [Self; 3] {
        [
            self.neighbor(TriDirection::Left),
            self.neighbor(TriDirection::Right),
            self.neighbor(TriDirection::Base),
        ]
    }

    /// Checks if all coordinates are neighbors
    pub fn are_neighbors(&self, coords: &[Self]) -> bool {
        let neighbors = self.neighbors();

        for coord in coords {
            if !neighbors.contains(coord) {
                return false;
            }
        }
        true
    }

    /// Computes L1 distance between coordinates
    pub fn distance(&self, b: Self) -> i32 {
        let dx = (self.x - b.x).abs();
        let dy = (self.y - b.y).abs();
        let dz = (self.z - b.z).abs();
        dx + dy + dz
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn are_neighbors() {
        assert!(triangle!(1, 0, 0).are_neighbors(&[
            triangle!(1, 1, 0),
            triangle!(2, 0, 0),
            triangle!(1, 0, 1)
        ]));
        assert!(!triangle!(0, 0, 1).are_neighbors(&[triangle!(2, 0, 0)]));
    }
}
