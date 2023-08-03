/// A 2D vector.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vec2 {
    /// X coordinate.
    pub x: i32,
    /// Y coordinate.
    pub y: i32,
}

impl Vec2 {
    /// Build a new 2D vector.
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl From<(i32, i32)> for Vec2 {
    fn from((x, y): (i32, i32)) -> Self {
        Vec2::new(x, y)
    }
}

impl From<Vec2> for (i32, i32) {
    fn from(value: Vec2) -> Self {
        (value.x, value.y)
    }
}
