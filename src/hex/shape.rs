use std::rc::Rc;

use crate::core::transform::{Transform, Transformable};

use super::coordinate::Axial;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct HexTile<T> {
    pub coord: Axial,
    pub data: Option<Rc<T>>,
}

impl<T: Default> HexTile<T> {
    pub fn new(coord: Axial, data: Option<T>) -> HexTile<T> {
        HexTile { coord, data: data.map(|value| Rc::new(value))}
    }
}

impl<T: Default> From<Axial> for HexTile<T> {
    fn from(value: Axial) -> Self {
        HexTile::new(value, None)
    }
}

// A shape consists of coordinate vectors that define which hexes in relation to the origin are a part of the shape
#[derive(Clone, PartialEq, Debug, Default)]
pub struct HexShape<T> {
    tiles: Vec<HexTile<T>>, // Vector of hex vectors pointing to origin that comprise this shape.
    pub transform: Transform<Axial>, // The transformation matrix to convert from parent grid to local space.
}

// TODO
//      - scale shape? This will be a complicated task as to interpolate inbetween hexes

impl<T> HexShape<T>
{
    pub fn new(hex_array: Option<Vec<HexTile<T>>>, transform: Option<Transform<Axial>>) -> Self
    where
        T: Default
    {
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

    pub fn resize(&mut self, scale: (f64, f64)) -> &Self {
        todo!()
    }

    // Bake the transform into localspace
    pub fn bake_transform(&mut self) -> HexShape<T> {
        todo!()
    }

    // Retrieves the hex array in local space
    pub fn get_hexes(&self) -> &[HexTile<T>] {
        self.tiles.as_slice()
    }

    // Retrieves the hex array in grid space
    // Because we modify the HexTile coordinates into gs we must
    // return a cloned vector instead of a slice.
    pub fn get_hexes_gs(&mut self) -> Vec<HexTile<T>>
    where
        T: Clone
    {
        self.tiles
            .iter()
            .map(|tile| {tile.coord.apply_all(self.transform); tile.clone()})
            .collect()
    }
}

// General shape constructors
// Move this to another file? Inside a different module?
pub mod shape_builders {
    use crate::{axial, transform};

    use super::*;

    pub struct Inequality {
        q_min: i32,
        q_max: i32,
        r_min: i32,
        r_max: i32,
        s_min: i32,
        s_max: i32,
    }

    impl Inequality {
        pub fn new(points: &[Axial]) -> Self {
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

            Inequality {
                q_min,
                q_max,
                r_min,
                r_max,
                s_min,
                s_max,
            }
        }

        pub fn solve<T, D>(&self, data: Option<D>, constuctor: &dyn Fn(Axial, Option<D>) -> T) -> Vec<T>
        where
            D: Clone
        {
            let mut vec = vec![];
            for q in self.q_min..=self.q_max {
                for r in self.r_min.max(-q - self.s_max)..=self.r_max.min(-q - self.s_min) {
                    vec.push(constuctor(axial!(q, r), data.clone()));
                }
            }
            vec
        }
    }

    // Creates a shape.
    // This algorithm will produce regular and irregular shapes with points from points argument.
    // The algorithm *WILL* calculate it's inequalities on EVERY point in the array. So, in example, if you have a point
    // inside a shape, that point will still be calculated but will not change anything about the resultant inequality.
    // points: Coordinates to calculate the system of linear inequalities from.
    pub fn make_shape<T>(points: &[Axial], origin: Option<Axial>, data: Option<T>, constuctor: &dyn Fn(Axial, Option<T>) -> HexTile<T>) -> HexShape<T>
    where
        T: Clone + Default
    {
        if points.is_empty() {
            return HexShape::new(None, None);
        }

        let transform = transform!(origin.unwrap_or(points[0]));

        let hexes = Inequality::new(points).solve::<HexTile<T>, T>(data, constuctor);

        HexShape::new(Some(hexes), Some(transform))
    }

    pub struct ShapeArgs {
        pub origin: Axial,
        pub size: u32,
        pub rot_dir: i32,
    }

    #[macro_export]
    macro_rules! shapeargs {
        ($o:expr, $s:expr, $rd:expr) => {
            ShapeArgs {
                origin: $o,
                size: $s,
                rot_dir: $rd,
            }
        };
    }
    pub use shapeargs;

    // Creates a line.
    // The shape has hexes originating from (0,0) in local space.
    // origin: Denotes the grid space location the shape occupies. Affects the transform.
    pub fn line<T>(args: ShapeArgs, data: Option<T>, constuctor: &dyn Fn(Axial, Option<T>) -> HexTile<T>) -> HexShape<T> 
    where
        T: Clone + Default
    {
        // Working in local space
        let vertex_a = axial!(0, 0);
        let vertex_b = vertex_a.make_vector(args.size as i32, args.rot_dir);
        make_shape(&[vertex_a, vertex_b], Some(args.origin), data, constuctor)
    }

    // Creates a regular triangle.
    // The shape has hexes originating from (0,0) in local space.
    // origin: Denotes the grid space location the shape occupies. Affects the transform.
    pub fn triangle<T>(args: ShapeArgs, data: Option<T>, constuctor: &dyn Fn(Axial, Option<T>) -> HexTile<T>) -> HexShape<T> 
    where
        T: Clone + Default
    {
        // Working in local space
        let vertex_a = axial!(0, 0);
        let vertex_b = vertex_a.make_vector(args.size as i32, args.rot_dir);
        let vertex_c = vertex_a.make_vector(args.size as i32, args.rot_dir + 1);

        make_shape(&[vertex_a, vertex_b, vertex_c], Some(args.origin), data, constuctor)
    }

    // Creates a regular rhombus.
    // The shape has hexes originating from (0,0) in local space.
    // origin: Denotes the grid space location the shape occupies. Affects the transform.
    pub fn rhombus<T>(args: ShapeArgs, data: Option<T>, constuctor: &dyn Fn(Axial, Option<T>) -> HexTile<T>) -> HexShape<T>
    where
        T: Clone + Default
    {
        // Working in local space
        let vertex_a = axial!(0, 0);
        let vertex_b = vertex_a.make_vector(args.size as i32, args.rot_dir);
        let vertex_c = vertex_a.make_vector(args.size as i32, args.rot_dir + 1);
        let vertex_d = vertex_b.make_vector(args.size as i32, args.rot_dir + 1);

        make_shape(&[vertex_a, vertex_b, vertex_c, vertex_d], Some(args.origin), data, constuctor)
    }
}
