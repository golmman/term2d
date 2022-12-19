use super::point::Point;

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

    pub fn contains(&self, p: &Point) -> bool {
        if p.x < self.pos.x || p.x >= self.pos.x + self.size.width() {
            return false;
        }

        if p.y < self.pos.y || p.y >= self.pos.y + self.size.height() {
            return false;
        }

        true
    }
}

impl From<Point> for Rect {
    fn from(p: Point) -> Self {
        Self {
            pos: Point::new(0, 0),
            size: p,
        }
    }
}

impl From<&Point> for Rect {
    fn from(p: &Point) -> Self {
        Self {
            pos: Point::new(0, 0),
            size: p.clone(),
        }
    }
}
