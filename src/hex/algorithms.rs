use std::ops::Rem;

use crate::axial;

use super::coordinate::{Axes, Axial, HexDirection};

impl Axial {
    pub fn make_vector(&self, distance: i32, rot_dir: i32) -> Self {
        *self + HexDirection::from(rot_dir).to_movement_vector() * distance
    }

    pub fn neighbor(&self, direction: HexDirection) -> Self {
        self.make_vector(1, direction.into())
    }

    pub fn distance(&self, b: Self) -> i32 {
        let vec = *self - b;
        (i32::abs(vec.q) + i32::abs(vec.q + vec.r) + i32::abs(vec.r)) / 2
    }

    // utilize f64 to preserve lossless conversion for i32
    fn lerp_internal(a: i32, b: i32, t: f64) -> f64 {
        a as f64 + (b - a) as f64 * t
    }

    // This algorithm is based on the round function by Jacob Rus
    // https://observablehq.com/@jrus/hexround
    pub fn round(fcoord: (f64, f64)) -> Self {
        let qgrid = fcoord.0.round();
        let rgrid = fcoord.1.round();

        let q_rem = fcoord.0 - qgrid;
        let r_rem = fcoord.1 - rgrid;

        if q_rem.abs() >= r_rem.abs() {
            let q = qgrid + f64::round(q_rem + 0.5 * r_rem);
            axial!(q as i32, rgrid as i32)
        } else {
            let r = rgrid + f64::round(r_rem + 0.5 * q_rem);
            axial!(qgrid as i32, r as i32)
        }
    }

    // Calculate nearest hex along time t
    pub fn lerp(&self, b: Self, t: f64) -> Self {
        let q = Self::lerp_internal(self.q, b.q, t);
        let r = Self::lerp_internal(self.r, b.r, t);
        Self::round((q, r))
    }

    pub fn line(&self, b: Self) -> Vec<Self> {
        let dist = self.distance(b);
        let mut ret = vec![];

        let constant = 1.0 / dist as f64;

        for i in 0..=dist {
            ret.push(self.lerp(b, constant * i as f64));
        }

        ret
    }

    pub fn range(&self, range: i32) -> Vec<Self> {
        let mut ret = vec![];

        for q in -range..=range {
            for r in i32::max(-range, -q - range)..=i32::min(range, -q + range) {
                ret.push(*self + axial!(q, r));
            }
        }

        ret
    }

    // center: Option<Self> denotes a point to reflect about. If provided None, coordinate (0,0) will be used.
    pub fn reflect(&self, center: Option<Self>, axes: Axes) -> Self {
        let center = match center {
            Some(c) => c,
            None => axial!(0, 0),
        };

        let centered_coord = *self - center;

        match axes {
            Axes::Q => axial!(centered_coord.q, centered_coord.compute_s()) + center,
            Axes::R => axial!(centered_coord.compute_s(), centered_coord.r) + center,
            Axes::S => axial!(centered_coord.r, centered_coord.q) + center,
        }
    }

    pub(self) fn rotate_recursive(&self, iter: usize, cw: bool) -> Self {
        if iter == 0 {
            *self
        } else {
            let input = match cw {
                true => -self.swizzle_l(),
                false => -self.swizzle_r(),
            };
            input.rotate_recursive(iter - 1, cw)
        }
    }

    // Positive dir means CW, negative CCW, and magnitude denotes how many 60 degree rotations in that direction.
    pub fn rotate(&self, center: Option<Self>, rot_dir: i32) -> Self {
        let center = match center {
            Some(c) => c,
            None => axial!(0, 0),
        };

        let centered_coord = *self - center;

        if rot_dir < 0 {
            // negative, CCW
            centered_coord.rotate_recursive(rot_dir.rem(6).unsigned_abs() as usize, false) + center
        } else {
            // positive, CW
            centered_coord.rotate_recursive(rot_dir.rem(6).unsigned_abs() as usize, true) + center
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_vector() {
        assert_eq!(axial!(6, 4).make_vector(4, 0), axial!(10, 4));
        assert_eq!(axial!(6, 4).make_vector(4, 1), axial!(6, 8));
        assert_eq!(axial!(6, 4).make_vector(4, 2), axial!(2, 8));
        assert_eq!(axial!(6, 4).make_vector(4, 3), axial!(2, 4));
        assert_eq!(axial!(6, 4).make_vector(4, 4), axial!(6, 0));
        assert_eq!(axial!(6, 4).make_vector(4, 5), axial!(10, 0));
    }

    #[test]
    fn neighbor() {
        assert_eq!(axial!(6, 4).neighbor(HexDirection::Back), axial!(5, 4));
        assert_eq!(axial!(6, 4).neighbor(HexDirection::BackLeft), axial!(6, 3));
        assert_eq!(axial!(6, 4).neighbor(HexDirection::BackRight), axial!(5, 5));
        assert_eq!(axial!(6, 4).neighbor(HexDirection::Front), axial!(7, 4));
        assert_eq!(axial!(6, 4).neighbor(HexDirection::FrontLeft), axial!(7, 3));
        assert_eq!(
            axial!(6, 4).neighbor(HexDirection::FrontRight),
            axial!(6, 5)
        );
    }

    #[test]
    fn distance() {
        assert_eq!(axial!(-1, -1).distance(axial!(-1, -1)), 0);
        assert_eq!(axial!(-1, -1).distance(axial!(1, -1)), 2);
        assert_eq!(axial!(-1, -1).distance(axial!(-1, 1)), 2);
        assert_eq!(axial!(-1, -1).distance(axial!(2, 1)), 5);
    }

    #[test]
    fn round() {
        assert_eq!(Axial::round((2.5, 1.5)), axial!(2, 2));
        assert_eq!(Axial::round((2.5, -1.5)), axial!(3, -2));
        assert_eq!(Axial::round((-2.5, -1.5)), axial!(-2, -2));
        assert_eq!(Axial::round((-2.5, 1.5)), axial!(-3, 2));
    }

    #[test]
    fn lerp() {
        assert_eq!(axial!(-1, -1).lerp(axial!(9, 19), -0.25), axial!(-3, -6));
        assert_eq!(axial!(-1, -1).lerp(axial!(9, 19), 0.0), axial!(-1, -1));
        assert_eq!(axial!(-1, -1).lerp(axial!(9, 19), 0.25), axial!(1, 4));
        assert_eq!(axial!(-1, -1).lerp(axial!(9, 19), 0.5), axial!(4, 9));
        assert_eq!(axial!(-1, -1).lerp(axial!(9, 19), 0.75), axial!(6, 14));
        assert_eq!(axial!(-1, -1).lerp(axial!(9, 19), 1.0), axial!(9, 19));
        assert_eq!(axial!(-1, -1).lerp(axial!(9, 19), 1.25), axial!(11, 24));
    }

    #[test]
    fn line() {
        assert_eq!(
            axial!(-1, -1).line(axial!(-1, 1)),
            vec![axial!(-1, -1), axial!(-1, 0), axial!(-1, 1)]
        );
        assert_eq!(
            axial!(-1, -1).line(axial!(1, -1)),
            vec![axial!(-1, -1), axial!(0, -1), axial!(1, -1)]
        );
        assert_eq!(
            axial!(-1, -1).line(axial!(0, 1)),
            vec![axial!(-1, -1), axial!(-1, 0), axial!(0, 0), axial!(0, 1)]
        );
        assert_eq!(
            axial!(-1, -1).line(axial!(1, 0)),
            vec![axial!(-1, -1), axial!(0, -1), axial!(0, 0), axial!(1, 0)]
        );
        assert_eq!(
            axial!(-1, -1).line(axial!(1, 1)),
            vec![
                axial!(-1, -1),
                axial!(0, -1),
                axial!(0, 0),
                axial!(0, 1),
                axial!(1, 1)
            ]
        );
        assert_eq!(
            axial!(-1, 1).line(axial!(1, -1)),
            vec![axial!(-1, 1), axial!(0, 0), axial!(1, -1)]
        );
        assert_eq!(
            axial!(1, 3).line(axial!(3, 1)),
            vec![axial!(1, 3), axial!(2, 2), axial!(3, 1)]
        );
        assert_eq!(
            axial!(0, 0).line(axial!(1, 1)),
            vec![axial!(0, 0), axial!(0, 1), axial!(1, 1)]
        );
    }

    #[test]
    fn range() {
        assert_eq!(axial!(0, 0).range(0), vec![axial!(0, 0)]);
        assert_eq!(
            axial!(0, 0).range(1),
            vec![
                axial!(-1, 0),
                axial!(-1, 1),
                axial!(0, -1),
                axial!(0, 0),
                axial!(0, 1),
                axial!(1, -1),
                axial!(1, 0)
            ]
        );
        assert_eq!(
            axial!(0, 0).range(3),
            vec![
                axial!(-3, 0),
                axial!(-3, 1),
                axial!(-3, 2),
                axial!(-3, 3),
                axial!(-2, -1),
                axial!(-2, 0),
                axial!(-2, 1),
                axial!(-2, 2),
                axial!(-2, 3),
                axial!(-1, -2),
                axial!(-1, -1),
                axial!(-1, 0),
                axial!(-1, 1),
                axial!(-1, 2),
                axial!(-1, 3),
                axial!(0, -3),
                axial!(0, -2),
                axial!(0, -1),
                axial!(0, 0),
                axial!(0, 1),
                axial!(0, 2),
                axial!(0, 3),
                axial!(1, -3),
                axial!(1, -2),
                axial!(1, -1),
                axial!(1, 0),
                axial!(1, 1),
                axial!(1, 2),
                axial!(2, -3),
                axial!(2, -2),
                axial!(2, -1),
                axial!(2, 0),
                axial!(2, 1),
                axial!(3, -3),
                axial!(3, -2),
                axial!(3, -1),
                axial!(3, 0),
            ]
        );
    }

    #[test]
    fn reflect() {
        assert_eq!(axial!(-1, 1).reflect(None, Axes::Q), axial!(-1, 0));
        assert_eq!(
            axial!(1, 3).reflect(Some(axial!(1, 2)), Axes::Q),
            axial!(1, 1)
        );

        assert_eq!(axial!(-1, 1).reflect(None, Axes::R), axial!(0, 1));
        assert_eq!(
            axial!(1, 3).reflect(Some(axial!(1, 2)), Axes::R),
            axial!(0, 3)
        );

        assert_eq!(axial!(-1, 1).reflect(None, Axes::S), axial!(1, -1));
        assert_eq!(
            axial!(1, 3).reflect(Some(axial!(1, 2)), Axes::S),
            axial!(2, 2)
        );
    }

    #[test]
    fn rotate() {
        // CW
        assert_eq!(axial!(-1, 1).rotate(None, 1), axial!(-1, 0));
        assert_eq!(axial!(-1, 1).rotate(None, 2), axial!(0, -1));
        assert_eq!(axial!(-1, 1).rotate(None, 3), axial!(1, -1));
        assert_eq!(axial!(-1, 1).rotate(None, 7), axial!(-1, 0));
        assert_eq!(axial!(-1, 1).rotate(None, 8), axial!(0, -1));
        assert_eq!(axial!(-1, 1).rotate(None, 9), axial!(1, -1));

        // CCW
        assert_eq!(axial!(-1, 1).rotate(None, -1), axial!(0, 1));
        assert_eq!(axial!(-1, 1).rotate(None, -2), axial!(1, 0));
        assert_eq!(axial!(-1, 1).rotate(None, -3), axial!(1, -1));

        // About non (0, 0) center
        assert_eq!(axial!(0, 0).rotate(Some(axial!(1, 1)), 1), axial!(2, -1));
        assert_eq!(axial!(0, 0).rotate(Some(axial!(1, 1)), 2), axial!(3, 0));
        assert_eq!(axial!(0, 0).rotate(Some(axial!(1, 1)), 3), axial!(2, 2));
    }
}
