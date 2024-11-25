use std::rc::Rc;

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
