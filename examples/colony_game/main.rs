pub mod game_types;

use game_types::*;
use gridava::core::collection::Collection;

use gridava::hex::edge::Edge;
use gridava::hex::vertex::Vertex;
use gridava::{axial, hex::coordinate::Axial, hex::shape::HexShape};
use rand::Rng;
use rand::{seq::IteratorRandom, seq::SliceRandom, thread_rng};

use crate::{GameTile, TileType, COST_TABLE, NUMBER_POOL, TILE_POOL};

impl Collection<Axial, GameTile> for GameBoard {
    fn set(&mut self, coord: Axial, data: GameTile) {
        self.tiles.insert(coord, data);
    }
}

/// This example provides real world applications of the library in the context of a colony board game.
/// For the sake of brevity, the application will provide mainly examples of how the game logic will
/// interact with the library. i.e. How to generate an island or calculate longest road.
/// The example in this repository won't have fully functioning graphics or a game loop.
fn main() {
    let mut rng = thread_rng();
    // Declare our application specific data storage, GameBoard struct
    let mut game_board = GameBoard::default();

    // Declare a dummy array of 'players' for our colony game.
    let mut player_array: Vec<Player> = vec![
        Player {
            resources: Default::default(),
            id: 0,
        },
        Player {
            resources: Default::default(),
            id: 1,
        },
    ];

    // Generate an island as a HexShape and apply that HexShape into our long term data storage.
    generate_island().apply_shape(&mut game_board);
    game_board.robber_tile = game_board
        .tiles
        .iter()
        .find(|&(_, tile)| tile.tile_type == TileType::Desert)
        .map(|(coord, _)| *coord)
        .expect("There should always be at minimum 1 desert tile on the board.");

    // Roll 2d6 and sum them together to figure out the next phase of the game.
    let roll = rng.gen_range(2..=12);

    // If a 7 is rolled, activate and move the robber; no resource collection.
    if roll == 7 {
        game_board.robber_tile = *game_board
            .tiles
            .keys()
            .choose(&mut rng)
            .unwrap_or(&axial!(2, 2));
    } else {
        // Otherwise collect resources with the roll result
        collect_resources(roll, &game_board, &mut player_array[0]);
    }
}

pub fn generate_island() -> HexShape<GameTile> {
    // Begin: Setting up random pool iterators
    let mut rng = thread_rng();

    let mut shuffled_tile_pool = TILE_POOL;
    shuffled_tile_pool.shuffle(&mut rng);

    let mut shuffled_number_pool = NUMBER_POOL;
    shuffled_number_pool.shuffle(&mut rng);

    let mut tile_pool_iter = shuffled_tile_pool.into_iter();
    let mut number_pool_iter = shuffled_number_pool.into_iter();
    // End: Setting up random pool iterators

    // Fill in the island. Tile pool is 19 long because we add a desert tile, and number pool is 18 long because we have to ensure
    //  the desert tile has an unreachable number roll by a 2d6 like 0.
    HexShape::make_hexagon(2, 0, true, |_| match tile_pool_iter.next() {
        Some(TileType::Desert) => GameTile {
            tile_type: TileType::Desert,
            number: 0,
        },
        Some(tile_type) => GameTile {
            tile_type,
            number: number_pool_iter
                .next()
                .expect("Number pool ran out before tile pool"),
        },
        None => unreachable!("Tile pool ran out unexpectedly"),
    })
}

/// Collect resources from the board given a roll and award it to the player.
pub fn collect_resources(roll: u32, board: &GameBoard, player: &mut Player) {
    let pid = player.id;
    board
        // First we filter the storage for any tile that contains the rolled number, and does not have a robber on it since that does not give resources.
        .tiles
        .iter()
        .filter(|(coord, tile_data)| tile_data.number == roll && **coord != board.robber_tile)
        .flat_map(|(coord, tile_data)| {
            // Then we filter based on vertices that are owned by the player and have a development on them.
            coord.vertices().into_iter().filter_map(move |vert| {
                board
                    .verts
                    .get(&vert)
                    .filter(|vert_data| {
                        vert_data.owning_player == pid && vert_data.vert_type != DevType::None
                    })
                    // We then combine the tile_data and vert_data into a single iterator for processing
                    .map(|vert_data| (tile_data, vert_data))
            })
        })
        // For each vertice that has a development on a tile that gives resources do this logic.
        .for_each(|(tile_data, vert_data)| {
            // This should always be Ok since it should be impossible to roll a 0 from 2d6, which is the desert tile, and desert tile is what
            // will cause the try_into to fail. But for completeness we'll do proper error checking here.
            if let Ok(resource_type) = TryInto::<ResourceType>::try_into(tile_data.tile_type) {
                // Determine the increment amount based on the development type.
                let increment = match vert_data.vert_type {
                    DevType::None => 0,
                    DevType::House => 1,
                    DevType::City => 2,
                };
                // Award resources to the player
                player.resources[resource_type as usize] += increment;
            }
        });
}

/// Check if we have the resources to purchase an upgrade.
pub fn can_purchase(ty: PurchaseType, player: &Player) -> bool {
    COST_TABLE[ty as usize]
        .iter()
        .enumerate()
        .all(|(i, amount)| player.resources[i] >= *amount)
}

/// Remove resources from our player stockpile
pub fn remove_resources(ty: PurchaseType, player: &mut Player) {
    COST_TABLE[ty as usize]
        .iter()
        .enumerate()
        .for_each(|(i, amount)| {
            player.resources[i] -= amount;
        });
}

/// Purchase an upgrade
///
/// Supplied predicate pred is used to mutate player or board state after purchase is finalized.
pub fn purchase<F>(ty: PurchaseType, player: &mut Player, mut pred: F) -> Result<(), GameError>
where
    F: FnMut(),
{
    if can_purchase(ty, player) {
        remove_resources(ty, player);
        pred();
        Ok(())
    } else {
        Err(GameError::NotEnoughResources)
    }
}

/// Place a house on the board at the vertex
///
/// Checks for if a house can be placed following the rules as follows:
///
/// - A house must be on a vertex that is connected (adjacent) to a owned road.
/// - A house must not have another house in any of the three adjacent vertices.
pub fn place_house(
    board: &mut GameBoard,
    vert: &Vertex,
    player: &mut Player,
) -> Result<(), GameError> {
    let player_id = player.id;

    // Check if we have a house in an adjacent vertex.
    let is_house_adjacent = vert.adjacent_vertices().iter().any(|vert| {
        board
            .verts
            .get(vert)
            .map_or(false, |val| val.vert_type != DevType::None)
    });

    // Check if we have a road on one of our adjacent edges.
    let is_on_road = vert.adjacent_edges().iter().any(|edge| {
        board.edges.get(edge).map_or(false, |val| {
            val.edge_type == EdgeType::Road && val.owning_player == player_id
        })
    });

    // Both values must be false in order to be a valid placement.
    if !is_house_adjacent || !is_on_road {
        return Err(GameError::InvalidLocation);
    }

    // Consume resources and purchase, awarding the player the development
    purchase(PurchaseType::House, player, || {
        board.verts.insert(
            *vert,
            GameVert {
                vert_type: DevType::House,
                owning_player: player_id,
            },
        );
    })
}

/// Place a road on the board at the edge
///
/// Checks for if an edge can be placed following the rules as follows:
///
/// - A road must be on a edge with an adjacent edge being another road owned by the same player
///
/// or
/// - A road must be on an edge with an adjacent vertex being a development such as a house or city.
pub fn place_road(
    board: &mut GameBoard,
    edge: &Edge,
    player: &mut Player,
) -> Result<(), GameError> {
    let player_id = player.id;

    // Is there a road adjacent to this road?
    let is_road_adjacent = edge.adjacent_edges().iter().any(|e| {
        board.edges.get(e).map_or(false, |val| {
            val.edge_type == EdgeType::Road && val.owning_player == player_id
        })
    });

    // Do we have a owned and developed vertex adjacent to us?
    let is_development_adjacent = edge.endpoints().iter().any(|v| {
        board.verts.get(v).map_or(false, |val| {
            val.vert_type != DevType::None && val.owning_player == player_id
        })
    });

    // Either value must be false to be a valid placement
    if !is_road_adjacent && !is_development_adjacent {
        return Err(GameError::InvalidLocation);
    }

    purchase(PurchaseType::Road, player, || {
        board.edges.insert(
            *edge,
            GameEdge {
                edge_type: EdgeType::Road,
                owning_player: player_id,
            },
        );
    })
}
