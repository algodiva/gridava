//! collection of constructors for a shape.

use std::error::Error;

use super::{coordinate::Axial, shape::HexShape};

use ndarray::Array;

use crate::{
    axial,
    core::tile::Tile,
    core::transform::{Transform, Vector2D},
    transform, vector2d,
};

#[derive(Debug)]
pub enum InequalityError {
    EmptyInputArray,
}

impl std::fmt::Display for InequalityError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            InequalityError::EmptyInputArray => {
                write!(f, "Cannot create an Inequality with an empty array")
            }
        }
    }
}

impl Error for InequalityError {}

// Struct to hold data for a system of linear inequalities for Axial coordinates.
pub struct Inequality {
    pub q_min: i32,
    pub q_max: i32,
    pub r_min: i32,
    pub r_max: i32,
    pub s_min: i32,
    pub s_max: i32,
}

impl Inequality {
    pub fn new(points: &[Axial]) -> Result<Self, InequalityError> {
        if points.is_empty() {
            return Err(InequalityError::EmptyInputArray);
        }
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

        Ok(Inequality {
            q_min,
            q_max,
            r_min,
            r_max,
            s_min,
            s_max,
        })
    }

    // Solve the system of linear inequalities to produce an array of coordinates that fall within
    // the resultant area described by the inequality.
    pub fn solve(&self) -> Vec<Axial> {
        (self.q_min..=self.q_max)
            .flat_map(|q| {
                (self.r_min.max(-q - self.s_max)..=self.r_max.min(-q - self.s_min))
                    .map(move |r| axial!(q - self.q_min, r - self.r_min))
            })
            .collect()
    }

    pub fn q_stride(&self) -> usize {
        (self.q_max - self.q_min).unsigned_abs() as usize
    }

    pub fn r_stride(&self) -> usize {
        (self.r_max - self.r_min).unsigned_abs() as usize
    }

    pub fn largest_stride(&self) -> usize {
        self.q_stride().max(self.r_stride())
    }
}

/// Creates a shape via inequalities.
///
/// Will create an irregular or regular shape based on a system of linear inequalities
/// derived from the provided points slice.
///
/// ```
/// use gridava::core::tile::Tile;
/// use gridava::hex::coordinate::*;
/// use gridava::hex::shape_constructors;
///
/// /// shape_verts stores a triangle of size 1
/// let shape_verts = vec![axial!(0, 0), axial!(0, 1), axial!(1, 0)];
/// let my_shape = shape_constructors::make_shape(&shape_verts, || Tile::new(Some(1)));
/// ```
///
/// The algorithm *WILL* calculate it's inequalities on EVERY point in the array. So, in example, if you have a point
/// inside a shape, that point will still be calculated but will not change anything about the resultant inequality.
pub fn make_shape<T, F>(points: &[Axial], mut constructor: F) -> HexShape<T>
where
    T: Clone + Default,
    F: FnMut() -> Tile<T>,
{
    if points.is_empty() {
        return HexShape::new(None, None);
    }

    let transform = transform!(axial!(0, 0));

    // This unwrap is safe due to our if guard above.
    let ineq = Inequality::new(points).unwrap();

    let hexes = ineq.solve();

    let mut arr = Array::from_shape_simple_fn(
        (ineq.largest_stride() + 1, ineq.largest_stride() + 1),
        &Tile::default,
    );

    for coord in hexes {
        arr[[coord.q as usize, coord.r as usize]] = constructor();
    }

    HexShape::new(Some(arr), Some(transform))
}

/// Struct to handle arguments to shape constructors.
///
/// `size` field denotes length.
///
/// `rot_dir`: positive denotes CW, negative CCW, magnitude denotes how many 60 degree rotations.
pub struct ShapeArgs {
    pub size: u32,
    pub rot_dir: i32,
}

/// Helper macro to create [`ShapeArgs`]
#[macro_export]
macro_rules! shapeargs {
    ($s:expr, $rd:expr) => {
        ShapeArgs {
            size: $s,
            rot_dir: $rd,
        }
    };
}
pub use shapeargs;

/// Create a line shape.
///
/// Given a size and direction, see [`ShapeArgs`], this will create a line.
///
/// see [`make_shape`] for more.
///
/// ```
/// use gridava::core::tile::Tile;
/// use gridava::hex::{shape_constructors, shape_constructors::*};
///
/// /// Creates a line of size 1, 0-1 inclusive, and sets the tiles to Some(1)
/// let my_shape = shape_constructors::line(shapeargs!(1, 0), || Tile::new(Some(1)));
/// ```
pub fn line<T, F>(args: ShapeArgs, constructor: F) -> HexShape<T>
where
    T: Clone + Default,
    F: FnMut() -> Tile<T>,
{
    // Working in local space
    let vertex_a = axial!(0, 0);
    let vertex_b = vertex_a.make_vector(args.size as i32, args.rot_dir);
    make_shape(&[vertex_a, vertex_b], constructor)
}

/// Create a triangle shape.
///
/// Given a size and direction, see [`ShapeArgs`], this will create a triangle.
///
/// see [`make_shape`] for more.
///
/// ```
/// use gridava::core::tile::Tile;
/// use gridava::hex::{shape_constructors, shape_constructors::*};
///
/// /// Creates a triangle of size 1, 0-1 inclusive, and sets the tiles to Some(1)
/// let my_shape = shape_constructors::triangle(shapeargs!(1, 0), || Tile::new(Some(1)));
/// ```
pub fn triangle<T, F>(args: ShapeArgs, constructor: F) -> HexShape<T>
where
    T: Clone + Default,
    F: FnMut() -> Tile<T>,
{
    // Working in local space
    let vertex_a = axial!(0, 0);
    let vertex_b = vertex_a.make_vector(args.size as i32, args.rot_dir);
    let vertex_c = vertex_a.make_vector(args.size as i32, args.rot_dir + 1);

    make_shape(&[vertex_a, vertex_b, vertex_c], constructor)
}

/// Create a rhombus shape.
///
/// Given a size and direction, see [`ShapeArgs`], this will create a rhombus.
///
/// see [`make_shape`] for more.
///
/// ```
/// use gridava::core::tile::Tile;
/// use gridava::hex::{shape_constructors, shape_constructors::*};
///
/// /// Creates a rhombus of size 1, 0-1 inclusive, and sets the tiles to Some(1)
/// let my_shape = shape_constructors::rhombus(shapeargs!(1, 0), || Tile::new(Some(1)));
/// ```
pub fn rhombus<T, F>(args: ShapeArgs, constructor: F) -> HexShape<T>
where
    T: Clone + Default,
    F: FnMut() -> Tile<T>,
{
    // Working in local space
    let vertex_a = axial!(0, 0);
    let vertex_b = vertex_a.make_vector(args.size as i32, args.rot_dir);
    let vertex_c = vertex_a.make_vector(args.size as i32, args.rot_dir + 1);
    let vertex_d = vertex_b.make_vector(args.size as i32, args.rot_dir + 1);

    make_shape(&[vertex_a, vertex_b, vertex_c, vertex_d], constructor)
}

#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_line() {
        let default_tile_fn = &Tile::<i32>::default;

        assert_eq!(
            line(
                ShapeArgs {
                    size: 0,
                    rot_dir: 0
                },
                default_tile_fn
            ),
            make_shape(&[axial!(0, 0)], default_tile_fn)
        );
        assert_eq!(
            line(
                ShapeArgs {
                    size: 4,
                    rot_dir: 0
                },
                default_tile_fn
            ),
            make_shape(&[axial!(0, 0), axial!(0, 4)], default_tile_fn)
        );
        assert_eq!(
            line(
                ShapeArgs {
                    size: 2,
                    rot_dir: 1
                },
                default_tile_fn
            ),
            make_shape(&[axial!(0, 2), axial!(1, 2), axial!(2, 2)], default_tile_fn)
        );
        assert_eq!(
            line(
                ShapeArgs {
                    size: 2,
                    rot_dir: 2
                },
                default_tile_fn
            ),
            make_shape(&[axial!(0, 4), axial!(1, 4), axial!(2, 4)], default_tile_fn)
        );
    }
}
