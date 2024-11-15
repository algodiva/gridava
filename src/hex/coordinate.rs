pub struct Cube<T: Copy> {
    q: T,
    r: T,
    s: T,
}

pub struct Axial<T: Copy> {
    q: T,
    r: T,
}

pub fn compute_s<T>(from: (T, T)) -> T
where
    T: std::ops::Neg<Output = T> + std::ops::Sub<Output = T>,
{
    -from.0 - from.1
}

impl<T: Copy> std::convert::From<Axial<T>> for Cube<T>
where
    T: std::ops::Neg<Output = T> + std::ops::Sub<Output = T>,
{
    fn from(value: Axial<T>) -> Self {
        Cube {
            q: value.q,
            r: value.r,
            s: compute_s((value.q, value.r)),
        }
    }
}

impl<T: Copy> std::convert::From<Cube<T>> for Axial<T> {
    fn from(value: Cube<T>) -> Self {
        Axial {
            q: value.q,
            r: value.r,
        }
    }
}
