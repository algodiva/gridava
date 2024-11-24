use std::ops::{Add, AddAssign, Mul, MulAssign, Neg};

// Transform TODO:
//      - Implement methods of combining transforms such as trans1 + trans2
//      - Implement rotation helper functions/rotation struct.

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
pub struct Float2D<T> {
    pub x: T,
    pub y: T,
}

#[macro_export]
macro_rules! float2d {
    ($x:expr) => {
        Float2D { x: $x, y: $x }
    };

    ($x:expr, $y:expr) => {
        Float2D { x: $x, y: $y }
    };
}
pub use float2d;

impl<T> Float2D<T> {}

impl<T: AddAssign> AddAssign for Float2D<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: MulAssign> MulAssign for Float2D<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<T: Add<Output = T> + Copy> Add for Float2D<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        float2d!(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Neg<Output = T>> Neg for Float2D<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        float2d!(-self.x, -self.y)
    }
}

// Transformation matrix data structure.
// Stores information to manipulate a coordinate in space.
#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
pub struct Transform<T: Copy + AddAssign> {
    pub translation: T,
    pub rotation: i32,       // rotation around z-axis; positive CW, negative CCW
    pub scale: Float2D<f32>, // Can this be a coordinate or even a tuple of floats and not a i32?
}

#[macro_export]
macro_rules! transform {
    ($t:expr) => {
        Transform {
            translation: $t,
            rotation: 0,
            scale: float2d!(1.0),
        }
    };

    ($t:expr, $r:expr) => {
        Transform {
            translation: $t,
            rotation: $r,
            scale: float2d!(1.0, 1.0),
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
impl<T: Copy + AddAssign + Default> Default for Transform<T> {
    fn default() -> Self {
        Self {
            translation: Default::default(),
            rotation: Default::default(),
            scale: float2d!(1.0),
        }
    }
}

// Overloading '+' operator to facilitate combining multiple transforms.
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

// Overloading '-' operator to create an inverse transform
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
        let mut f2d = float2d!(2, 3);
        f2d += float2d!(4, 5);
        assert_eq!(f2d, float2d!(6, 8));
    }

    #[test]
    fn mul_assign() {
        let mut f2d = float2d!(2, 3);
        f2d *= float2d!(4, 5);
        assert_eq!(f2d, float2d!(8, 15));
    }

    #[test]
    fn add() {
        assert_eq!(float2d!(2, 3) + float2d!(4, 5), float2d!(6, 8));
    }

    #[test]
    fn neg() {
        assert_eq!(-float2d!(-2), float2d!(2));
    }

    #[test]
    fn create_transform() {
        assert_eq!(Transform::default(), transform!(0, 0, float2d!(1.0, 1.0)));
        assert_eq!(transform!(2), transform!(2, 0, float2d!(1.0, 1.0)));
        assert_eq!(transform!(2, 4), transform!(2, 4, float2d!(1.0, 1.0)));
    }

    #[test]
    fn add_transform() {
        let trans = transform!(0, 0, float2d!(1.0, 1.0));

        assert_eq!(
            trans.add(transform!(1, 2, float2d!(0.5, 1.5))),
            transform!(1, 2, float2d!(1.5, 2.5))
        );
    }

    #[test]
    fn neg_transform() {
        assert_eq!(
            -transform!(2, 6, float2d!(2.0, 3.0)),
            transform!(-2, -6, float2d!(-2.0, -3.0)),
        );
    }
}
