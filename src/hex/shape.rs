use std::ops::AddAssign;

use crate::core::transform::{Transform, Transformable};

use super::coordinate::Axial;

// A shape consists of coordinate vectors that define which hexes in relation to the origin are a part of the shape
#[derive(Clone, PartialOrd, PartialEq, Debug, Default)]
pub struct Shape<T: Copy + AddAssign + Transformable<T>> {
    tiles: Vec<T>, // Vector of hex vectors pointing to origin that comprise this shape.
    pub transform: Transform<T>, // The transformation matrix to convert from parent grid to local space.
}

// TODO
//      - scale shape? This will be a complicated task as to interpolate inbetween hexes

impl Shape<Axial> {
    pub fn new(hex_array: Option<Vec<Axial>>, transform: Option<Transform<Axial>>) -> Self {
        Self {
            tiles: hex_array.unwrap_or_default(),
            transform: transform.unwrap_or_default(),
        }
    }

    // Translate the shape in grid space.
    // coord: Vector defining translation direction and magnitude.
    pub fn translate(&mut self, coord: Axial) -> &Self {
        self.transform.translate(coord);
        self
    }

    // Rotates the shape about a specific point in grid space.
    // coord: A specific point to rotate about.
    // rot_dir: positive denotes CW, negative CCW, magnitude denotes how many 60 degree rotations.
    pub fn rotate_about(&mut self, coord: Axial, rot_dir: i32) -> &Self {
        self.transform.translation = self.transform.translation.rotate(Some(coord), rot_dir);
        self.transform.rotate(rot_dir);
        self
    }

    // Rotates the shape, either about it's local origin or some point.
    // coord: Some denotes about a specific point, None denotes around the shape's center
    // rot_dir: positive denotes CW, negative CCW, magnitude denotes how many 60 degree rotations.
    pub fn rotate(&mut self, coord: Option<Axial>, rot_dir: i32) -> &Self {
        match coord {
            Some(coord) => self.rotate_about(coord, rot_dir),
            None => {
                self.transform.rotate(rot_dir);
                self
            }
        }
    }

    // Retrieves the hex array in local space
    pub fn get_hexes(&self) -> Vec<Axial> {
        self.tiles.clone()
    }

    // Retrieves the hex array in grid space
    pub fn get_hexes_gs(&self) -> Vec<Axial> {
        self.tiles
            .iter()
            .map(|&coord| coord.apply_all(self.transform))
            .collect()
    }
}
