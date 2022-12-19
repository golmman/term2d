use super::point::Point;

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Circle {
    pub pos: Point,
    pub radius: i32,
}

impl Circle {
    pub fn new(x: i32, y: i32, radius: i32) -> Self {
        Self {
            pos: Point::new(x, y),
            radius,
        }
    }
}
