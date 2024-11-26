//! Shapes in a hex grid.

use ndarray::{Array, Array2, ArrayView2};

use crate::{
    axial,
    core::{
        tile::Tile,
        transform::{Transform, Vector2D},
    },
    transform, vector2d,
};

use super::axial::Axial;

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

    /// Creates a shape via inequalities.
    ///
    /// Will create an irregular or regular shape based on a system of linear inequalities
    /// derived from the provided points slice.
    ///
    /// ```
    /// use gridava::core::tile::Tile;
    /// use gridava::hex::axial::*;
    /// use gridava::hex::shape::*;
    ///
    /// /// shape_verts stores a triangle of size 1
    /// let shape_verts = vec![axial!(0, 0), axial!(0, 1), axial!(1, 0)];
    /// let my_shape = HexShape::make_shape(&shape_verts, true, || Tile::new(Some(1)));
    /// ```
    ///
    /// The algorithm *WILL* calculate its inequalities on EVERY point in the array. So, in example, if you have a point
    /// inside a shape, that point will still be calculated but will not change anything about the resultant inequality.
    pub fn make_shape<F>(points: &[Axial], square_bb: bool, mut constructor: F) -> HexShape<T>
    where
        T: Clone + Default,
        F: FnMut() -> Tile<T>,
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
        let mut arr = Array::from_shape_simple_fn(size, &Tile::default);

        // Construct tiles that the shape contains.
        for coord in hexes {
            arr[[coord.q as usize, coord.r as usize]] = constructor();
        }

        HexShape::new(Some(arr), Some(transform))
    }

    /// Create a line shape.
    ///
    /// Given a size and direction, see [`ShapeArgs`], this will create a line.
    ///
    /// see [`Self::make_shape`] for more.
    ///
    /// ```
    /// use gridava::core::tile::Tile;
    /// use gridava::hex::shape::*;
    ///
    /// /// Creates a line of size 1, 0-1 inclusive, and sets the tiles to Some(1)
    /// let my_shape = HexShape::line(shapeargs!(1, 0, true), || Tile::new(Some(1)));
    /// ```
    pub fn line<F>(args: ShapeArgs, constructor: F) -> HexShape<T>
    where
        T: Clone + Default,
        F: FnMut() -> Tile<T>,
    {
        // Working in local space
        let vertex_a = axial!(0, 0);
        let vertex_b = vertex_a.make_vector(args.size as i32, args.rot_dir);
        Self::make_shape(&[vertex_a, vertex_b], args.square_bb, constructor)
    }

    /// Create a triangle shape.
    ///
    /// Given a size and direction, see [`ShapeArgs`], this will create a triangle.
    ///
    /// see [`Self::make_shape`] for more.
    ///
    /// ```
    /// use gridava::core::tile::Tile;
    /// use gridava::hex::shape::*;
    ///
    /// /// Creates a triangle of size 1, 0-1 inclusive, and sets the tiles to Some(1)
    /// let my_shape = HexShape::triangle(shapeargs!(1, 0, true), || Tile::new(Some(1)));
    /// ```
    pub fn triangle<F>(args: ShapeArgs, constructor: F) -> HexShape<T>
    where
        T: Clone + Default,
        F: FnMut() -> Tile<T>,
    {
        // Working in local space
        let vertex_a = axial!(0, 0);
        let vertex_b = vertex_a.make_vector(args.size as i32, args.rot_dir);
        let vertex_c = vertex_a.make_vector(args.size as i32, args.rot_dir + 1);

        Self::make_shape(&[vertex_a, vertex_b, vertex_c], args.square_bb, constructor)
    }

    /// Create a rhombus shape.
    ///
    /// Given a size and direction, see [`ShapeArgs`], this will create a rhombus.
    ///
    /// see [`Self::make_shape`] for more.
    ///
    /// ```
    /// use gridava::core::tile::Tile;
    /// use gridava::hex::shape::*;
    ///
    /// /// Creates a rhombus of size 1, 0-1 inclusive, and sets the tiles to Some(1)
    /// let my_shape = HexShape::rhombus(shapeargs!(1, 0, true), || Tile::new(Some(1)));
    /// ```
    pub fn rhombus<F>(args: ShapeArgs, constructor: F) -> HexShape<T>
    where
        T: Clone + Default,
        F: FnMut() -> Tile<T>,
    {
        // Working in local space
        let vertex_a = axial!(0, 0);
        let vertex_b = vertex_a.make_vector(args.size as i32, args.rot_dir);
        let vertex_c = vertex_a.make_vector(args.size as i32, args.rot_dir + 1);
        let vertex_d = vertex_b.make_vector(args.size as i32, args.rot_dir + 1);

        Self::make_shape(
            &[vertex_a, vertex_b, vertex_c, vertex_d],
            args.square_bb,
            constructor,
        )
    }

    /// Translate the shape.
    ///
    /// Mutates the transform of the shape.
    ///
    /// ```
    /// use gridava::hex::axial::*;
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
    /// use gridava::hex::axial::{Axial, axial};
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
    /// use gridava::hex::axial::*;
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
    /// use gridava::hex::axial::{Axial, axial};
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

    #[test]
    fn test_line() {
        let default_tile_fn = &Tile::<i32>::default;

        assert_eq!(
            HexShape::line(
                ShapeArgs {
                    size: 0,
                    rot_dir: 0,
                    square_bb: true
                },
                default_tile_fn
            ),
            HexShape::make_shape(&[axial!(0, 0)], true, default_tile_fn)
        );
        assert_eq!(
            HexShape::line(
                ShapeArgs {
                    size: 4,
                    rot_dir: 0,
                    square_bb: true
                },
                default_tile_fn
            ),
            HexShape::make_shape(&[axial!(0, 0), axial!(0, 4)], true, default_tile_fn)
        );
        assert_eq!(
            HexShape::line(
                ShapeArgs {
                    size: 2,
                    rot_dir: 1,
                    square_bb: true
                },
                default_tile_fn
            ),
            HexShape::make_shape(
                &[axial!(0, 2), axial!(1, 2), axial!(2, 2)],
                true,
                default_tile_fn
            )
        );
        assert_eq!(
            HexShape::line(
                ShapeArgs {
                    size: 2,
                    rot_dir: 2,
                    square_bb: true
                },
                default_tile_fn
            ),
            HexShape::make_shape(
                &[axial!(0, 4), axial!(1, 4), axial!(2, 4)],
                true,
                default_tile_fn
            )
        );
    }

    // TODO: scale, get_hexes
}
