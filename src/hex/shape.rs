use super::coordinate::Axial;

// Transform TODO:
//      - Implement methods of combining transforms such as trans1 + trans2
//      - Implement rotation helper functions/rotation struct.

// This might be better in core module
#[derive(Copy, Clone, PartialOrd, PartialEq, Debug, Default)]
pub struct Transform<T: Copy> {
    translation: T,
    rotation: i32, // rotation around z-axis; positive CW, negative CCW
    scale: T,      // Can this be a coordinate or does it have to be a tuple of floats?
}

// A shape consists of coordinate vectors that define which hexes in relation to the origin are a part of the shape
#[derive(Clone, PartialOrd, PartialEq, Debug, Default)]
pub struct Shape<T: Copy> {
    tiles: Vec<T>, // Vector of hex vectors pointing to origin that comprise this shape.
    origin: T,     // The hex coordinate that denotes the origin of the shape.
    transform: Transform<T>, // The transformation matrix to convert from parent grid to local space.
}

// TODO
//      - rotate shape, about point
//      - scale shape? This will be a complicated task as to interpolate inbetween hexes

impl Shape<Axial> {
    pub fn translate(&mut self, coord: Axial) -> &Self {
        self.transform.translation += coord;
        self
    }

    pub fn rotate(&mut self, rot_dir: i32) -> &Self {
        self.transform.rotation += rot_dir;
        self
    }

    pub fn scale(&mut self, scale: Axial) -> &Self {
        self.transform.scale += scale;
        self
    }
}

#[cfg(test)]
mod test {}
