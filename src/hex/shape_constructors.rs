use std::error::Error;

use super::{coordinate::Axial, shape::HexShape};

use ndarray::Array;

use crate::{
    axial,
    core::tile::Tile,
    core::transform::{Float2D, Transform},
    float2d, transform,
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

// Creates a shape.
// This algorithm will produce regular and irregular shapes with points from points argument.
// The algorithm *WILL* calculate it's inequalities on EVERY point in the array. So, in example, if you have a point
// inside a shape, that point will still be calculated but will not change anything about the resultant inequality.
// points: Coordinates to calculate the system of linear inequalities from.
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

pub struct ShapeArgs {
    pub size: u32,
    pub rot_dir: i32,
}

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

// Creates a line.
// The shape has hexes originating from (0,0) in local space.
// origin: Denotes the grid space location the shape occupies. Affects the transform.
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

// Creates a regular triangle.
// The shape has hexes originating from (0,0) in local space.
// origin: Denotes the grid space location the shape occupies. Affects the transform.
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

// Creates a regular rhombus.
// The shape has hexes originating from (0,0) in local space.
// origin: Denotes the grid space location the shape occupies. Affects the transform.
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
