//! A collection defines the operations required of the library in order to interface
//! with application specific data storage methods.

/// The collection trait that defines behavior needed from a data storage schema.
pub trait Collection<C, T> {
    /// The ability to set a coordinate in the schema, this can be thought of like assignment,
    /// or HashMap insert function.
    fn set(&mut self, coord: C, data: T);
}
