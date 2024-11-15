use std::{collections::HashMap, error::Error, iter::Map};

use super::types::{Tile, XYCoordinate};

#[derive(Debug)]
pub enum TileCollectionError {
    AccessError,
    SetError,
}

impl std::fmt::Display for TileCollectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TileCollectionError::AccessError => write!(f, "Could not access the collection"),
            TileCollectionError::SetError => write!(f, "Could not set the tile in the collection"),
            _ => todo!(),
        }
    }
}

impl Error for TileCollectionError {}

// Trait defining what it means to be a collection of tiles
pub trait TileCollection<TileType: Tile> {
    fn get(&self, coord: XYCoordinate) -> Result<TileType, TileCollectionError>;
    fn set(&mut self, tile: TileType, coord: XYCoordinate) -> Result<(), TileCollectionError>;
}

pub struct MapCollection<TileType: Tile> {
    collection: HashMap<XYCoordinate, TileType>,
}

impl<TileType> TileCollection<TileType> for MapCollection<TileType>
where
    TileType: Tile,
{
    fn get(&self, coord: XYCoordinate) -> Result<TileType, TileCollectionError> {
        todo!()
    }

    fn set(&mut self, tile: TileType, coord: XYCoordinate) -> Result<(), TileCollectionError> {
        todo!()
    }
}

impl<TileType: Tile> Default for MapCollection<TileType> {
    fn default() -> Self {
        MapCollection {
            collection: HashMap::new(),
        }
    }
}
