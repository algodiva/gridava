use ndarray::{Array2, ArrayView2};

use crate::core::{
    tile::Tile,
    transform::{Float2D, Transform},
};

use super::coordinate::Axial;

// A shape consists of coordinate vectors that define which hexes in relation to the origin are a part of the shape
#[derive(Clone, PartialEq, Debug, Default)]
pub struct HexShape<T> {
    shape: Array2<Tile<T>>, // 2D array of tiles, index corresponds to coordinate (q, r). Origin of (0,0) to (+∞,+∞).
    pub transform: Transform<Axial>, // The transformation matrix to convert from parent grid to local space.
}

// TODO
//      - scale shape? This will be a complicated task as to interpolate inbetween hexes

impl<T> HexShape<T> {
    pub fn new(hex_array: Option<Array2<Tile<T>>>, transform: Option<Transform<Axial>>) -> Self
    where
        T: Default,
    {
        Self {
            shape: hex_array.unwrap_or_default(),
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

    pub fn resize(&mut self, scale: Float2D<f32>) -> &Self {
        // trilinear resizing

        // let x_ratio = if scale.x > 1.0 {
        //     (self.transform.scale.x - 1.0) / (scale.x - 1.0)
        // } else {
        //     0.0
        // };
        // let y_ratio = if scale.y > 1.0 {
        //     (self.transform.scale.y - 1.0) / (scale.y - 1.0)
        // } else {
        //     0.0
        // };

        todo!()
    }

    // Bake the transform into localspace
    pub fn bake_transform(&mut self) -> HexShape<T> {
        todo!()
    }

    // Retrieves the hex array in local space
    pub fn get_hexes(&self) -> ArrayView2<Tile<T>> {
        self.shape.view()
    }
}
