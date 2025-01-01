use std::collections::HashMap;

use gridava::hex::{coordinate::Axial, edge::Edge, vertex::Vertex};

#[derive(Clone, Debug)]
pub enum GameError {
    InvalidLocation,
    NotEnoughResources,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Default)]
pub enum TileType {
    #[default]
    Desert,
    Plains,
    Farmland,
    Forest,
    Quarry,
    Mountain,
}

impl From<ResourceType> for TileType {
    fn from(value: ResourceType) -> Self {
        match value {
            ResourceType::Lumber => TileType::Forest,
            ResourceType::Brick => TileType::Quarry,
            ResourceType::Wool => TileType::Plains,
            ResourceType::Grain => TileType::Farmland,
            ResourceType::Ore => TileType::Mountain,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct GameTile {
    pub tile_type: TileType,
    pub number: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DevType {
    None,
    House,
    City,
}

#[derive(Clone, Debug)]
pub struct GameVert {
    pub vert_type: DevType,
    pub owning_player: usize,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum EdgeType {
    None,
    Road,
}

#[derive(Clone, Debug)]
pub struct GameEdge {
    pub edge_type: EdgeType,
    pub owning_player: usize,
}

#[derive(Clone, Debug, Default)]
pub struct GameBoard {
    pub tiles: HashMap<Axial, GameTile>,
    pub edges: HashMap<Edge, GameEdge>,
    pub vertices: HashMap<Vertex, GameVert>,
    pub robber_tile: Axial,
}

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub enum ResourceType {
    Lumber,
    Brick,
    Wool,
    Grain,
    Ore,
}
pub const RESOURCE_TYPE_NUM: usize = ResourceType::Ore as usize + 1;

impl TryFrom<TileType> for ResourceType {
    type Error = String;

    fn try_from(value: TileType) -> Result<Self, Self::Error> {
        match value {
            TileType::Desert => Err("Desert does not have a resource".to_string()),
            TileType::Plains => Ok(ResourceType::Wool),
            TileType::Farmland => Ok(ResourceType::Grain),
            TileType::Forest => Ok(ResourceType::Lumber),
            TileType::Quarry => Ok(ResourceType::Brick),
            TileType::Mountain => Ok(ResourceType::Ore),
        }
    }
}

impl TryFrom<usize> for ResourceType {
    type Error = String;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ResourceType::Lumber),
            1 => Ok(ResourceType::Brick),
            2 => Ok(ResourceType::Wool),
            3 => Ok(ResourceType::Grain),
            4 => Ok(ResourceType::Ore),
            _ => Err("Invalid input".to_string()),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum PurchaseType {
    Road,
    House,
    City,
    DevCard,
}
pub const PURCHASE_TYPE_NUM: usize = PurchaseType::DevCard as usize + 1;

pub const COST_TABLE: [[u32; RESOURCE_TYPE_NUM]; PURCHASE_TYPE_NUM] = [
    [1, 1, 0, 0, 0], // Road
    [1, 1, 1, 1, 0], // House
    [0, 0, 0, 2, 3], // City
    [0, 0, 1, 1, 1], // DevCard
];

#[derive(Clone, Debug, Default)]
pub struct Player {
    pub resources: [u32; RESOURCE_TYPE_NUM],
    pub id: usize,
}

pub const TILE_POOL: [TileType; 19] = [
    TileType::Plains,
    TileType::Plains,
    TileType::Plains,
    TileType::Plains,
    TileType::Forest,
    TileType::Forest,
    TileType::Forest,
    TileType::Forest,
    TileType::Farmland,
    TileType::Farmland,
    TileType::Farmland,
    TileType::Farmland,
    TileType::Quarry,
    TileType::Quarry,
    TileType::Quarry,
    TileType::Mountain,
    TileType::Mountain,
    TileType::Mountain,
    TileType::Desert,
];

pub const NUMBER_POOL: [u32; 18] = [2, 3, 3, 4, 4, 5, 5, 6, 6, 8, 8, 9, 9, 10, 10, 11, 11, 12];
