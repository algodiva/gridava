// SVG file generation for hex grids

use svg::Document;
use svg::node::element::{Path,SVG,Text};
use svg::node::element::path::Data;

use crate::hex::grid::{HexGrid,HexOrientation};
use crate::hex::coordinate::{Axial,axial};

const BASE_SHORT: f64 = 26.0;
const BASE_LONG: f64 = 45.0;
const PAD: f64 = 10.0;

pub fn render_shape<T: Clone>(orientation: HexOrientation) -> SVG {
    /*
    let mut data = render_one_tile::<T>(Data::new(), axial!(0, 0), orientation.clone());

    data = render_one_tile::<T>(data, axial!(1, 0), HexOrientation::PointyTop);
    data = render_one_tile::<T>(data, axial!(0, 1), HexOrientation::PointyTop);
    data = render_one_tile::<T>(data, axial!(1, 2), HexOrientation::PointyTop);
    data = data.close();
    */
    let grid = HexGrid::<(), ()> {
        orientation: orientation.clone(),
        hex_size: (BASE_SHORT * 2.0) as f32,
        ..HexGrid::default()
    };

    let mut doc = Document::new();
    let tiles = [ axial!(0, 0), axial!(1, 0), axial!(0, 1), axial!(1, 3) ];
    let mut max_q = BASE_LONG;
    let mut min_q = -max_q;
    let mut max_r = BASE_SHORT * 2.0;
    let mut min_r = -max_r;

    tiles.map(| coords | {
        let (base_q, base_r) = grid.hex_to_world(coords);
        let data = Data::new();

        // These only apply for PointyTop
        if base_q - BASE_LONG < min_q { min_q = base_q - BASE_LONG; }
        if base_q + BASE_LONG > max_q { max_q = base_q + BASE_LONG; }

        if base_r - BASE_SHORT * 2.0 < min_r { min_r = base_r - BASE_SHORT * 2.0; }
        if base_r + BASE_SHORT * 2.0 > max_r { max_r = base_r + BASE_SHORT * 2.0; }

        let hexagon = match orientation {
            HexOrientation::PointyTop => data
                .move_to((base_q, base_r + BASE_SHORT * 2.0))
                .line_to((base_q + BASE_LONG, base_r + BASE_SHORT))
                .line_to((base_q + BASE_LONG, base_r - BASE_SHORT))
                .line_to((base_q, base_r - BASE_SHORT * 2.0))
                .line_to((base_q - BASE_LONG, base_r - BASE_SHORT))
                .line_to((base_q - BASE_LONG, base_r + BASE_SHORT))
                .line_to((base_q, base_r + BASE_SHORT * 2.0)),
            HexOrientation::FlatTop => data
                .move_to((base_q + BASE_SHORT * 2.0, base_r))
                .line_to((base_q + BASE_SHORT, base_r + BASE_LONG))
                .line_to((base_q - BASE_SHORT, base_r + BASE_LONG))
                .line_to((base_q - BASE_SHORT * 2.0, base_r))
                .line_to((base_q - BASE_SHORT, base_r - BASE_LONG))
                .line_to((base_q + BASE_SHORT, base_r - BASE_LONG))
                .line_to((base_q + BASE_SHORT * 2.0, base_r)),
        };

        let path = Path::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 3)
            .set("d", hexagon);

        let txt = format!("{},{}", coords.q, coords.r);
        let text = Text::new(txt).set("x", base_q).set("y", base_r).set("text-anchor", "middle").set("font-size", 12);

        doc = doc.clone().add(path).add(text);
    });

    println!("q: {}/{}; r: {}/{}", min_q, max_q, min_r, max_r);

    min_q -= PAD;
    max_q += PAD;
    min_r -= PAD;
    max_r += PAD;

    let border = Data::new()
        .move_to((min_q, min_r))
        .line_to((min_q, max_r))
        .line_to((max_q, max_r))
        .line_to((max_q, min_r))
        .line_to((min_q, min_r));

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke_width", 1)
        .set("d", border);

    doc 
        .add(path)
        .set("viewBox", (min_q, min_r, max_q - min_q, max_r - min_r))
        .set("style", "background-color: #DDDDDD; stroke-width: 1px")
}

pub fn save_image(path: &str, document: SVG) -> Result<(), std::io::Error> {
    svg::save(path, &document)
}

#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_render() {
        let ret = save_image("test.svg", render_shape::<i32>(HexOrientation::PointyTop));

        assert!(ret.is_ok());
    }
}