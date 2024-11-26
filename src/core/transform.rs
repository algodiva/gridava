use std::ops::{Add, AddAssign, Mul, MulAssign, Neg};

/// A 2-dimensional vector.
#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
pub struct Vector2D<T> {
    /// x axis
    pub x: T,
    /// y axis
    pub y: T,
}

/// Helper macro to quickly instantiate a [`Vector2D`].
#[macro_export]
macro_rules! vector2d {
    ($x:expr) => {
        Vector2D { x: $x, y: $x }
    };

    ($x:expr, $y:expr) => {
        Vector2D { x: $x, y: $y }
    };
}
pub use vector2d;

impl<T: AddAssign> AddAssign for Vector2D<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: MulAssign> MulAssign for Vector2D<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<T: Add<Output = T> + Copy> Add for Vector2D<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        vector2d!(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Neg<Output = T>> Neg for Vector2D<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        vector2d!(-self.x, -self.y)
    }
}

/// Transformation matrix data structure.
///
/// Stores translation, rotation and scale data to be able to perform operations with.
#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
pub struct Transform<T: Copy + AddAssign> {
    /// Movement away from the origin.
    pub translation: T,
    /// Rotation of the object around the z-axis. Positive CW, negative CCW
    pub rotation: i32,
    /// 2D scale of an object.
    pub scale: Vector2D<f32>, // Can this be a coordinate or even a tuple of floats and not a i32?
}

/// Helper macro to create [`Transform`].
///
/// Accepts 1, 2, or 3 arguments
///
/// - 1 => translation.
/// - 2 => translation, rotation.
/// - 3 => translation, rotation, scale.
#[macro_export]
macro_rules! transform {
    ($t:expr) => {
        Transform {
            translation: $t,
            rotation: 0,
            scale: vector2d!(1.0),
        }
    };

    ($t:expr, $r:expr) => {
        Transform {
            translation: $t,
            rotation: $r,
            scale: vector2d!(1.0, 1.0),
        }
    };

    ($t:expr, $r:expr, $s:expr) => {
        Transform {
            translation: $t,
            rotation: $r,
            scale: $s,
        }
    };
}
pub use transform;

// Default trait implementation.
// Uses default traits of internals except for scale, scale is defaulted to 1.
/// Default trait implementation
///
/// See [`Default`] for base implementation.
///
/// We manually set scale to a default of 1 because at base scale * size should not do anything
/// which is the multiplicative identity 1.
impl<T: Copy + AddAssign + Default> Default for Transform<T> {
    fn default() -> Self {
        Self {
            translation: Default::default(),
            rotation: Default::default(),
            scale: vector2d!(1.0),
        }
    }
}

impl<T> Add<Transform<T>> for Transform<T>
where
    T: Copy + AddAssign + Add<T, Output = T>,
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

impl<T> Neg for Transform<T>
where
    T: Copy + AddAssign + Mul<i32, Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Transform {
            translation: self.translation * -1,
            rotation: -self.rotation,
            scale: -self.scale,
        }
    }
}

#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn add_assign() {
        let mut f2d = vector2d!(2, 3);
        f2d += vector2d!(4, 5);
        assert_eq!(f2d, vector2d!(6, 8));
    }

    #[test]
    fn mul_assign() {
        let mut f2d = vector2d!(2, 3);
        f2d *= vector2d!(4, 5);
        assert_eq!(f2d, vector2d!(8, 15));
    }

    #[test]
    fn add() {
        assert_eq!(vector2d!(2, 3) + vector2d!(4, 5), vector2d!(6, 8));
    }

    #[test]
    fn neg() {
        assert_eq!(-vector2d!(-2), vector2d!(2));
    }

    #[test]
    fn create_transform() {
        assert_eq!(Transform::default(), transform!(0, 0, vector2d!(1.0, 1.0)));
        assert_eq!(transform!(2), transform!(2, 0, vector2d!(1.0, 1.0)));
        assert_eq!(transform!(2, 4), transform!(2, 4, vector2d!(1.0, 1.0)));
    }

    #[test]
    fn add_transform() {
        let trans = transform!(0, 0, vector2d!(1.0, 1.0));

        assert_eq!(
            trans.add(transform!(1, 2, vector2d!(0.5, 1.5))),
            transform!(1, 2, vector2d!(1.5, 2.5))
        );
    }

    #[test]
    fn neg_transform() {
        assert_eq!(
            -transform!(2, 6, vector2d!(2.0, 3.0)),
            transform!(-2, -6, vector2d!(-2.0, -3.0)),
        );
    }
}
