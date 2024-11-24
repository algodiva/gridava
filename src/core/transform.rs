use std::ops::{Add, AddAssign, Mul};

// Transform TODO:
//      - Implement methods of combining transforms such as trans1 + trans2
//      - Implement rotation helper functions/rotation struct.

// Transformation matrix data structure.
// Stores information to manipulate a coordinate in space.
#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
pub struct Transform<T: Copy + AddAssign + Transformable<T>> {
    pub translation: T,
    pub rotation: i32, // rotation around z-axis; positive CW, negative CCW
    pub scale: i32,    // Can this be a coordinate or even a tuple of floats and not a i32?
}

#[macro_export]
macro_rules! transform {
    ($t:expr) => {
        Transform {
            translation: $t,
            rotation: 0,
            scale: 1,
        }
    };

    ($t:expr, $r:expr) => {
        Transform {
            translation: $t,
            rotation: $r,
            scale: 1,
        }
    }; 
    
    
    
    // When scale is working we can uncomment this.
       // ($t:expr, $r:expr, $s:expr) => {
       //     Transform {
       //         translation: $t,
       //         rotation: $r,
       //         scale: $s,
       //     }
       // };
}
pub use transform;

// Default trait implementation.
// Uses default traits of internals except for scale, scale is defaulted to 1.
impl<T: Copy + AddAssign + Transformable<T> + Default> Default for Transform<T> {
    fn default() -> Self {
        Self {
            translation: Default::default(),
            rotation: Default::default(),
            scale: 1,
        }
    }
}

// Transformable trait that defines how a transform manipulates a data structure.
// apply_all trait function *MUST* follow a specific order of operations to maintain accuracy -> self.Scale.Rotation.Translation
pub trait Transformable<T: Copy + AddAssign + Transformable<T>> {
    // Apply the translation to the coordinate
    fn apply_translation(&self, translation: T) -> T
    where
        Self: Add<T, Output = T> + Copy,
    {
        *self + translation
    }

    // Apply the rotation to the coordinate
    fn apply_rotation(&self, rotation: i32) -> T;

    // Apply the scale to the coordinate
    // TODO SCALE: for now we ignore scale since it can cause errors and is not implemented.
    #[allow(unused_variables)]
    fn apply_scale(&self, scale: i32) -> T
    where
        Self: Mul<i32, Output = T> + Copy,
    {
        *self * 1
    }

    // Apply all the operations to the coordinate
    #[inline]
    fn apply_all(&self, transform: Transform<T>) -> T
    where
        Self: Mul<i32, Output = T> + Copy,
        T: Add<T, Output = T>,
    {
        // The order of operations is specific. In order to get accurate results, we must scale first, then apply our local rotations, then translate the coordinates.
        self.apply_scale(transform.scale)
            .apply_rotation(transform.rotation)
            .apply_translation(transform.translation)
    }
}

impl<T: Copy + AddAssign + Transformable<T>> Transform<T> {
    #[inline]
    pub fn translate(&mut self, coord: T) -> &Self {
        self.translation += coord;
        self
    }

    #[inline]
    pub fn rotate(&mut self, rot_dir: i32) -> &Self {
        self.rotation += rot_dir;
        self
    }

    #[inline]
    pub fn scale(&mut self, scale: i32) -> &Self {
        self.scale += scale;
        self
    }
}

// Overloading '+' operator to facilitate combining multiple transforms.
impl<T> Add<Transform<T>> for Transform<T>
where
    T: Copy + AddAssign + Add<T, Output = T> + Transformable<T>,
{
    type Output = Transform<T>;

    fn add(self, rhs: Transform<T>) -> Self::Output {
        Transform {
            translation: self.translation + rhs.translation,
            rotation: self.rotation + rhs.rotation,
            scale: self.scale + rhs.scale,
        }
    }
}
