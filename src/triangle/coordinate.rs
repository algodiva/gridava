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

    /// Solve for the third component of a tri-face coordinate.
    #[inline]
    pub fn solve_coord(partial: (i32, i32), orientation: TriOrientation) -> i32 {
        match orientation {
            TriOrientation::Up => 2 - partial.0 - partial.1,
            TriOrientation::Down => 1 - partial.0 - partial.1,
        }
    }

    /// Compute the z coordinate for a tri-face coordinate
    pub fn compute_z(&self, orientation: TriOrientation) -> Self {
        Triangle {
            z: Self::solve_coord((self.x, self.y), orientation),
            ..*self
        }
    }

    /// Converts a tri coordinate to cartesian coordinates.
    pub fn to_cartesian(&self) -> (f64, f64) {
        (
            (0.5 * self.x as f64 + -0.5 * self.z as f64),
            (-SQRT_3 / 6.0 * self.x as f64 + SQRT_3 / 3.0 * self.y as f64
                - SQRT_3 / 6.0 * self.z as f64),
        )
    }

    /// Converts from cartesian coordinates to the nearest tri face coordinate.
    pub fn nearest_tri_face(cartesian: (f64, f64)) -> Self {
        triangle!(
            (1.0 * cartesian.0 - SQRT_3 / 3.0 * cartesian.1).ceil() as i32,
            (SQRT_3 * 2.0 / 3.0 * cartesian.1).floor() as i32 + 1,
            (-1.0 * cartesian.0 - SQRT_3 / 3.0 * cartesian.1).ceil() as i32
        )
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

    /// Linear interpolation between two tri faces
    pub fn lerp(&self, b: &Self, t: f64) -> Self {
        // Until a method to do it can be found for native tri coords,
        // we'll just convert to cartesian and interpolate there.

        let (self_x, self_y) = self.to_cartesian();
        let (b_x, b_y) = b.to_cartesian();

        let x = crate::core::misc::lerp(self_x, b_x, t);
        let y = crate::core::misc::lerp(self_y, b_y, t);

        Self::nearest_tri_face((x, y))
    }

    /// Produces a line from self to b
    ///
    /// The elements of the array are in order of grid traversal, the length of the array
    /// will also equal the distance of self -> b + 1.
    #[cfg(any(feature = "std", feature = "alloc"))]
    pub fn line(&self, b: &Self) -> Vec<Self> {
        // Non-Inclusive
        let basic_line = |a: Triangle, b: Triangle, dist| {
            (0..dist).map(move |d| a.lerp(&b, d as f64 / dist as f64))
        };

        // If the coordinates share an axis, we can just return a basic lerped line
        if self.x == b.x || self.y == b.y || self.z == b.z {
            return basic_line(*self, *b, self.distance(b) + 1).collect();
        }

        // Otherwise, for a smooth line that follows standard tri movements
        // we have to do a different method
        let slope = (self.x - b.x) / (self.y - b.y);
        let (intersected, offsets) = match slope.is_negative() {
            // Negative slope
            true => {
                let (y, z) = match self.x < b.x {
                    true => (b.y, self.z),
                    false => (self.y, b.z),
                };

                (
                    [
                        triangle!(Self::solve_coord((y, z), TriOrientation::Up), y, z),
                        triangle!(Self::solve_coord((y, z), TriOrientation::Down), y, z),
                    ],
                    [triangle!(1, -1, 1), triangle!(1, -1, -1)],
                )
            }
            // Positive slope
            false => {
                let (x, y) = match self.x < b.x {
                    true => (b.x, self.y),
                    false => (self.x, b.y),
                };

                (
                    [
                        triangle!(x, y, Self::solve_coord((x, y), TriOrientation::Up)),
                        triangle!(x, y, Self::solve_coord((x, y), TriOrientation::Down)),
                    ],
                    [triangle!(-1, 1, -1), triangle!(-1, 1, 1)],
                )
            }
        };

        let ds0 = self.distance(&intersected[0]);
        let ds1 = self.distance(&intersected[1]);

        let db0 = b.distance(&intersected[0]);
        let db1 = b.distance(&intersected[1]);

        if ds0 < ds1 {
            // self -> 1 && b -> 0
            basic_line(*self, intersected[0], ds0)
                .chain(
                    intersected
                        .iter()
                        .enumerate()
                        .map(|(i, v)| *v + offsets[i])
                        .rev(),
                )
                .chain(basic_line(*b, intersected[1], db1).rev())
                .collect()
        } else {
            // b -> 0 && self -> 1
            basic_line(*self, intersected[1], ds1)
                .chain(intersected.iter().enumerate().map(|(i, v)| *v + offsets[i]))
                .chain(basic_line(*b, intersected[0], db0).rev())
                .collect()
        }
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
    use assert_float_eq::*;

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
    fn to_cartesian() {
        // Interoperability
        assert_eq!(
            Triangle::nearest_tri_face(triangle!(0, 1, 0).to_cartesian()),
            triangle!(0, 1, 0)
        );

        assert_eq!(
            Triangle::nearest_tri_face(triangle!(0, 2, -1).to_cartesian()),
            triangle!(0, 2, -1)
        );

        assert_eq!(
            Triangle::nearest_tri_face(triangle!(-1, 1, 2).to_cartesian()),
            triangle!(-1, 1, 2)
        );

        // Accuracy
        macro_rules! tupexpand {
            ($lhs:expr, $tup:expr) => {
                let lhs = $lhs;
                assert_f64_near!(lhs.0, $tup.0);
                assert_f64_near!(lhs.1, $tup.1);
            };
        }
        tupexpand!(triangle!(0, 1, 0).to_cartesian(), (0.0, 0.5773502691896262));
        tupexpand!(
            triangle!(0, 2, -1).to_cartesian(),
            (0.5, 1.4433756729740643)
        );
        tupexpand!(
            triangle!(0, 0, 2).to_cartesian(),
            (-1.0, -0.5773502691896257)
        );
    }

    #[test]
    fn nearest_tri_face() {
        assert_eq!(Triangle::nearest_tri_face((0.0, 0.0)), triangle!(0, 1, 0));
        assert_eq!(
            Triangle::nearest_tri_face((-1.0, -1.0)),
            triangle!(0, -1, 2)
        );
        assert_eq!(Triangle::nearest_tri_face((1.0, -1.0)), triangle!(2, -1, 0));
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
    fn lerp() {
        assert_eq!(
            triangle!(-1, 1, 1).lerp(&triangle!(1, 1, 0), 0.51),
            triangle!(0, 1, 0)
        );

        assert_eq!(
            triangle!(-1, 1, 1).lerp(&triangle!(1, 1, 0), 1.0),
            triangle!(1, 1, 0)
        );

        assert_eq!(
            triangle!(-1, 1, 1).lerp(&triangle!(2, 0, -1), 0.63),
            triangle!(1, 1, 0)
        );

        assert_eq!(
            triangle!(-1, 1, 1).lerp(&triangle!(2, 0, -1), 0.3),
            triangle!(0, 1, 1)
        );
    }

    #[test]
    fn line() {
        let tria = triangle!(-1, 0, 2);
        let trib = triangle!(2, 1, -1);

        todo!()
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
