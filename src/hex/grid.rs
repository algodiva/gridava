//! The entity that owns tiles.

use std::collections::HashMap;

use coordinate::Axial;

use super::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Enum denoting orientation of hexagons in a grid.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum HexOrientation {
    FlatTop,
    PointyTop,
}

// Because Rust has determined to hide a constant behind an 'unstable' tag we restate it here.
/// Constant calculation of square root of 3.
#[allow(clippy::excessive_precision)]
pub const SQRT_3: f64 = 1.732050807568877293527446341505872367_f64;

/// A grid of tiles.
///
/// This entity owns the tiles in its coordinate system.
///
/// Contains useful functions to convert to and from world space and grid coordinates.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
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
    /// Convert from worldspace to hex coordinates.
    ///
    /// Takes in a float 64 tuple of the form (x, y) and outputs the coordinates of the nearest tile.
    ///
    /// # Example
    /// ```
    /// let my_object_pos = (100.0, 432.0);
    /// /// ...
    /// use gridava::hex::grid::*;
    ///
    /// let my_grid = HexGrid::<i32>::default();
    /// let nearest_tile = my_grid.world_to_hex(my_object_pos);
    /// ```
    ///
    /// The parent world space can be anything not just a 'game world.' For instance, the screen width and height could be your worldspace.
    /// The grid could even exist in a 3d space and your world's x and y component used.
    pub fn world_to_hex(&self, worldspace: (f64, f64)) -> Axial {
        use crate::axial;

        match self.orientation {
            HexOrientation::PointyTop => {
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
            HexOrientation::FlatTop => {
                let y = worldspace.0 / (SQRT_3 * self.hex_size as f64);
                let x = -worldspace.1 / (SQRT_3 * self.hex_size as f64);
                let t = SQRT_3 * y + 1.0;
                let temp1 = f64::floor(t + x);
                let temp2 = t - x;
                let temp3 = 2.0 * x + 1.0;
                let rf = (temp1 + temp3) / 3.0;
                let qf = (temp1 + temp2) / 3.0;
                axial!(f64::floor(qf) as i32, -f64::floor(rf) as i32)
            }
        }
    }

    /// Convert from hex to worldspace coordinates.
    ///
    /// Takes in a hex coordinate and outputs the worldspace coordinates of the tile's center.
    ///
    /// # Example
    /// ```
    /// /// ...
    /// use gridava::hex::grid::*;
    /// use gridava::hex::coordinate::*;
    ///
    /// let my_grid = HexGrid::<i32>::default();
    /// let world_pos = my_grid.hex_to_world(axial!(12, 33));
    /// ```
    ///
    /// The parent world space can be anything not just a 'game world.' For instance, the screen width and height could be your worldspace.
    /// The grid could even exist in a 3d space and your world's x and y component used.
    pub fn hex_to_world(&self, coord: Axial) -> (f64, f64) {
        match self.orientation {
            HexOrientation::PointyTop => {
                let x = self.hex_size as f64
                    * (SQRT_3 * coord.q as f64 + SQRT_3 / 2.0 * coord.r as f64);
                let y = self.hex_size as f64 * (3.0 / 2.0 * coord.r as f64);
                (x, y)
            }
            HexOrientation::FlatTop => {
                let x = self.hex_size as f64 * (3.0 / 2.0 * coord.q as f64);
                let y = self.hex_size as f64
                    * (SQRT_3 / 2.0 * coord.q as f64 + SQRT_3 * coord.r as f64);
                (x, y)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::axial;
    use assert_float_eq::*;

    #[test]
    fn default() {
        let def = HexGrid::<i32>::default();

        assert_eq!(def.orientation, HexOrientation::PointyTop);
        assert_eq!(def.hex_size, 32.0);
        assert!(def.collection.is_empty());
    }

    #[test]
    fn world_to_hex() {
        let pt10p = HexGrid::<i32> {
            orientation: HexOrientation::PointyTop,
            hex_size: 10.0,
            collection: Default::default(),
        };
        let pt32p = HexGrid::<i32> {
            orientation: HexOrientation::PointyTop,
            hex_size: 32.0,
            collection: Default::default(),
        };
        let pt10f = HexGrid::<i32> {
            orientation: HexOrientation::FlatTop,
            hex_size: 10.0,
            collection: Default::default(),
        };
        let pt32f = HexGrid::<i32> {
            orientation: HexOrientation::FlatTop,
            hex_size: 32.0,
            collection: Default::default(),
        };

        assert_eq!(pt10p.world_to_hex((0.0, 0.0)), axial!(0, 0));
        assert_eq!(pt10p.world_to_hex((SQRT_3 * 112.0, 0.0)), axial!(11, 0));
        assert_eq!(pt10p.world_to_hex((SQRT_3 * 56.0, -470.0)), axial!(21, -31));
        assert_eq!(pt10p.world_to_hex((0.0, 640.0)), axial!(-21, 42));
        assert_eq!(pt10p.world_to_hex((SQRT_3 * 144.0, 640.0)), axial!(-7, 43));

        assert_eq!(pt32p.world_to_hex((0.0, 0.0)), axial!(0, 0));
        assert_eq!(pt32p.world_to_hex((SQRT_3 * 112.0, 0.0)), axial!(4, 0));
        assert_eq!(pt32p.world_to_hex((SQRT_3 * 56.0, -470.0)), axial!(7, -10));
        assert_eq!(pt32p.world_to_hex((0.0, 640.0)), axial!(-6, 13));
        assert_eq!(pt32p.world_to_hex((SQRT_3 * 144.0, 640.0)), axial!(-2, 13));

        assert_eq!(pt10f.world_to_hex((0.0, 0.0)), axial!(0, 0));
        assert_eq!(pt10f.world_to_hex((SQRT_3 * 112.0, 0.0)), axial!(13, -7)); // TODO: should this not give (13, -6)?
        assert_eq!(pt10f.world_to_hex((SQRT_3 * 56.0, -470.0)), axial!(6, -30));
        assert_eq!(pt10f.world_to_hex((0.0, 640.0)), axial!(0, 37));
        assert_eq!(pt10f.world_to_hex((SQRT_3 * 144.0, 640.0)), axial!(16, 29));

        assert_eq!(pt32f.world_to_hex((0.0, 0.0)), axial!(0, 0));
        assert_eq!(pt32f.world_to_hex((SQRT_3 * 112.0, 0.0)), axial!(4, -2));
        assert_eq!(pt32f.world_to_hex((SQRT_3 * 56.0, -470.0)), axial!(2, -9));
        assert_eq!(pt32f.world_to_hex((0.0, 640.0)), axial!(0, 12));
        assert_eq!(pt32f.world_to_hex((SQRT_3 * 144.0, 640.0)), axial!(5, 9));
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
        let pt10p = HexGrid::<i32> {
            orientation: HexOrientation::PointyTop,
            hex_size: 10.0,
            collection: Default::default(),
        };
        let pt40p = HexGrid::<i32> {
            orientation: HexOrientation::PointyTop,
            hex_size: 40.0,
            collection: Default::default(),
        };
        let pt10f = HexGrid::<i32> {
            orientation: HexOrientation::FlatTop,
            hex_size: 10.0,
            collection: Default::default(),
        };
        let pt40f = HexGrid::<i32> {
            orientation: HexOrientation::FlatTop,
            hex_size: 40.0,
            collection: Default::default(),
        };

        assert_f64_tuples_near!(pt10p.hex_to_world(axial!(0, -15)), (SQRT_3 * -75.0, -225.0));
        assert_f64_tuples_near!(pt10p.hex_to_world(axial!(0, 0)), (0.0, 0.0));
        assert_f64_tuples_near!(pt10p.hex_to_world(axial!(0, 15)), (SQRT_3 * 75.0, 225.0));
        assert_f64_tuples_near!(pt10p.hex_to_world(axial!(8, -12)), (SQRT_3 * 20.0, -180.0));
        assert_f64_tuples_near!(pt10p.hex_to_world(axial!(8, 12)), (SQRT_3 * 140.0, 180.0));
        assert_f64_tuples_near!(pt10p.hex_to_world(axial!(12, -8)), (SQRT_3 * 80.0, -120.0));
        assert_f64_tuples_near!(pt10p.hex_to_world(axial!(12, 8)), (SQRT_3 * 160.0, 120.0));
        assert_f64_tuples_near!(pt10p.hex_to_world(axial!(15, 0)), (SQRT_3 * 150.0, 0.0));
        assert_f64_tuples_near!(
            pt10p.hex_to_world(axial!(-8, -12)),
            (SQRT_3 * -140.0, -180.0)
        );
        assert_f64_tuples_near!(pt10p.hex_to_world(axial!(-8, 12)), (SQRT_3 * -20.0, 180.0));
        assert_f64_tuples_near!(
            pt10p.hex_to_world(axial!(-12, -8)),
            (SQRT_3 * -160.0, -120.0)
        );
        assert_f64_tuples_near!(pt10p.hex_to_world(axial!(-12, 8)), (SQRT_3 * -80.0, 120.0));
        assert_f64_tuples_near!(pt10p.hex_to_world(axial!(-15, 0)), (SQRT_3 * -150.0, 0.0));

        assert_f64_tuples_near!(pt10f.hex_to_world(axial!(0, -15)), (0.0, SQRT_3 * -150.0));
        assert_f64_tuples_near!(pt10f.hex_to_world(axial!(0, 0)), (0.0, 0.0));
        assert_f64_tuples_near!(pt10f.hex_to_world(axial!(0, 15)), (0.0, SQRT_3 * 150.0));
        assert_f64_tuples_near!(pt10f.hex_to_world(axial!(8, -12)), (120.0, SQRT_3 * -80.0));
        assert_f64_tuples_near!(pt10f.hex_to_world(axial!(8, 12)), (120.0, SQRT_3 * 160.0));
        assert_f64_tuples_near!(pt10f.hex_to_world(axial!(12, -8)), (180.0, SQRT_3 * -20.0));
        assert_f64_tuples_near!(pt10f.hex_to_world(axial!(12, 8)), (180.0, SQRT_3 * 140.0));
        assert_f64_tuples_near!(pt10f.hex_to_world(axial!(15, 0)), (225.0, SQRT_3 * 75.0));
        assert_f64_tuples_near!(
            pt10f.hex_to_world(axial!(-8, -12)),
            (-120.0, SQRT_3 * -160.0)
        );
        assert_f64_tuples_near!(pt10f.hex_to_world(axial!(-8, 12)), (-120.0, SQRT_3 * 80.0));
        assert_f64_tuples_near!(pt10f.hex_to_world(axial!(-10, 10)), (-150.0, SQRT_3 * 50.0));
        assert_f64_tuples_near!(
            pt10f.hex_to_world(axial!(-12, -8)),
            (-180.0, SQRT_3 * -140.0)
        );
        assert_f64_tuples_near!(pt10f.hex_to_world(axial!(-12, 8)), (-180.0, SQRT_3 * 20.0));
        assert_f64_tuples_near!(pt10f.hex_to_world(axial!(-15, 0)), (-225.0, SQRT_3 * -75.0));

        assert_f64_tuples_near!(pt40p.hex_to_world(axial!(0, 0)), (0.0, 0.0));
        assert_f64_tuples_near!(
            pt40p.hex_to_world(axial!(0, -15)),
            (SQRT_3 * -300.0, -900.0)
        );
        assert_f64_tuples_near!(pt40p.hex_to_world(axial!(15, 0)), (SQRT_3 * 600.0, 0.0));
        assert_f64_tuples_near!(pt40p.hex_to_world(axial!(-12, 8)), (SQRT_3 * -320.0, 480.0));

        assert_f64_tuples_near!(pt40f.hex_to_world(axial!(0, 0)), (0.0, 0.0));
        assert_f64_tuples_near!(pt40f.hex_to_world(axial!(0, -15)), (0.0, SQRT_3 * -600.0));
        assert_f64_tuples_near!(pt40f.hex_to_world(axial!(15, 0)), (900.0, SQRT_3 * 300.0));
        assert_f64_tuples_near!(pt40f.hex_to_world(axial!(-12, 8)), (-720.0, SQRT_3 * 80.0));
    }

    macro_rules! two_way_conversion {
        ($grid:expr, $tup:expr) => {
            let (grid, tup) = ($grid, $tup);

            assert_eq!(grid.world_to_hex(grid.hex_to_world(tup)), tup);
        };
    }

    #[test]
    fn two_way_identity() {
        let pt10p = HexGrid::<i32> {
            orientation: HexOrientation::PointyTop,
            hex_size: 10.0,
            collection: Default::default(),
        };

        two_way_conversion!(pt10p.clone(), axial!(0, 0));
        two_way_conversion!(pt10p.clone(), axial!(12, -8));
        two_way_conversion!(pt10p.clone(), axial!(15, 0));
        two_way_conversion!(pt10p.clone(), axial!(0, -15));

        let ft10p = HexGrid::<i32> {
            orientation: HexOrientation::FlatTop,
            hex_size: 10.0,
            collection: Default::default(),
        };

        two_way_conversion!(ft10p.clone(), axial!(0, 0));
        two_way_conversion!(ft10p.clone(), axial!(12, -8));
        two_way_conversion!(ft10p.clone(), axial!(15, 0));
        two_way_conversion!(ft10p.clone(), axial!(0, -15));
    }
}
