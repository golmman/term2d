use term2d::point::Point;

pub struct State {
    pub frame: u32,
    pub screen_size: Point,
}

impl State {
    pub fn new() -> Self {
        Self {
            frame: 0,
            screen_size: Point::new(0, 0),
        }
    }

    pub fn resize(&mut self, screen_size: Point) {
        self.screen_size = screen_size;
    }
}
