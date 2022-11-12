use crate::point::Point;

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Rect {
    pub pos: Point,
    pub size: Point,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            pos: Point::new(x, y),
            size: Point::new(width, height),
        }
    }
}
