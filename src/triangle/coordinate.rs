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

    /// Rotate about the origin tri(0, 0, 0)
    ///
    /// Positive rot_dir means CW, negative is CCW
    pub fn rotate(&self, rot_dir: i32) -> Self {
        match rot_dir.rem_euclid(6) {
            0 => *self,
            1 => triangle!(1 - self.z, 1 - self.x, 1 - self.y),
            2 => triangle!(self.y, self.z, self.x),
            3 => triangle!(1 - self.x, 1 - self.y, 1 - self.z),
            4 => triangle!(self.z, self.x, self.y),
            5 => triangle!(1 - self.y, 1 - self.z, 1 - self.x),
            _ => unreachable!(), // should never reach
        }
    }

    /// Rotate a tri about another coordinate
    ///
    /// If rot_dir is a multiple of 2 a rotation of a tri face about another tri face
    /// will produce another tri face. However, a rot_dir that is odd with the same
    /// coordinates will produce half step gibberish.
    pub fn rotate_about(&self, about_b: &Self, rot_dir: i32) -> Self {
        *about_b + (*self - *about_b).rotate(rot_dir)
    }

    /// Reflect a tri across the cartesian x axis
    pub fn reflect_x(&self) -> Self {
        triangle!(self.z, self.y, self.x)
    }

    /// Reflect a tri across the cartesian y axis
    pub fn reflect_y(&self) -> Self {
        triangle!(1 - self.z, 1 - self.y, 1 - self.x)
    }

    /// Produce the coordinates within a set distance from this coordinate
    #[cfg(any(feature = "std", feature = "alloc"))]
    pub fn range(&self, dist: i32) -> Vec<Self> {
        let mut ret = Vec::with_capacity((dist.pow(2) + 2 * dist + 1) as usize);
        for dx in -dist..=dist {
            for dy in (-dist - dx).max(-dist)..=(dist - dx).min(dist) {
                let dz0 = 1 - (self.x + self.y + self.z + dx + dy);

                if dx.abs() + dy.abs() + dz0.abs() <= dist {
                    ret.push(*self + triangle!(dx, dy, dz0));
                }

                let dz1 = dz0 + 1;
                if dx.abs() + dy.abs() + dz1.abs() <= dist {
                    ret.push(*self + triangle!(dx, dy, dz1));
                }
            }
        }
        ret
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
    pub fn distance(&self, b: &Self) -> i32 {
        let dt = *self - *b;
        dt.x.abs() + dt.y.abs() + dt.z.abs()
    }
}

impl Add for Triangle {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Triangle {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Triangle {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Triangle {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_z_vert() {
        assert_eq!(triangle!(0, 0, 0).compute_z_vert(), triangle!(0, 0, 0));
        assert_eq!(triangle!(1, 0, 0).compute_z_vert(), triangle!(1, 0, -1));
    }

    #[test]
    fn compute_z() {
        assert_eq!(
            triangle!(1, 0, 0).compute_z(TriOrientation::Up),
            triangle!(1, 0, 1)
        );

        assert_eq!(
            triangle!(0, 0, 0).compute_z(TriOrientation::Down),
            triangle!(0, 0, 1)
        );
    }

    #[test]
    fn rotate() {
        assert_eq!(triangle!(0, 1, 0).rotate(2), triangle!(1, 0, 0));
        assert_eq!(triangle!(0, 1, 0).rotate(3), triangle!(1, 0, 1));
        assert_eq!(triangle!(0, 1, 0).rotate(4), triangle!(0, 0, 1));
        assert_eq!(triangle!(0, 1, 0).rotate(5), triangle!(0, 1, 1));
        assert_eq!(triangle!(0, 1, 0).rotate(6), triangle!(0, 1, 0).rotate(0));
    }

    #[test]
    fn rotate_about() {
        assert_eq!(
            triangle!(1, 1, 0).rotate_about(&triangle!(1, 0, -1), 1),
            triangle!(1, 1, -1)
        );
    }

    #[test]
    fn reflect_x() {
        assert_eq!(triangle!(1, 1, 0).reflect_x(), triangle!(0, 1, 1));
        assert_eq!(triangle!(2, 1, -1).reflect_x(), triangle!(-1, 1, 2));
        assert_eq!(triangle!(0, 1, 0).reflect_x(), triangle!(0, 1, 0));
    }

    #[test]
    fn reflect_y() {
        assert_eq!(triangle!(1, 1, 0).reflect_y(), triangle!(1, 0, 0));
        assert_eq!(triangle!(2, 1, -1).reflect_y(), triangle!(2, 0, -1));
        assert_eq!(triangle!(0, 1, 0).reflect_y(), triangle!(1, 0, 1));
    }

    #[test]
    fn range() {
        assert_eq!(
            triangle!(0, 1, 0).range(1),
            vec![
                triangle!(0, 1, 0),
                triangle!(0, 1, 1),
                triangle!(0, 2, 0),
                triangle!(1, 1, 0),
            ]
        );

        assert_eq!(
            triangle!(0, 0, 2).range(1),
            vec![
                triangle!(-1, 0, 2),
                triangle!(0, -1, 2),
                triangle!(0, 0, 1),
                triangle!(0, 0, 2),
            ]
        );

        assert_eq!(
            triangle!(1, 0, 0).range(2),
            vec![
                triangle!(0, 0, 1),
                triangle!(0, 1, 0),
                triangle!(1, -1, 1),
                triangle!(1, 0, 0),
                triangle!(1, 0, 1),
                triangle!(1, 1, -1),
                triangle!(1, 1, 0),
                triangle!(2, -1, 0),
                triangle!(2, 0, -1),
                triangle!(2, 0, 0),
            ]
        );
    }

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
