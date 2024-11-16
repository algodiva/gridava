use std::{borrow::BorrowMut, collections::HashMap};

use super::*;

pub enum HexOrientation {
    FlatTop,
    PointyTop,
}

pub struct HexGrid<TileType, Coordinate> {
    pub orientation: HexOrientation,
    pub hex_size: f32,
    pub collection: HashMap<Coordinate, TileType>,
}

impl<TileType, Coordinate> Grid<TileType, Coordinate> for HexGrid<TileType, Coordinate> {
    fn get_collection(&self) -> &HashMap<Coordinate, TileType> {
        &self.collection
    }

    fn get_collection_mut(&mut self) -> &mut HashMap<Coordinate, TileType> {
        self.collection.borrow_mut()
    }
}
