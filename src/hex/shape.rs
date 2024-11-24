use ndarray::{Array2, ArrayView2};

use crate::core::{
    tile::Tile,
    transform::{Float2D, Transform},
};

use super::coordinate::Axial;

// A shape consists of coordinate vectors that define which hexes in relation to the origin are a part of the shape
#[derive(Clone, PartialEq, Debug, Default)]
pub struct HexShape<T: Clone> {
    shape: Array2<Tile<T>>, // 2D array of tiles, index corresponds to coordinate (q, r). Origin of (0,0) to (+∞,+∞).
    pub transform: Transform<Axial>, // The transformation matrix to convert from parent grid to local space.
}

impl<T: Clone> HexShape<T> {
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
        self.transform.translation += coord;
        self
    }

    // Rotates the shape about a specific point in grid space.
    // coord: A specific point to rotate about.
    // rot_dir: positive denotes CW, negative CCW, magnitude denotes how many 60 degree rotations.
    pub fn rotate_about(&mut self, coord: Axial, rot_dir: i32) -> &Self {
        self.transform.translation = self.transform.translation.rotate(Some(coord), rot_dir);
        self.transform.rotation += rot_dir;
        self
    }

    // Rotates the shape, either about it's local origin or some point.
    // coord: Some denotes about a specific point, None denotes around the shape's center
    // rot_dir: positive denotes CW, negative CCW, magnitude denotes how many 60 degree rotations.
    pub fn rotate(&mut self, coord: Option<Axial>, rot_dir: i32) -> &Self {
        match coord {
            Some(coord) => self.rotate_about(coord, rot_dir),
            None => {
                self.transform.rotation += rot_dir;
                self
            }
        }
    }

    pub fn scale(&mut self, scale: Float2D<f32>) -> &Self {
        // Uses bilinear interpolation algorithm, it's lossless  meaning if you apply a scale and then it's inverse
        //  it will return to it's original shape.

        let shape = self.shape.shape();

        let new_x = (shape[0] as f32 * scale.x).round() as usize;
        let new_y = (shape[1] as f32 * scale.y).round() as usize;

        let mut new_arr = Array2::from_shape_simple_fn((new_x, new_y), &Tile::default);

        let x_ratio = (shape[0] as f32) / (new_x as f32);
        let y_ratio = (shape[1] as f32) / (new_y as f32);

        for y in 0..new_y {
            for x in 0..new_x {
                // Map new indices to old indices directly using ratios
                //  Using .round() here to preserve inverse scaling returns to input
                let src_x = (x as f32 * x_ratio).round() as usize;
                let src_y = (y as f32 * y_ratio).round() as usize;

                // Clamp indices to prevent out-of-bounds access
                let src_x_clamped = src_x.min(shape[0] - 1);
                let src_y_clamped = src_y.min(shape[1] - 1);

                // Assign the corresponding tile from the source array
                new_arr[[x, y]] = self.shape[[src_x_clamped, src_y_clamped]].clone();
            }
        }
        self.transform.scale *= scale;
        self.shape = new_arr;
        self
    }

    // Retrieves the hex array in local space
    pub fn get_hexes(&self) -> ArrayView2<Tile<T>> {
        self.shape.view()
    }
}
