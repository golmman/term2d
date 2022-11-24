use term2d::point::Point;

use crate::world::World;

pub struct State {
    pub frame: u32,
    pub world: World,
}

impl State {
    pub fn new(size: &Point) -> Self {
        Self {
            frame: 0,
            world: World::new(size),
        }
    }
}
