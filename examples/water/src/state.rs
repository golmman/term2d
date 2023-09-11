use term2d::model::point::Point;
use term2d::App;

use crate::world::PixelType;
use crate::world::World;

pub fn init_model(_app: &App) -> State {
    State {
        cursor: Point::new(50, 2),
        frame: 0,
        world: World::new(&Point::new(40, 4), &Point::new(50, 25)),
    }
}

pub struct State {
    pub cursor: Point,
    pub frame: u32,
    pub world: World,
}

impl State {
    pub fn toggle_dirt(&mut self) {
        let p = Point::new(
            self.cursor.x - self.world.pos.x,
            self.cursor.y - self.world.pos.y,
        );

        match self.world.get_type(&p) {
            Some(PixelType::Dirt) => self.world.set_pixel(&p, PixelType::Empty),
            Some(PixelType::Empty) => self.world.set_pixel(&p, PixelType::Dirt),
            Some(_) => {}
            None => {}
        }
    }

    pub fn add_droplet(&mut self) {
        let p = Point::new(
            self.cursor.x - self.world.pos.x,
            self.cursor.y - self.world.pos.y,
        );

        self.world.add_droplet(&p);
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
