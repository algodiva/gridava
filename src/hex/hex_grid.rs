use std::collections::HashMap;

use coordinate::Axial;

use super::*;

pub enum HexOrientation {
    FlatTop,
    PointyTop,
}

// Because Rust has determined to hide a constant behind an 'unstable' tag we restate it here.
#[allow(clippy::excessive_precision)]
pub const SQRT_3: f64 = 1.732050807568877293527446341505872367_f64;

pub struct HexGrid<TileType> {
    pub orientation: HexOrientation,
    pub hex_size: f32,
    pub collection: HashMap<Axial, TileType>,
}

impl<TileType> Default for HexGrid<TileType> {
    fn default() -> Self {
        Self {
            orientation: HexOrientation::PointyTop,
            hex_size: 32.0,
            collection: Default::default(),
        }
    }
}

impl<TileType> HexGrid<TileType> {
    // uses point-top. Need to get conversion for flat top
    pub fn world_to_hex(&self, worldspace: (f64, f64)) -> Axial {
        use crate::axial;
        let x = worldspace.0 / (SQRT_3 * self.hex_size as f64);
        let y = -worldspace.1 / (SQRT_3 * self.hex_size as f64);
        let t = SQRT_3 * y + 1.0;
        let temp1 = f64::floor(t + x);
        let temp2 = t - x;
        let temp3 = 2.0 * x + 1.0;
        let qf = (temp1 + temp3) / 3.0;
        let rf = (temp1 + temp2) / 3.0;
        axial!(f64::floor(qf) as i32, -f64::floor(rf) as i32)
    }

    // uses pointy-top. Need to get conversion for flat top
    pub fn hex_to_world(&self, coord: Axial) -> (f64, f64) {
        let x = self.hex_size as f64 * (SQRT_3 * coord.q as f64 + SQRT_3 / 2.0 * coord.r as f64);
        let y = self.hex_size as f64 * (3.0 / 2.0 * coord.r as f64);
        (x, y)
    }
}
