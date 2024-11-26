//! Shapes in a hex grid.

use ndarray::{Array2, ArrayView2};

use crate::core::{
    tile::Tile,
    transform::{Transform, Vector2D},
};

use super::coordinate::Axial;

/// A shape is a collection of coordinates.
///
/// Each coordinate is a vector that 'points' to the origin coordinate creating a shape local space.
/// The transformation matrix associated to the shape is then used to convert this local space to the
/// coordinate space of the parent.
#[derive(Clone, PartialEq, Debug, Default)]
pub struct HexShape<T: Clone> {
    shape: Array2<Tile<T>>, // 2D array of tiles, index corresponds to coordinate (q, r). Origin of (0,0) to (+∞,+∞).
    pub transform: Transform<Axial>, // The transformation matrix to convert from parent grid to local space.
}

impl<T: Clone> HexShape<T> {
    /// Create a new shape.
    ///
    /// Can be provided with optional parameters to specify coordinates it occupies
    /// and its transform. These will default to none and (0, 0) respectively.
    ///
    /// ```
    /// use gridava::hex::shape::HexShape;
    ///
    /// let my_shape: HexShape<i32> = HexShape::new(None, None);
    /// ```
    pub fn new(hex_array: Option<Array2<Tile<T>>>, transform: Option<Transform<Axial>>) -> Self
    where
        T: Default,
    {
        Self {
            shape: hex_array.unwrap_or_default(),
            transform: transform.unwrap_or_default(),
        }
    }

    /// Translate the shape.
    ///
    /// Mutates the transform of the shape.
    ///
    /// ```
    /// use gridava::hex::coordinate::{Axial, axial};
    /// use gridava::hex::shape::HexShape;
    ///
    /// let mut my_shape: HexShape<i32> = HexShape::new(None, None);
    /// /// Move the shape in the positive q and r axes by 1.
    /// my_shape.translate(axial!(1, 1));
    /// ```
    pub fn translate(&mut self, coord: Axial) -> &Self {
        self.transform.translation += coord;
        self
    }

    /// Rotate the shape.
    ///
    /// Mutates the transform of the shape.
    ///
    /// `rot_dir`: positive denotes CW, negative CCW, magnitude denotes how many 60 degree rotations.
    ///
    /// ```
    /// use gridava::hex::coordinate::{Axial, axial};
    /// use gridava::hex::shape::HexShape;
    ///
    /// /// The shape has an origin of (0, 0)
    /// let mut my_shape: HexShape<i32> = HexShape::new(None, None);
    /// /// Rotate the shape around the coordinate (1, 1) clockwise.
    /// my_shape.rotate_about(axial!(1, 1), 1);
    /// ```
    pub fn rotate_about(&mut self, coord: Axial, rot_dir: i32) -> &Self {
        self.transform.translation = self.transform.translation.rotate(Some(coord), rot_dir);
        self.transform.rotation += rot_dir;
        self
    }

    /// Rotates the shape, either about its local origin or some point.
    ///
    /// Mutates the transform of the shape.
    ///
    /// `coord`: Some denotes about a specific point, None denotes around the shape's center.
    ///
    /// `rot_dir`: positive denotes CW, negative CCW, magnitude denotes how many 60 degree rotations.
    /// ```
    /// use gridava::hex::shape::HexShape;
    /// use gridava::hex::coordinate::{Axial, axial};
    ///
    /// /// Rotate the shape about the coordinate (1, 1) clockwise.
    /// HexShape::<i32>::new(None, None).rotate(Some(axial!(1, 1)), 1);
    /// ```
    pub fn rotate(&mut self, coord: Option<Axial>, rot_dir: i32) -> &Self {
        match coord {
            Some(coord) => self.rotate_about(coord, rot_dir),
            None => {
                self.transform.rotation += rot_dir;
                self
            }
        }
    }

    /// Scale a shape.
    ///
    /// Mutates the internal array itself.
    ///
    /// ```
    /// use gridava::hex::shape::HexShape;
    /// use gridava::core::transform::{Vector2D, vector2d};
    ///
    /// let mut my_shape: HexShape<i32> = HexShape::new(None, None);
    /// my_shape.scale(vector2d!(2.0, 2.0));
    /// ```
    pub fn scale(&mut self, scale: Vector2D<f32>) -> &Self {
        // Uses bilinear interpolation algorithm, it's lossless  meaning if you apply a scale and then its inverse
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

    /// Returns a vector of [`Axial`] denoting coordinates the shape contains.
    ///
    /// ```
    /// use gridava::hex::coordinate::{Axial, axial};
    /// use gridava::hex::shape::HexShape;
    /// use gridava::core::tile::Tile;
    /// use ndarray::array;
    ///
    /// let my_shape: HexShape<i32> = HexShape::new(Some(array![[Tile::<i32>::default()],[Tile::<i32>::default()]]), None);
    /// let hexes_ls = my_shape.get_hexes();
    /// ```
    pub fn get_hexes(&self) -> ArrayView2<Tile<T>> {
        self.shape.view()
    }
}

#[allow(unused_imports)]
mod tests {
    use super::*;
    use crate::axial;

    #[test]
    fn translate() {
        let mut shape = HexShape::<i32>::new(None, None);

        shape.translate(axial!(4, 6));
        assert_eq!(shape.transform.translation, axial!(4, 6));

        shape.translate(axial!(2, -3));
        assert_eq!(shape.transform.translation, axial!(6, 3));
    }

    #[test]
    fn rotate_about() {
        let mut shape = HexShape::<i32>::new(None, None);

        shape.rotate_about(axial!(1, 2), 0);
        assert_eq!(shape.transform.translation, axial!(0, 0));
        assert_eq!(shape.transform.rotation, 0);

        shape.rotate_about(axial!(1, 2), 1);
        assert_eq!(shape.transform.translation, axial!(3, -1));
        assert_eq!(shape.transform.rotation, 1);

        shape.rotate_about(axial!(1, 2), -1);
        assert_eq!(shape.transform.translation, axial!(0, 0));
        assert_eq!(shape.transform.rotation, 0);

        shape.rotate_about(axial!(1, 2), 2);
        assert_eq!(shape.transform.translation, axial!(4, 1));
        assert_eq!(shape.transform.rotation, 2);

        shape.rotate_about(axial!(1, 2), -2);
        assert_eq!(shape.transform.translation, axial!(0, 0));
        assert_eq!(shape.transform.rotation, 0);

        shape.rotate_about(axial!(1, 2), 3);
        assert_eq!(shape.transform.translation, axial!(2, 4));
        assert_eq!(shape.transform.rotation, 3);
    }

    #[test]
    fn rotate() {
        let mut shape = HexShape::<i32>::new(None, None);

        shape.rotate(None, 2);
        assert_eq!(shape.transform.translation, axial!(0, 0));
        assert_eq!(shape.transform.rotation, 2);

        shape.rotate(Some(axial!(1, 2)), 2);
        assert_eq!(shape.transform.translation, axial!(4, 1));
        assert_eq!(shape.transform.rotation, 4);
    }

    // TODO: scale, get_hexes
}
