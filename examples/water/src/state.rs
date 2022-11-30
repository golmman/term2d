use term2d::point::Point;

use crate::world::{PixelType, World, Droplet};

pub struct State {
    pub cursor: Point,
    pub frame: u32,
    pub world: World,
}

impl State {
    pub fn new() -> Self {
        Self {
            cursor: Point::new(50, 2),
            frame: 0,
            world: World::new(&Point::new(40, 4), &Point::new(50, 25)),
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

    pub fn add_droplet(&mut self) {
        let p = Point::new(
            self.cursor.x - self.world.pos.x,
            self.cursor.y - self.world.pos.y,
        );

        self.world.water.push(Droplet::new(&p));
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
