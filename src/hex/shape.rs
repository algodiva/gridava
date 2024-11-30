//! Shapes in a hex grid.

use ndarray::{array, Array, Array2};

use crate::{
    axial,
    core::transform::{Transform, Vector2D},
    transform, vector2d,
};

use super::coordinate::Axial;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A shape is a collection of coordinates.
///
/// Each coordinate is a vector that 'points' to the origin coordinate creating a shape local space.
/// The transformation matrix associated to the shape is then used to convert this local space to the
/// coordinate space of the parent.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, PartialEq, Debug, Default)]
pub struct HexShape<T: Clone> {
    shape: Array2<Option<T>>, // 2D array of tiles, index corresponds to coordinate (q, r). Origin of (0,0) to (+∞,+∞).
    pub transform: Transform<Axial>, // The transformation matrix to convert from parent grid to local space.
}

/// Struct to handle arguments to shape constructors.
///
/// `size` field denotes length.
///
/// `rot_dir`: positive denotes CW, negative CCW, magnitude denotes how many 60 degree rotations.
pub struct ShapeArgs {
    pub size: u32,
    pub rot_dir: i32,
    pub square_bb: bool,
}

/// Helper macro to create [`ShapeArgs`]
#[macro_export]
macro_rules! shapeargs {
    ($s:expr, $rd:expr, $sbb:expr) => {
        ShapeArgs {
            size: $s,
            rot_dir: $rd,
            square_bb: $sbb,
        }
    };
}
pub use shapeargs;

impl<T: Clone> HexShape<T> {
    /// Create a new shape.
    ///
    /// Can be provided with optional parameters to specify coordinates it occupies
    /// and its transform. These will default to empty and (0, 0) respectively.
    ///
    /// ```
    /// use gridava::hex::shape::HexShape;
    ///
    /// let my_shape: HexShape<i32> = HexShape::new(None, None);
    /// ```
    pub fn new(shape: Option<Array2<Option<T>>>, transform: Option<Transform<Axial>>) -> Self {
        Self {
            shape: match shape {
                Some(arr) => arr,
                None => array![[], []],
            },
            transform: transform.unwrap_or_default(),
        }
    }

    /// Creates a shape via inequalities.
    ///
    /// Will create an irregular or regular shape based on a system of linear inequalities
    /// derived from the provided points slice.
    ///
    /// ```
    /// use gridava::core::tile::Tile;
    /// use gridava::hex::coordinate::{Axial, axial};
    /// use gridava::hex::shape::HexShape;
    ///
    /// /// shape_verts stores a triangle of size 1
    /// let shape_verts = vec![axial!(0, 0), axial!(0, 1), axial!(1, 0)];
    /// let my_shape = HexShape::make_shape(&shape_verts, true, || Tile::new(Some(1)));
    /// ```
    ///
    /// The algorithm *WILL* calculate its inequalities on EVERY point in the array. So, in example, if you have a point
    /// inside a shape, that point will still be calculated but will not change anything about the resultant inequality.
    pub fn make_shape<F>(points: &[Axial], square_bb: bool, mut constructor: F) -> Self
    where
        F: FnMut() -> T,
    {
        // We cannot construct a shape with no points, return an empty shape.
        if points.is_empty() {
            return HexShape::new(None, None);
        }

        // Assign our transform.
        let transform = transform!(axial!(0, 0));

        // Compute a system of linear inequalities.
        let mut q_min = i32::MAX;
        let mut q_max = i32::MIN;
        let mut r_min = i32::MAX;
        let mut r_max = i32::MIN;
        let mut s_min = i32::MAX;
        let mut s_max = i32::MIN;
        for point in points {
            q_min = q_min.min(point.q);
            q_max = q_max.max(point.q);
            r_min = r_min.min(point.r);
            r_max = r_max.max(point.r);
            let s = point.compute_s();
            s_min = s_min.min(s);
            s_max = s_max.max(s);
        }

        // Solve for all the hexes inside an inequality.
        let hexes: Vec<Axial> = (q_min..=q_max)
            .flat_map(|q| {
                (r_min.max(-q - s_max)..=r_max.min(-q - s_min))
                    .map(move |r| axial!(q - q_min, r - r_min))
            })
            .collect();

        // Find the bounding size of this inequality as a square or tightly
        // bound according to the value of square_bb.
        let size = match square_bb {
            true => {
                let largest = (q_max - q_min)
                    .unsigned_abs()
                    .max((r_max - r_min).unsigned_abs());
                ((largest + 1) as usize, (largest + 1) as usize)
            }
            false => (
                ((q_max - q_min).unsigned_abs() + 1) as usize,
                ((r_max - r_min).unsigned_abs() + 1) as usize,
            ),
        };

        // Create our array.
        let mut arr = Array::from_shape_simple_fn(size, || None);

        // Construct tiles that the shape contains.
        for coord in hexes {
            arr[[coord.q as usize, coord.r as usize]] = Some(constructor());
        }

        HexShape::new(Some(arr), Some(transform))
    }

    /// Create a line shape.
    ///
    /// Given a size and direction this will create a line.
    ///
    /// see [`Self::make_shape`] for more.
    ///
    /// ```
    /// use gridava::core::tile::Tile;
    /// use gridava::hex::shape::HexShape;
    ///
    /// /// Creates a line of size 1, 0-1 inclusive, and sets the tiles to Some(1)
    /// let my_shape = HexShape::make_line(1, 0, true, || Tile::new(Some(1)));
    /// ```
    pub fn make_line<F>(size: u32, rot_dir: i32, square_bb: bool, constructor: F) -> Self
    where
        F: FnMut() -> T,
    {
        // Working in local space
        let vertex_a = axial!(0, 0);
        let vertex_b = vertex_a.make_vector(size as i32, rot_dir);
        Self::make_shape(&[vertex_a, vertex_b], square_bb, constructor)
    }

    /// Create a triangle shape.
    ///
    /// Given a size and direction this will create a triangle.
    ///
    /// see [`Self::make_shape`] for more.
    ///
    /// ```
    /// use gridava::core::tile::Tile;
    /// use gridava::hex::shape::HexShape;
    ///
    /// /// Creates a triangle of size 1, 0-1 inclusive, and sets the tiles to Some(1)
    /// let my_shape = HexShape::make_triangle(1, 0, true, || Tile::new(Some(1)));
    /// ```
    pub fn make_triangle<F>(size: u32, rot_dir: i32, square_bb: bool, constructor: F) -> Self
    where
        F: FnMut() -> T,
    {
        // Working in local space
        let vertex_a = axial!(0, 0);
        let vertex_b = vertex_a.make_vector(size as i32, rot_dir);
        let vertex_c = vertex_a.make_vector(size as i32, rot_dir + 1);

        Self::make_shape(&[vertex_a, vertex_b, vertex_c], square_bb, constructor)
    }

    /// Create a rhombus shape.
    ///
    /// Given a size and direction this will create a rhombus.
    ///
    /// see [`Self::make_shape`] for more.
    ///
    /// ```
    /// use gridava::core::tile::Tile;
    /// use gridava::hex::shape::HexShape;
    ///
    /// /// Creates a rhombus of size 1, 0-1 inclusive, and sets the tiles to Some(1)
    /// let my_shape = HexShape::make_rhombus(1, 0, true, || Tile::new(Some(1)));
    /// ```
    pub fn make_rhombus<F>(size: u32, rot_dir: i32, square_bb: bool, constructor: F) -> Self
    where
        F: FnMut() -> T,
    {
        // Working in local space
        let vertex_a = axial!(0, 0);
        let vertex_b = vertex_a.make_vector(size as i32, rot_dir);
        let vertex_c = vertex_a.make_vector(size as i32, rot_dir + 1);
        let vertex_d = vertex_b.make_vector(size as i32, rot_dir + 1);

        Self::make_shape(
            &[vertex_a, vertex_b, vertex_c, vertex_d],
            square_bb,
            constructor,
        )
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
    pub fn scale(mut self, scale: Vector2D<f32>) -> Self {
        // Uses bilinear interpolation algorithm, it's lossless  meaning if you apply a scale and then its inverse
        //  it will return to its original shape.

        let shape = self.shape.shape();

        let new_x = (shape[0] as f32 * scale.x).round() as usize;
        let new_y = (shape[1] as f32 * scale.y).round() as usize;

        let mut new_arr = Array2::from_shape_simple_fn((new_x, new_y), || None);

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
        self.shape = new_arr;
        self
    }

    /// Set a new origin for the shape.
    pub fn set_origin(&mut self, new_origin: Transform<Axial>) -> &Self {
        self.transform = new_origin;
        self
    }

    /// Overwrite the internal working array of the shape.
    pub fn set_hexes(mut self, in_arr: Array2<Option<T>>) -> Self {
        self.shape = in_arr;
        self
    }

    /// Get a reference to the shape's tile array.
    ///
    /// # Example
    /// ```
    /// use gridava::hex::coordinate::Axial;
    /// use gridava::hex::shape::HexShape;
    /// use gridava::core::tile::Tile;
    /// use ndarray::array;
    ///
    /// let arr = array![[Some(Tile::<i32>::default()), None],
    ///                 [None, Some(Tile::<i32>::default())]];
    ///
    /// let my_shape = HexShape::new(Some(arr), None);
    /// let hexes_ls = my_shape.get_hexes();
    /// ```
    pub fn get_hexes(&self) -> &Array2<Option<T>> {
        &self.shape
    }

    /// Get a mutable version of the shape's array.
    ///
    /// # Example
    /// ```
    /// use gridava::hex::coordinate::Axial;
    /// use gridava::hex::shape::HexShape;
    /// use gridava::core::tile::Tile;
    /// use ndarray::array;
    ///
    /// let arr = array![[Some(Tile::<i32>::default()), None],
    ///                 [None, Some(Tile::<i32>::default())]];
    ///
    /// let mut my_shape = HexShape::new(Some(arr), None);
    /// let hexes_ls = my_shape.get_hexes_mut();
    /// ```
    pub fn get_hexes_mut(&mut self) -> &mut Array2<Option<T>> {
        &mut self.shape
    }
}

#[allow(unused_imports)]
mod tests {
    use super::*;
    use crate::{axial, core::tile::Tile};

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

    #[test]
    fn make_shape() {
        assert_eq!(
            HexShape::make_shape(&[], true, || 1),
            HexShape::new(None, None)
        );

        assert_eq!(
            HexShape::make_shape(&[axial!(0, 0), axial!(2, 0)], false, || 1).get_hexes(),
            Array::from_shape_simple_fn((3, 1), || Some(1))
        )
    }

    #[test]
    fn make_line() {
        let default_tile_fn = &Tile::<i32>::default;

        assert_eq!(
            HexShape::make_line(0, 0, true, default_tile_fn),
            HexShape::make_shape(&[axial!(0, 0)], true, default_tile_fn)
        );
        assert_eq!(
            HexShape::make_line(4, 0, true, default_tile_fn),
            HexShape::make_shape(&[axial!(0, 0), axial!(4, 0)], true, default_tile_fn)
        );
        assert_eq!(
            HexShape::make_line(2, 1, true, default_tile_fn),
            HexShape::make_shape(
                &[axial!(0, 0), axial!(0, 1), axial!(0, 2)],
                true,
                default_tile_fn
            )
        );
        assert_eq!(
            HexShape::make_line(2, 2, true, default_tile_fn),
            HexShape::make_shape(
                &[axial!(2, 0), axial!(1, 1), axial!(0, 2)],
                true,
                default_tile_fn
            )
        );
    }

    #[test]
    fn make_triangle() {
        assert_eq!(
            HexShape::make_triangle(1, 0, true, || 1),
            HexShape::make_shape(&[axial!(0, 0), axial!(1, 0), axial!(0, 1)], true, || 1)
        );
    }

    #[test]
    fn make_rhombus() {
        assert_eq!(
            HexShape::make_rhombus(1, 0, true, || 1),
            HexShape::make_shape(
                &[axial!(0, 0), axial!(1, 0), axial!(0, 1), axial!(1, 1)],
                true,
                || 1
            )
        );
    }

    #[test]
    fn scale() {
        assert_eq!(
            HexShape::make_rhombus(1, 0, true, || 1).scale(vector2d!(2.0, 2.0)),
            HexShape::make_rhombus(3, 0, true, || 1)
        );

        assert_eq!(
            HexShape::make_rhombus(1, 0, true, || 1)
                .scale(vector2d!(2.0, 2.0))
                .scale(vector2d!(0.5, 0.5)),
            HexShape::make_rhombus(1, 0, true, || 1)
        );
    }

    #[test]
    fn set_origin() {
        let mut shape = HexShape::<i32>::new(None, None);
        assert_eq!(
            shape.set_origin(transform!(axial!(1, 1))).transform,
            transform!(axial!(1, 1))
        );
    }

    #[test]
    fn set_hexes() {
        let shape = HexShape::make_rhombus(1, 0, true, || 1);
        let new_arr = Array::from_shape_simple_fn((1, 1), || Some(1));
        assert_eq!(shape.set_hexes(new_arr.clone()).get_hexes(), new_arr);
    }

    #[test]
    fn get_hexes() {
        let shape = HexShape::make_rhombus(1, 0, true, || 1);
        assert_eq!(
            shape.get_hexes(),
            Array::from_shape_simple_fn((2, 2), || Some(1))
        )
    }

    #[test]
    fn get_hexes_mut() {
        let mut shape = HexShape::make_rhombus(1, 0, true, || 1);
        assert_eq!(
            shape.get_hexes_mut(),
            &mut Array::from_shape_simple_fn((2, 2), || Some(1))
        )
    }

    // TODO: scale, get_hexes
}
