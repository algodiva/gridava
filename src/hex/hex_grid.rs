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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::axial;
    use assert_float_eq::*;

    #[test]
    fn world_to_hex() {
        let pt10 = HexGrid::<i32> {
            orientation: HexOrientation::PointyTop,
            hex_size: 10.0,
            collection: Default::default(),
        };
        let pt32 = HexGrid::<i32> {
            orientation: HexOrientation::PointyTop,
            hex_size: 32.0,
            collection: Default::default(),
        };

        assert_eq!(pt10.world_to_hex((0.0, 0.0)), axial!(0, 0));
        assert_eq!(pt10.world_to_hex((SQRT_3 * 112.0, 0.0)), axial!(11, 0));
        assert_eq!(pt10.world_to_hex((SQRT_3 * 56.0, 480.0)), axial!(-10, 32));
        assert_eq!(pt10.world_to_hex((0.0, 640.0)), axial!(-21, 42));
        assert_eq!(pt10.world_to_hex((SQRT_3 * 144.0, 640.0)), axial!(-7, 43));

        assert_eq!(pt32.world_to_hex((0.0, 0.0)), axial!(0, 0));
        assert_eq!(pt32.world_to_hex((SQRT_3 * 112.0, 0.0)), axial!(4, 0));
        assert_eq!(pt32.world_to_hex((SQRT_3 * 56.0, 480.0)), axial!(-3, 10));
        assert_eq!(pt32.world_to_hex((0.0, 640.0)), axial!(-6, 13));
        assert_eq!(pt32.world_to_hex((SQRT_3 * 144.0, 640.0)), axial!(-2, 13));
    }

    macro_rules! assert_f64_tuples_near {
        ($tup:expr, $cmp:expr) => {
            let (tup, cmp) = ($tup, $cmp);
            assert_f64_near!(tup.0, cmp.0, 4);
            assert_f64_near!(tup.1, cmp.1, 4);
        };
    }

    #[test]
    fn hex_to_world() {
        let pt10 = HexGrid::<i32> {
            orientation: HexOrientation::PointyTop,
            hex_size: 10.0,
            collection: Default::default(),
        };
        let pt40 = HexGrid::<i32> {
            orientation: HexOrientation::PointyTop,
            hex_size: 40.0,
            collection: Default::default(),
        };

        assert_f64_tuples_near!(pt10.hex_to_world(axial!(0, 0)), (0.0, 0.0));
        assert_f64_tuples_near!(pt10.hex_to_world(axial!(15, 0)), (SQRT_3 * 150.0, 0.0));
        assert_f64_tuples_near!(pt10.hex_to_world(axial!(10, 10)), (SQRT_3 * 150.0, 150.0));
        assert_f64_tuples_near!(pt10.hex_to_world(axial!(0, 15)), (SQRT_3 * 75.0, 225.0));
        assert_f64_tuples_near!(pt10.hex_to_world(axial!(-10, 10)), (SQRT_3 * -50.0, 150.0));
        assert_f64_tuples_near!(pt10.hex_to_world(axial!(-15, 0)), (SQRT_3 * -150.0, 0.0));
        assert_f64_tuples_near!(
            pt10.hex_to_world(axial!(-10, -10)),
            (SQRT_3 * -150.0, -150.0)
        );
        assert_f64_tuples_near!(pt10.hex_to_world(axial!(0, -15)), (SQRT_3 * -75.0, -225.0));

        assert_f64_tuples_near!(pt40.hex_to_world(axial!(0, 0)), (0.0, 0.0));
        assert_f64_tuples_near!(pt40.hex_to_world(axial!(15, 0)), (SQRT_3 * 600.0, 0.0));
        assert_f64_tuples_near!(pt40.hex_to_world(axial!(10, 10)), (SQRT_3 * 600.0, 600.0));
        assert_f64_tuples_near!(pt40.hex_to_world(axial!(0, 15)), (SQRT_3 * 300.0, 900.0));
        assert_f64_tuples_near!(pt40.hex_to_world(axial!(-10, 10)), (SQRT_3 * -200.0, 600.0));
        assert_f64_tuples_near!(pt40.hex_to_world(axial!(-15, 0)), (SQRT_3 * -600.0, 0.0));
        assert_f64_tuples_near!(
            pt40.hex_to_world(axial!(-10, -10)),
            (SQRT_3 * -600.0, -600.0)
        );
        assert_f64_tuples_near!(pt40.hex_to_world(axial!(0, -15)), (SQRT_3 * -300.0, -900.0));
    }
}
