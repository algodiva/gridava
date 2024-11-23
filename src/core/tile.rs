use std::rc::Rc;

#[derive(Clone, PartialEq, Debug)]
pub struct Tile<T> {
    pub data: Option<Rc<T>>,
}

impl<T: Default> Tile<T> {
    pub fn new(data: Option<T>) -> Tile<T> {
        Tile {
            data: data.map(|value| Rc::new(value)),
        }
    }
}

impl<T> Default for Tile<T> {
    fn default() -> Self {
        Self { data: None }
    }
}
