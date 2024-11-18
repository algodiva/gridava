use crate::axial;

use super::coordinate::{Axial, HexDirection};

impl Axial {
    pub fn neighbor(&self, direction: HexDirection) -> Self {
        match direction {
            HexDirection::Front => axial!(self.q + 1, self.r),
            HexDirection::FrontRight => axial!(self.q, self.r + 1),
            HexDirection::BackRight => axial!(self.q - 1, self.r + 1),
            HexDirection::Back => axial!(self.q - 1, self.r),
            HexDirection::BackLeft => axial!(self.q, self.r - 1),
            HexDirection::FrontLeft => axial!(self.q + 1, self.r - 1),
        }
    }

    pub fn distance(&self, b: Self) -> i32 {
        let vec = *self - b;
        (i32::abs(vec.q) + i32::abs(vec.q + vec.r) + i32::abs(vec.r)) / 2
    }

    // utilize f64 to preserve lossless conversion for i32
    fn lerp_internal(a: i32, b: i32, t: f64) -> f64 {
        a as f64 + (b - a) as f64 * t
    }

    // Calculate nearest hex along time t
    pub fn lerp(&self, b: Self, t: f64) -> Self {
        axial!(
            Self::lerp_internal(self.q, b.q, t).round() as i32,
            Self::lerp_internal(self.r, b.r, t).round() as i32
        )
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
}
