//! Tiles are represented by this struct in the library.

use crate::lib::*;

/// The tile used by this library.
///
/// There is a supplied generic hook that uses a smart pointer so you can store custom data into a tile.
///
/// # Example
/// ```
/// #[derive(Clone, Default)]
/// pub struct MyData {
///     pub custom_field: i32,
/// }
///
/// use gridava::core::tile::Tile;
///
/// let my_tile = Tile::<MyData>::default();
/// /// or
/// let my_tile = Tile::new(Some(MyData { custom_field: 1 }));
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, PartialEq, Debug)]
pub struct Tile<T: Clone> {
    /// Data stored with a tile
    pub data: T,
}

impl<T: Default + Clone> Tile<T> {
    /// Constructs a new tile with an optional provided data.
    pub fn new(data: Option<T>) -> Tile<T> {
        Tile {
            data: data.unwrap_or_default(),
        }
    }
}

impl<T: Default + Clone> Default for Tile<T> {
    fn default() -> Self {
        Self { data: T::default() }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::tile::Tile;

    #[test]
    fn new() {
        assert_eq!(Tile { data: 1 }, Tile::new(Some(1)));
        assert_ne!(Tile { data: 1 }, Tile::new(None));
    }

    #[test]
    fn default() {
        assert_eq!(Tile::<i32>::default(), Tile::new(Some(0)));
    }
}
