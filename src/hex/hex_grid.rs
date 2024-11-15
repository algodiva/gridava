use super::*;

pub enum HexOrientation {
	FlatTop,
	PointyTop
}

pub struct HexGrid<TileType: Tile, TC>
where 
	TC: TileCollection<TileType>{
	pub orientation: HexOrientation,
	pub hex_size: f32,
	pub collection: TC,
	pub _marker: std::marker::PhantomData<TileType>
}

impl<TileType: Tile, TC: TileCollection<TileType>> Grid<TileType> for HexGrid<TileType, TC> {
	fn get_collection<T: TileCollection<TileType>>(&self) -> Result<T, GridError> {
		Err(GridError::AccessError)
	}
}