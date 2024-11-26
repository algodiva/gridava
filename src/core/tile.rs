//! Tiles are represented by this struct in the library.

use std::rc::Rc;

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
#[derive(Clone, PartialEq, Debug)]
pub struct Tile<T: Clone> {
    pub data: Option<Rc<T>>,
}

impl<T: Default + Clone> Tile<T> {
    pub fn new(data: Option<T>) -> Tile<T> {
        Tile {
            data: data.map(|value| Rc::new(value)),
        }
    }
}

impl<T: Clone> Default for Tile<T> {
    fn default() -> Self {
        Self { data: None }
    }
}
