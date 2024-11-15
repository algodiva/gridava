pub struct XYCoordinate {
	x:u32, y:u32
}

// Coordinate X Y
#[allow(unused)]
pub trait Coordinate { 
	fn get_coordinate(&self) -> XYCoordinate;
}

pub trait Tile {

}