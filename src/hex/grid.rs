//! The entity that owns tiles.

use crate::lib::*;

use super::coordinate::{axial, Axial};

/// Enum denoting orientation of hexagons in a grid.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum HexOrientation {
    /// The top of a hexagon is flat
    FlatTop,
    #[default]
    /// The top of a hexagon is pointy
    PointyTop,
}

/// A helper converter struct that will help facilitate conversion to and from the grid and world space.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Default)]
pub struct WSConverter {
    /// Size of the hexagon
    pub size: f32,
    /// Which orientation is the hexagon in.
    pub orientation: HexOrientation,
}

impl WSConverter {
    /// Convert from world space to hex coordinates.
    ///
    /// Takes in a float 64 tuple of the form (x, y) and outputs the coordinates of the nearest tile.
    ///
    /// # Example
    /// ```
    /// let my_object_pos = (100.0, 432.0);
    /// /// ...
    /// use gridava::hex::grid::{WSConverter, HexOrientation};
    ///
    /// let converter = WSConverter { size: 32.0, orientation: HexOrientation::PointyTop };
    /// let nearest_tile = converter.world_to_hex(my_object_pos);
    /// ```
    ///
    /// The parent world space can be anything not just a 'game world.' For instance, the screen
    /// width and height could be your world space.
    /// The grid could even exist in a 3d space and your world's x and y component used.
    pub fn world_to_hex(&self, ws_coord: (f64, f64)) -> Axial {
        use crate::axial;

        match self.orientation {
            HexOrientation::PointyTop => {
                let x = ws_coord.0 / (SQRT_3 * self.size as f64);
                let y = -ws_coord.1 / (SQRT_3 * self.size as f64);
                let t = SQRT_3 * y + 1.0;
                let temp1 = f64::floor(t + x);
                let temp2 = t - x;
                let temp3 = 2.0 * x + 1.0;
                let qf = (temp1 + temp3) / 3.0;
                let rf = (temp1 + temp2) / 3.0;
                axial!(f64::floor(qf) as i32, -f64::floor(rf) as i32)
            }
            HexOrientation::FlatTop => {
                let y = ws_coord.0 / (SQRT_3 * self.size as f64);
                let x = -ws_coord.1 / (SQRT_3 * self.size as f64);
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

    /// Convert from hex to world space coordinates.
    ///
    /// Takes in a hex coordinate and outputs the world space coordinates of the tile's center.
    ///
    /// # Example
    /// ```
    /// /// ...
    /// use gridava::hex::coordinate::{Axial, axial};
    /// use gridava::hex::grid::{WSConverter, HexOrientation};
    ///
    /// let converter = WSConverter { size: 32.0, orientation: HexOrientation::PointyTop };
    /// let nearest_tile = converter.hex_to_world(axial!(12, 33));
    /// ```
    ///
    /// The parent world space can be anything not just a 'game world.' For instance, the screen
    /// width and height could be your world space.
    /// The grid could even exist in a 3d space and your world's x and y component used.
    pub fn hex_to_world(&self, coord: Axial) -> (f64, f64) {
        match self.orientation {
            HexOrientation::PointyTop => {
                let x =
                    self.size as f64 * (SQRT_3 * coord.q as f64 + SQRT_3 / 2.0 * coord.r as f64);
                let y = self.size as f64 * (3.0 / 2.0 * coord.r as f64);
                (x, y)
            }
            HexOrientation::FlatTop => {
                let x = self.size as f64 * (3.0 / 2.0 * coord.q as f64);
                let y =
                    self.size as f64 * (SQRT_3 / 2.0 * coord.q as f64 + SQRT_3 * coord.r as f64);
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
    fn world_to_hex() {
        // Size 10 PT
        let grid10p = WSConverter {
            size: 10.0,
            orientation: HexOrientation::PointyTop,
        };

        assert_eq!(grid10p.world_to_hex((0.0, 0.0)), axial!(0, 0));
        assert_eq!(grid10p.world_to_hex((SQRT_3 * 112.0, 0.0)), axial!(11, 0));
        assert_eq!(
            grid10p.world_to_hex((SQRT_3 * 56.0, -470.0)),
            axial!(21, -31)
        );
        assert_eq!(grid10p.world_to_hex((0.0, 640.0)), axial!(-21, 42));
        assert_eq!(
            grid10p.world_to_hex((SQRT_3 * 144.0, 640.0)),
            axial!(-7, 43)
        );

        // size 32 PT
        let grid32p = WSConverter {
            size: 32.0,
            ..grid10p
        };

        assert_eq!(grid32p.world_to_hex((0.0, 0.0)), axial!(0, 0));
        assert_eq!(grid32p.world_to_hex((SQRT_3 * 112.0, 0.0)), axial!(4, 0));
        assert_eq!(
            grid32p.world_to_hex((SQRT_3 * 56.0, -470.0)),
            axial!(7, -10)
        );
        assert_eq!(grid32p.world_to_hex((0.0, 640.0)), axial!(-6, 13));
        assert_eq!(
            grid32p.world_to_hex((SQRT_3 * 144.0, 640.0)),
            axial!(-2, 13)
        );

        // size 10 FT
        let grid10f = WSConverter {
            size: 10.0,
            orientation: HexOrientation::FlatTop,
        };

        assert_eq!(grid10f.world_to_hex((0.0, 0.0)), axial!(0, 0));
        assert_eq!(grid10f.world_to_hex((SQRT_3 * 112.0, 0.0)), axial!(13, -7)); // TODO: should this not give (13, -6)?
        assert_eq!(
            grid10f.world_to_hex((SQRT_3 * 56.0, -470.0)),
            axial!(6, -30)
        );
        assert_eq!(grid10f.world_to_hex((0.0, 640.0)), axial!(0, 37));
        assert_eq!(
            grid10f.world_to_hex((SQRT_3 * 144.0, 640.0)),
            axial!(16, 29)
        );

        // size 32 FT
        let grid32f = WSConverter {
            size: 32.0,
            ..grid10f
        };

        assert_eq!(grid32f.world_to_hex((0.0, 0.0)), axial!(0, 0));
        assert_eq!(grid32f.world_to_hex((SQRT_3 * 112.0, 0.0)), axial!(4, -2));
        assert_eq!(grid32f.world_to_hex((SQRT_3 * 56.0, -470.0)), axial!(2, -9));
        assert_eq!(grid32f.world_to_hex((0.0, 640.0)), axial!(0, 12));
        assert_eq!(grid32f.world_to_hex((SQRT_3 * 144.0, 640.0)), axial!(5, 9));
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
        // Size 10 PT
        let grid = WSConverter {
            size: 10.0,
            orientation: HexOrientation::PointyTop,
        };

        assert_f64_tuples_near!(grid.hex_to_world(axial!(0, -15)), (SQRT_3 * -75.0, -225.0));
        assert_f64_tuples_near!(grid.hex_to_world(axial!(0, 0)), (0.0, 0.0));
        assert_f64_tuples_near!(grid.hex_to_world(axial!(0, 15)), (SQRT_3 * 75.0, 225.0));
        assert_f64_tuples_near!(grid.hex_to_world(axial!(8, -12)), (SQRT_3 * 20.0, -180.0));
        assert_f64_tuples_near!(grid.hex_to_world(axial!(8, 12)), (SQRT_3 * 140.0, 180.0));
        assert_f64_tuples_near!(grid.hex_to_world(axial!(12, -8)), (SQRT_3 * 80.0, -120.0));
        assert_f64_tuples_near!(grid.hex_to_world(axial!(12, 8)), (SQRT_3 * 160.0, 120.0));
        assert_f64_tuples_near!(grid.hex_to_world(axial!(15, 0)), (SQRT_3 * 150.0, 0.0));
        assert_f64_tuples_near!(
            grid.hex_to_world(axial!(-8, -12)),
            (SQRT_3 * -140.0, -180.0)
        );
        assert_f64_tuples_near!(grid.hex_to_world(axial!(-8, 12)), (SQRT_3 * -20.0, 180.0));
        assert_f64_tuples_near!(
            grid.hex_to_world(axial!(-12, -8)),
            (SQRT_3 * -160.0, -120.0)
        );
        assert_f64_tuples_near!(grid.hex_to_world(axial!(-12, 8)), (SQRT_3 * -80.0, 120.0));
        assert_f64_tuples_near!(grid.hex_to_world(axial!(-15, 0)), (SQRT_3 * -150.0, 0.0));

        // Size 40 PT
        let grid = WSConverter { size: 40.0, ..grid };

        assert_f64_tuples_near!(grid.hex_to_world(axial!(0, 0)), (0.0, 0.0));
        assert_f64_tuples_near!(grid.hex_to_world(axial!(0, -15)), (SQRT_3 * -300.0, -900.0));
        assert_f64_tuples_near!(grid.hex_to_world(axial!(15, 0)), (SQRT_3 * 600.0, 0.0));
        assert_f64_tuples_near!(grid.hex_to_world(axial!(-12, 8)), (SQRT_3 * -320.0, 480.0));

        // Size 40 FT
        let grid = WSConverter {
            orientation: HexOrientation::FlatTop,
            ..grid
        };

        assert_f64_tuples_near!(grid.hex_to_world(axial!(0, 0)), (0.0, 0.0));
        assert_f64_tuples_near!(grid.hex_to_world(axial!(0, -15)), (0.0, SQRT_3 * -600.0));
        assert_f64_tuples_near!(grid.hex_to_world(axial!(15, 0)), (900.0, SQRT_3 * 300.0));
        assert_f64_tuples_near!(grid.hex_to_world(axial!(-12, 8)), (-720.0, SQRT_3 * 80.0));

        // Size 10 FT
        let grid = WSConverter { size: 10.0, ..grid };

        assert_f64_tuples_near!(grid.hex_to_world(axial!(0, -15)), (0.0, SQRT_3 * -150.0));
        assert_f64_tuples_near!(grid.hex_to_world(axial!(0, 0)), (0.0, 0.0));
        assert_f64_tuples_near!(grid.hex_to_world(axial!(0, 15)), (0.0, SQRT_3 * 150.0));
        assert_f64_tuples_near!(grid.hex_to_world(axial!(8, -12)), (120.0, SQRT_3 * -80.0));
        assert_f64_tuples_near!(grid.hex_to_world(axial!(8, 12)), (120.0, SQRT_3 * 160.0));
        assert_f64_tuples_near!(grid.hex_to_world(axial!(12, -8)), (180.0, SQRT_3 * -20.0));
        assert_f64_tuples_near!(grid.hex_to_world(axial!(12, 8)), (180.0, SQRT_3 * 140.0));
        assert_f64_tuples_near!(grid.hex_to_world(axial!(15, 0)), (225.0, SQRT_3 * 75.0));
        assert_f64_tuples_near!(
            grid.hex_to_world(axial!(-8, -12)),
            (-120.0, SQRT_3 * -160.0)
        );
        assert_f64_tuples_near!(grid.hex_to_world(axial!(-8, 12)), (-120.0, SQRT_3 * 80.0));
        assert_f64_tuples_near!(grid.hex_to_world(axial!(-10, 10)), (-150.0, SQRT_3 * 50.0));
        assert_f64_tuples_near!(
            grid.hex_to_world(axial!(-12, -8)),
            (-180.0, SQRT_3 * -140.0)
        );
        assert_f64_tuples_near!(grid.hex_to_world(axial!(-12, 8)), (-180.0, SQRT_3 * 20.0));
        assert_f64_tuples_near!(grid.hex_to_world(axial!(-15, 0)), (-225.0, SQRT_3 * -75.0));
    }

    macro_rules! two_way_conversion {
        ($grid:expr, $tup:expr) => {
            let (grid, tup) = ($grid, $tup);

            assert_eq!(grid.world_to_hex(grid.hex_to_world(tup)), tup);
        };
    }

    #[test]
    fn two_way_identity() {
        let pt10p = WSConverter {
            size: 10.0,
            orientation: HexOrientation::PointyTop,
        };

        two_way_conversion!(&pt10p, axial!(0, 0));
        two_way_conversion!(&pt10p, axial!(12, -8));
        two_way_conversion!(&pt10p, axial!(15, 0));
        two_way_conversion!(&pt10p, axial!(0, -15));

        let ft10p = WSConverter {
            orientation: HexOrientation::FlatTop,
            ..pt10p
        };

        two_way_conversion!(&ft10p, axial!(0, 0));
        two_way_conversion!(&ft10p, axial!(12, -8));
        two_way_conversion!(&ft10p, axial!(15, 0));
        two_way_conversion!(&ft10p, axial!(0, -15));
    }
}
