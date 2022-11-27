use term2d::point::Point;

use crate::world::{PixelType, World};

pub struct State {
    pub cursor: Point,
    pub frame: u32,
    pub world: World,
}

impl State {
    pub fn new(size: &Point) -> Self {
        Self {
            cursor: Point::new(1, 1),
            frame: 0,
            world: World::new(&Point::new(20, 5), size),
        }
    }

    pub fn toggle_dirt(&mut self) {
        let p = Point::new(
            self.cursor.x - self.world.pos.x,
            self.cursor.y - self.world.pos.y,
        );

        if self.world.get_type(&p) == PixelType::Empty {
            self.world.set_pixel(&p, PixelType::Dirt);
        } else {
            self.world.set_pixel(&p, PixelType::Empty);
        }
    }

    pub fn move_cursor_left(&mut self) {
        self.cursor = self.cursor.left();
    }

    pub fn move_cursor_right(&mut self) {
        self.cursor = self.cursor.right();
    }

    pub fn move_cursor_up(&mut self) {
        self.cursor = self.cursor.up();
    }

    pub fn move_cursor_down(&mut self) {
        self.cursor = self.cursor.down();
    }
}
